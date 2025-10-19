use crate::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBrace,
    RightSquareBrace,
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

#[derive(Debug, Clone)]
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

    pub fn get_value(&self) -> Option<Value> {
        return match self.get_type() {
            TokenType::True => Some(Value::Bool(true)),
            TokenType::False => Some(Value::Bool(false)),
            TokenType::Null => Some(Value::Null),
            TokenType::IntegerLiteral => match self.source.parse::<i128>() {
                Ok(value) => Some(Value::Integer(value)),
                Err(_) => None,
            },
            TokenType::FloatLiteral => match self.source.parse::<f64>() {
                Ok(value) => Some(Value::Float(value)),
                Err(_) => None,
            },
            TokenType::StringLiteral => Some(Value::String(
                self.source
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    // Handling escape sequences
                    .map(|s| s.replace("\\\"", "\"")) // Use map here
                    .map(|s| s.replace("\\n", "\n")) // Use map here
                    .map(|s| s.replace("\\t", "\t")) // Use map here
                    .unwrap(),
            )),
            TokenType::CharLiteral => Some(Value::Char(self.source.chars().nth(1).unwrap())),
            _ => None,
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self.token_type {
            TokenType::IntegerLiteral => {
                write!(f, "[{}:{}] Float: {}", self.line, self.col, self.source)
            }
            TokenType::FloatLiteral => {
                write!(f, "[{}:{}] Integer: {}", self.line, self.col, self.source)
            }
            TokenType::StringLiteral => {
                write!(f, "[{}:{}] String: {}", self.line, self.col, self.source)
            }
            TokenType::CharLiteral => {
                write!(f, "[{}:{}] Char: {}", self.line, self.col, self.source)
            }
            TokenType::Identifier => write!(f, "[{}:{}] Identifier: {}", self.line, self.col, self.source),
            TokenType::Error => write!(f, "[Error] [{}:{}] {}", self.line, self.col, self.source),
            _ => write!(f, "[{}:{}] {:?}", self.line, self.col, self.token_type),
        };
    }
}
