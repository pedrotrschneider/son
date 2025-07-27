pub mod error;
pub mod lexer;
pub mod parser;
pub mod serialization;
pub mod token;
pub mod util;
pub mod value;

pub use crate::error::{DeserializationError, Error, ParseError};
use crate::parser::SonParser;
pub use crate::serialization::{Deserialize, FromSon, Serialize, SonPrinter, ToSon};
pub use crate::value::Value;
pub use son_macros::{Deserialize, Serialize};
use std::fs::File;

pub fn from_file<T>(file_path: &str) -> Result<T, Error>
where
    T: Deserialize,
{
    let file = File::open(file_path)?;
    let mut parser = SonParser::new(file);
    let son = parser.parse()?;
    return Ok(T::from_son(son)?);
}

pub fn from_str<T>(str: &str) -> Result<T, Error>
where
    T: Deserialize,
{
    let mut parser = SonParser::new(str.as_bytes());
    let son = parser.parse()?;
    return Ok(T::from_son(son)?);
}

pub fn to_son<T>(value: &T) -> Value
where
    T: Serialize,
{
    return T::to_son(value);
}

pub fn to_string<T>(value: &T) -> String
where
    T: Serialize,
{
    return to_son(value).to_string();
}

pub fn to_string_pretty<T>(value: &T, indentation: &str) -> String
where
    T: Serialize,
{
    let son = to_son(value);
    let printer = SonPrinter::new(indentation.to_string());
    return printer.son_to_string(&son);
}
