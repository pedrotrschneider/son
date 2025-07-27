use crate::token::{Token, TokenType};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum DeserializationError {
    UnexpectedType { expected: String, found: String },
    MissingField { field: String },
    InvalidValue { message: String },
    UnknownVariant { variant: String, enum_name: String },
    Custom(String),
}

impl Display for DeserializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            DeserializationError::UnexpectedType { expected, found } => {
                write!(f, "Unexpected type: expected {}, found {}", expected, found)
            }
            DeserializationError::MissingField { field } => {
                write!(f, "Missing field: {}", field)
            }
            DeserializationError::InvalidValue { message } => {
                write!(f, "Invalid value: {}", message)
            }
            DeserializationError::UnknownVariant { variant, enum_name } => {
                write!(f, "Unknown variant '{}' for enum '{}'", variant, enum_name)
            }
            DeserializationError::Custom(message) => {
                write!(f, "{}", message)
            }
        };
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<TokenType>,
        found: Token,
        message: String,
    },
    UnexpectedIdentifier {
        expected: String,
        found: String,
        message: String,
    },
    UnexpectedEOF,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                message,
            } => {
                if message.is_empty() {
                    write!(f, "Unexpected token: expected one of {:?}, got: \n{}.", expected, found)
                } else {
                    write!(
                        f,
                        "Unexpected token: expected one of {:?}, got: \n{}. \n{}",
                        expected, found, message
                    )
                }
            }
            ParseError::UnexpectedIdentifier { .. } => {
                write!(f, "Unexpected identifier")
            }
            ParseError::UnexpectedEOF => write!(f, "Unexpected End of File"),
        };
    }
}
