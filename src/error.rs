use crate::token::{
    Token,
    TokenType,
};
use std::fmt::{
    Display,
    Formatter,
};

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
pub enum ParseStep {
    Start,
    Value,
    Array,
    Object,
}

impl Display for ParseStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            ParseStep::Start => write!(f, ""),
            ParseStep::Value => write!(f, "[Parse Value]"),
            ParseStep::Array => write!(f, "[Parse Array]"),
            ParseStep::Object => write!(f, "[Parse Object]"),
        };
    }
}

#[derive(Debug)]
pub enum ParseError {
    IOError(ParseStep, std::io::Error),
    UnexpectedToken {
        step: ParseStep,
        expected: &'static [TokenType],
        found: Token,
        message: String,
    },
    UnexpectedIdentifier {
        step: ParseStep,
        expected: String,
        found: String,
        message: String,
    },
    UnexpectedEOF(ParseStep),
    ErrorToken(ParseStep, Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            ParseError::IOError(step, e) => write!(f, "{step} {e}"),

            ParseError::UnexpectedToken {
                step,
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "{} Unexpected token: expected one of {:?}, got: \n{}",
                    step, expected, found
                )?;
                if !message.is_empty() {
                    write!(f, ".")
                } else {
                    write!(f, ". \n{message}")
                }
            }
            ParseError::UnexpectedIdentifier { step, .. } => {
                write!(f, "{step} Unexpected identifier")
            }
            ParseError::UnexpectedEOF(step) => write!(f, "{step} Unexpected End of File"),
            ParseError::ErrorToken(step, t) => write!(f, "{step} Error at: {t}"),
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
