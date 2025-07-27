use crate::ParseError::UnexpectedToken;
use crate::error::ParseError;
use crate::lexer::SonLexer;
use crate::token::TokenType;
use crate::value::Value;
use std::collections::HashMap;
use std::io::Read;

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
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftBracket => self.parse_array(),

            // Unexpected tokens
            TokenType::Error => Err(ParseError::ErrorToken(token)),
            TokenType::EOF => Err(ParseError::UnexpectedEOF),
            _ => Err(UnexpectedToken {
                expected: &[TokenType::LeftBrace, TokenType::LeftBracket],
                found: token,
                message: "SON files can only begin with either a { or [".to_string(),
            }),
        };
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        let expected_tokens: &'static [TokenType] = &[
            TokenType::LeftBrace,
            TokenType::LeftBracket,
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
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftBracket => self.parse_array(),
            TokenType::Negative => Ok(self.parse_value()?.negate()),
            TokenType::True
            | TokenType::False
            | TokenType::Null
            | TokenType::IntegerLiteral
            | TokenType::FloatLiteral
            | TokenType::StringLiteral
            | TokenType::CharLiteral => Ok(token.get_value().unwrap()),
            TokenType::Identifier => Ok(Value::Enum(token.get_source())),

            // Unexpected tokens
            TokenType::Error => Err(ParseError::ErrorToken(token)),
            TokenType::EOF => Err(ParseError::UnexpectedEOF),
            _ => Err(UnexpectedToken {
                expected: expected_tokens,
                found: token,
                message: "".to_string(),
            }),
        };
    }

    fn parse_object(&mut self) -> Result<Value, ParseError> {
        let expected_tokens: &'static [TokenType] = &[
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::LeftBracket,
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
                TokenType::LeftBrace => try_insert(&mut field_name, self.parse_object()?, token)?,
                TokenType::LeftBracket => try_insert(&mut field_name, self.parse_array()?, token)?,
                TokenType::RightBrace => break,
                TokenType::Comma => {}

                // Unexpected tokens
                TokenType::Error => return Err(ParseError::ErrorToken(token)),
                TokenType::EOF => return Err(ParseError::UnexpectedEOF),
                _ => {
                    return Err(UnexpectedToken {
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
        let expected_tokens: &'static [TokenType] = &[
            TokenType::LeftBrace,
            TokenType::LeftBracket,
            TokenType::RightBracket,
            TokenType::Colon,
        ];
        let mut value_array: Vec<Value> = Vec::new();
        while let Some(token) = self.lexer.next() {
            match token.get_type() {
                // Expected tokens
                TokenType::Colon => value_array.push(self.parse_value()?),
                TokenType::LeftBrace => value_array.push(self.parse_object()?),
                TokenType::LeftBracket => value_array.push(self.parse_array()?),
                TokenType::RightBracket => break,
                TokenType::Comma => {}

                // Unexpected tokens
                TokenType::Error => return Err(ParseError::ErrorToken(token.clone())),
                TokenType::EOF => return Err(ParseError::UnexpectedEOF),
                _ => {
                    return Err(UnexpectedToken {
                        expected: expected_tokens,
                        found: token,
                        message: String::new(),
                    });
                }
            }
        }

        return Ok(Value::Array(value_array));
    }
}
