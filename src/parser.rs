use crate::{
    ParseError::UnexpectedToken,
    error::{
        ParseError,
        ParseStep,
    },
    lexer::SonLexer,
    token::TokenType,
    value::Value,
};
use std::{
    collections::HashMap,
    io::Read,
};

pub struct SonParser<T>
where
    T: Sized + Read,
{
    lexer: SonLexer<T>,
}

impl<T> SonParser<T>
where
    T: Sized + Read,
{
    pub fn new(data: T) -> SonParser<T> {
        return Self {
            lexer: SonLexer::new(data),
        };
    }

    pub fn parse(&mut self) -> Result<Value, ParseError> {
        let token = self.lexer.next_token();
        return match token.get_type() {
            // Expected tokens
            TokenType::LeftCurlyBrace => self.parse_object(),
            TokenType::LeftSquareBrace => self.parse_array(),

            // Unexpected tokens
            TokenType::Error => Err(ParseError::ErrorToken(ParseStep::Start, token)),
            TokenType::EOF => Err(ParseError::UnexpectedEOF(ParseStep::Start)),
            _ => Err(UnexpectedToken {
                step: ParseStep::Start,
                expected: &[TokenType::LeftCurlyBrace, TokenType::LeftSquareBrace],
                found: token,
                message: "SON files can only begin with either a { or [".to_string(),
            }),
        };
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        let expected_tokens: &'static [TokenType] = &[
            TokenType::LeftCurlyBrace,
            TokenType::LeftSquareBrace,
            TokenType::Negative,
            TokenType::True,
            TokenType::False,
            TokenType::Null,
            TokenType::IntegerLiteral,
            TokenType::FloatLiteral,
            TokenType::StringLiteral,
            TokenType::CharLiteral,
            TokenType::Identifier,
        ];

        let token = self.lexer.next_token();
        return match token.get_type() {
            // Expected tokens
            TokenType::LeftCurlyBrace => self.parse_object(),
            TokenType::LeftSquareBrace => self.parse_array(),
            TokenType::Negative => Ok(self.parse_value()?.negate()),
            TokenType::True
            | TokenType::False
            | TokenType::Null
            | TokenType::IntegerLiteral
            | TokenType::FloatLiteral
            | TokenType::StringLiteral
            | TokenType::CharLiteral => Ok(token.get_value().unwrap()),
            TokenType::Identifier => Ok(Value::Enum(token.get_source())),
            TokenType::Comma => Ok(self.parse_value()?),

            // Unexpected tokens
            TokenType::Error => Err(ParseError::ErrorToken(ParseStep::Value, token)),
            TokenType::EOF => Err(ParseError::UnexpectedEOF(ParseStep::Value)),
            _ => Err(UnexpectedToken {
                step: ParseStep::Value,
                expected: expected_tokens,
                found: token,
                message: "".to_string(),
            }),
        };
    }

    fn parse_object(&mut self) -> Result<Value, ParseError> {
        let expected_tokens: &'static [TokenType] = &[
            TokenType::LeftCurlyBrace,
            TokenType::RightCurlyBrace,
            TokenType::LeftSquareBrace,
            TokenType::Colon,
            TokenType::Identifier,
        ];

        let mut object_map: HashMap<String, Value> = HashMap::new();
        let mut try_insert = |field_name: &mut String, value, token| {
            if !field_name.is_empty() {
                object_map.insert(field_name.clone(), value);
                field_name.clear();
                return Ok(());
            }
            return Err(UnexpectedToken {
                step: ParseStep::Object,
                expected: &[TokenType::Identifier],
                found: token,
                message: "Expected a field name".to_string(),
            });
        };

        let mut field_name = String::new();
        while let Some(token) = self.lexer.next() {
            match token.get_type() {
                // Expected tokens
                TokenType::Identifier => field_name = token.get_source(),
                TokenType::Colon => try_insert(&mut field_name, self.parse_value()?, token)?,
                TokenType::LeftCurlyBrace => {
                    try_insert(&mut field_name, self.parse_object()?, token)?
                }
                TokenType::RightCurlyBrace => break,
                TokenType::LeftSquareBrace => {
                    try_insert(&mut field_name, self.parse_array()?, token)?
                }
                TokenType::Comma => {}

                // Unexpected tokens
                TokenType::Error => return Err(ParseError::ErrorToken(ParseStep::Object, token)),
                TokenType::EOF => return Err(ParseError::UnexpectedEOF(ParseStep::Object)),
                _ => {
                    return Err(UnexpectedToken {
                        step: ParseStep::Object,
                        expected: expected_tokens,
                        found: token,
                        message: String::new(),
                    });
                }
            }
        }
        return Ok(Value::Object(object_map));
    }

    fn parse_array(&mut self) -> Result<Value, ParseError> {
        let expected_tokens: &'static [TokenType] = &[TokenType::RightSquareBrace];

        let mut value_array: Vec<Value> = Vec::new();
        while let Ok(value) = self.parse_value() {
            value_array.push(value);
        }

        let current_token = self.lexer.current().unwrap();
        return match current_token.get_type() {
            TokenType::RightSquareBrace => Ok(Value::Array(value_array)),
            _ => Err(UnexpectedToken {
                step: ParseStep::Array,
                expected: expected_tokens,
                found: current_token,
                message: String::new(),
            }),
        };
    }
}
