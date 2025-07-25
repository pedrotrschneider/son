use crate::serialization::SonValue;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Negative,

    True,
    False,
    Null,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    Identifier,

    Error,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: u32,
    col: u32,
    source: String,
}

impl Token {
    pub fn new(token_type: TokenType, line: u32, col: u32, source: String) -> Token {
        return Token {
            token_type,
            line,
            col,
            source,
        };
    }

    pub fn new_error(line: u32, col: u32, message: String) -> Token {
        return Token::new(TokenType::Error, line, col, message);
    }

    pub fn get_type(&self) -> TokenType {
        return self.token_type.clone();
    }

    pub fn get_source(&self) -> String {
        return self.source.clone();
    }

    pub fn get_value(&self) -> Option<SonValue> {
        return match self.get_type() {
            TokenType::True => Some(SonValue::Bool(true)),
            TokenType::False => Some(SonValue::Bool(false)),
            TokenType::IntegerLiteral => match self.source.parse::<i128>() {
                Ok(value) => Some(SonValue::Integer(value)),
                Err(_) => None,
            },
            TokenType::FloatLiteral => match self.source.parse::<f64>() {
                Ok(value) => Some(SonValue::Float(value)),
                Err(_) => None,
            },
            TokenType::StringLiteral => Some(SonValue::String(
                self.source
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap()
                    .to_owned(),
            )),
            TokenType::CharLiteral => Some(SonValue::Char(self.source.chars().nth(1).unwrap())),
            _ => None,
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self.token_type {
            TokenType::IntegerLiteral => write!(f, "[{}:{}] float: {}", self.line, self.col, self.source),
            TokenType::FloatLiteral => write!(f, "[{}:{}] int: {}", self.line, self.col, self.source),
            TokenType::StringLiteral => write!(f, "[{}:{}] string: {}", self.line, self.col, self.source),
            TokenType::CharLiteral => write!(f, "[{}:{}] char: {}", self.line, self.col, self.source),
            TokenType::Identifier => write!(f, "[{}:{}] Identifier: {}", self.line, self.col, self.source),
            TokenType::Error => write!(f, "[Error] [{}:{}] {}", self.line, self.col, self.source),
            _ => write!(f, "[{}:{}] {:?}", self.line, self.col, self.token_type),
        };
    }
}
