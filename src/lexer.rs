use crate::token::{Token, TokenType};
use crate::util;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

struct Keywords {}
impl Keywords {
    const TRUE: [char; 4] = ['t', 'r', 'u', 'e'];
    const FALSE: [char; 5] = ['f', 'a', 'l', 's', 'e'];
    const NULL: [char; 4] = ['n', 'u', 'l', 'l'];
}

const TOKENIZER_BUFFER_SIZE: usize = 1024;
pub struct SonLexer<T>
where
    T: Sized + Read,
{
    reader: BufReader<T>,
    line: u32,
    col: u32,

    current_chunk: VecDeque<char>,
    leftovers: Vec<u8>,
    current_token_source: Vec<char>,
    current_token_col: u32,

    current_token: Option<Token>,
    previous_token: Option<Token>,
}

impl<T> SonLexer<T>
where
    T: Sized + Read,
{
    pub fn new(data: T) -> SonLexer<T> {
        return Self::from_buf_reader(BufReader::with_capacity(TOKENIZER_BUFFER_SIZE, data));
    }

    pub fn from_buf_reader(buf_reader: BufReader<T>) -> SonLexer<T> {
        return SonLexer {
            reader: buf_reader,
            line: 1,
            col: 0,

            current_chunk: VecDeque::new(),
            leftovers: Vec::new(),
            current_token_source: Vec::new(),
            current_token_col: 0,

            current_token: None,
            previous_token: None,
        };
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.current_token_col = self.col;

        let Some(c) = self.advance() else {
            return self.new_token(TokenType::EOF);
        };

        let token = match c {
            '(' => self.new_token(TokenType::LeftParen),
            ')' => self.new_token(TokenType::RightParen),
            '{' => self.new_token(TokenType::LeftBrace),
            '}' => self.new_token(TokenType::RightBrace),
            '[' => self.new_token(TokenType::LeftBracket),
            ']' => self.new_token(TokenType::RightBracket),
            ',' => self.new_token(TokenType::Comma),
            '.' => self.new_token(TokenType::Dot),
            ':' => self.new_token(TokenType::Colon),
            '-' => self.new_token(TokenType::Negative),
            '"' => self.consume_string_literal(),
            '\'' => self.consume_char_literal(),
            '0'..='9' => self.consume_numeric_literal(),
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier(),
            _ => self.new_error_token("Unexpected character"),
        };

        self.previous_token = self.current_token.clone();
        self.current_token = Some(token.clone());

        return token;
    }
}

impl<T> Iterator for SonLexer<T>
where
    T: Sized + Read,
{
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        return match token.get_type() {
            TokenType::EOF => None,
            _ => Some(token),
        };
    }
}

impl<T> SonLexer<T>
where
    T: Sized + Read,
{
    fn new_token(&mut self, token_type: TokenType) -> Token {
        let token_source = self.current_token_source.iter().collect();
        self.current_token_source.clear();
        return Token::new(token_type, self.line, self.current_token_col, token_source);
    }

    fn new_error_token(&mut self, message: &str) -> Token {
        return Token::new_error(self.line, self.current_token_col, message.to_owned());
    }

    fn next_chunk(&mut self) -> bool {
        let buffer = self.reader.fill_buf().unwrap();
        if buffer.is_empty() {
            return false;
        }

        self.leftovers.extend_from_slice(buffer);
        match std::str::from_utf8(&self.leftovers) {
            Ok(valid_str) => {
                self.current_chunk.append(&mut valid_str.chars().collect());
                self.leftovers.clear();
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();

                if valid_up_to > 0 {
                    self.current_chunk.append(
                        &mut std::str::from_utf8(&self.leftovers[..valid_up_to])
                            .unwrap()
                            .chars()
                            .collect(),
                    );
                }

                self.leftovers = self.leftovers[valid_up_to..].to_vec();
            }
        };

        self.reader.consume(TOKENIZER_BUFFER_SIZE);
        return true;
    }

    // Check current character but don't consume.
    fn peek(&mut self) -> Option<char> {
        if !self.current_chunk.is_empty() || self.next_chunk() {
            return self.current_chunk.front().cloned();
        }

        return None;
    }

    // Check next character but don't consume.
    fn peek_next(&mut self) -> Option<char> {
        if !self.current_chunk.len() >= 2 || self.next_chunk() {
            return Some(self.current_chunk[1]);
        }

        return None;
    }

    // Get the current character and advance.
    fn advance(&mut self) -> Option<char> {
        if !self.current_chunk.is_empty() || self.next_chunk() {
            let c = self.current_chunk.pop_front().unwrap();
            self.current_token_source.push(c.clone());
            self.col += 1;
            return Some(c);
        }

        return None;
    }

    fn advance_line(&mut self) {
        self.line += 1;
        self.col = 0;
        self.advance();
    }

    fn skip_comment(&mut self) {
        while self.peek().is_some_and(|c| c != '\n') {
            self.advance();
        }
        self.advance(); // Skip final new line of the comment
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\r' | '\t' => util::discard(self.advance()),
                '\n' => self.advance_line(),
                '/' if self.peek_next() == Some('/') => self.skip_comment(),
                _ => break,
            }
        }
        self.current_token_source.clear();
    }

    // Check if the current character is some expected character.
    // Advance only if the characters match.
    fn match_token(&mut self, expected: char) -> bool {
        return match self.peek() {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            _ => false,
        };
    }

    fn consume_string_literal(&mut self) -> Token {
        // Advance until the end of the string literal.
        while let Some(c) = self.peek() {
            match c {
                '\n' => self.advance_line(),
                '"' => break,
                _ => util::discard(self.advance()),
            }
        }
        // Validate closing quotes.
        if !self.match_token('"') {
            return self.new_error_token("Unterminated string literal");
        }
        return self.new_token(TokenType::StringLiteral);
    }

    fn consume_char_literal(&mut self) -> Token {
        // Advance until the end of the char literal
        while self.peek().is_some_and(|c| c != '\'') {
            self.advance();
        }
        // Validate closing quote
        if !self.match_token('\'') {
            return self.new_error_token("Unterminated char literal");
        }
        return self.new_token(TokenType::CharLiteral);
    }

    fn consume_numeric_literal(&mut self) -> Token {
        // Advance until the end of the numeric literal.
        while self.peek().is_some_and(|c| c.is_numeric()) {
            self.advance();
        }

        // Look for a fractional part.
        if !self.match_token('.') {
            return self.new_token(TokenType::IntegerLiteral);
        }

        // Advance until the end of the fractional part.
        while self.peek().is_some_and(|c| c.is_numeric()) {
            self.advance();
        }

        return self.new_token(TokenType::FloatLiteral);
    }

    // This method requires all the characters from the identifier to have been consumed already.
    fn get_identifier_type(&mut self) -> TokenType {
        // Check if the identifier matches one of the keywords.
        match self.current_token_source[0] {
            't' if self.current_token_source == Keywords::TRUE => return TokenType::True,
            'f' if self.current_token_source == Keywords::FALSE => return TokenType::False,
            'n' if self.current_token_source == Keywords::NULL => return TokenType::Null,
            _ => (),
        }

        return TokenType::Identifier;
    }

    fn consume_identifier(&mut self) -> Token {
        // Consume all characters available for the identifier.
        while self.peek().is_some_and(|c| c.is_alphabetic() || c == '_') {
            self.advance();
        }

        let identifier_type = self.get_identifier_type();
        return self.new_token(identifier_type);
    }
}
