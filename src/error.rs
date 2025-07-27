use crate::token::{Token, TokenType};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
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
    IOError(std::io::Error),
    UnexpectedToken {
        expected: &'static [TokenType],
        found: Token,
        message: String,
    },
    UnexpectedIdentifier {
        expected: String,
        found: String,
        message: String,
    },
    UnexpectedEOF,
    ErrorToken(Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            ParseError::IOError(e) => write!(f, "{}", e),

            ParseError::UnexpectedToken {
                expected,
                found,
                message,
            } => {
                write!(f, "Unexpected token: expected one of {:?}, got: \n{}", expected, found)?;
                if !message.is_empty() {
                    write!(f, ".")
                } else {
                    write!(f, ". \n{}", message)
                }
            }
            ParseError::UnexpectedIdentifier { .. } => write!(f, "Unexpected identifier"),
            ParseError::UnexpectedEOF => write!(f, "Unexpected End of File"),
            ParseError::ErrorToken(t) => write!(f, "Error at: {}", t),
        };
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    DeserializationError(DeserializationError),
    IOError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Error::ParseError(pe) => write!(f, "Parse Error: {}", pe),
            Error::DeserializationError(de) => write!(f, "Deserialization Error: {}", de),
            Error::IOError(ioe) => write!(f, "IO Error: {}", ioe),
        };
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::IOError(e);
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        return Error::ParseError(e);
    }
}

impl From<DeserializationError> for Error {
    fn from(e: DeserializationError) -> Self {
        return Error::DeserializationError(e);
    }
}
