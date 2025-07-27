use crate::error::ParseError;
use crate::lexer::SonLexer;
use crate::serialization::SonValue;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct SonParser {
    lexer: SonLexer,
    negate_flag: bool,
}

impl SonParser {
    pub fn from_file_to_son_object(file_path: &str) -> Result<SonValue, ParseError> {
        let mut parser = Self {
            lexer: SonLexer::new(file_path),
            negate_flag: false,
        };
        return parser.parse();
    }

    pub fn parse(&mut self) -> Result<SonValue, ParseError> {
        if let Some(token) = self.lexer.next() {
            return match token.get_type() {
                TokenType::LeftBrace => self.parse_object(),
                TokenType::LeftBracket => self.parse_array(),
                _ => Err(ParseError::UnexpectedToken {
                    expected: vec![TokenType::LeftBrace, TokenType::LeftBracket],
                    found: token,
                    message: "SON files can only begin with either a { or [".to_string(),
                }),
            };
        }
        return Err(ParseError::UnexpectedEOF);
    }

    fn parse_value(&mut self, token: Token) -> Result<SonValue, ParseError> {
        return match token.get_type() {
            TokenType::True
            | TokenType::False
            | TokenType::Null
            | TokenType::IntegerLiteral
            | TokenType::FloatLiteral
            | TokenType::StringLiteral
            | TokenType::CharLiteral => {
                let son_value = token.get_value().unwrap();
                let son_value = if self.negate_flag {
                    son_value.negate()
                } else {
                    son_value
                };
                self.negate_flag = false;
                Ok(son_value)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: vec![
                    TokenType::True,
                    TokenType::False,
                    TokenType::Null,
                    TokenType::IntegerLiteral,
                    TokenType::FloatLiteral,
                    TokenType::StringLiteral,
                    TokenType::CharLiteral,
                ],
                found: token,
                message: "".to_string(),
            }),
        };
    }

    fn parse_object(&mut self) -> Result<SonValue, ParseError> {
        let mut object_map: HashMap<String, SonValue> = HashMap::new();

        let mut field_name = String::new();
        while let Some(token) = self.lexer.next() {
            match token.get_type() {
                TokenType::LeftParen => {}
                TokenType::RightParen => {}
                TokenType::LeftBrace => {
                    let object = self.parse_object()?;
                    object_map.insert(field_name, object);
                    field_name = String::new();
                }
                TokenType::RightBrace => break,
                TokenType::LeftBracket => {
                    let array = self.parse_array()?;
                    object_map.insert(field_name, array);
                    field_name = String::new();
                }
                TokenType::RightBracket => {}
                TokenType::Comma => {}
                TokenType::Dot => {}
                TokenType::Colon => {}
                TokenType::Negative => self.negate_flag = true,
                TokenType::True
                | TokenType::False
                | TokenType::Null
                | TokenType::IntegerLiteral
                | TokenType::FloatLiteral
                | TokenType::StringLiteral
                | TokenType::CharLiteral => {
                    let value = self.parse_value(token)?;
                    object_map.insert(field_name, value);
                    field_name = String::new();
                }
                TokenType::Identifier => {
                    if self
                        .lexer
                        .previous_token()
                        .is_some_and(|t| t.get_type() == TokenType::Colon)
                    {
                        object_map.insert(field_name, SonValue::Enum(token.get_source()));
                        field_name = String::new();
                    } else {
                        field_name = token.get_source()
                    }
                }
                TokenType::Error => {}
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: vec![
                            TokenType::True,
                            TokenType::False,
                            TokenType::Null,
                            TokenType::IntegerLiteral,
                            TokenType::FloatLiteral,
                            TokenType::StringLiteral,
                            TokenType::CharLiteral,
                            TokenType::Identifier,
                        ],
                        found: token,
                        message: "".to_string(),
                    });
                }
            }
        }

        return Ok(SonValue::Object(object_map));
    }

    fn parse_array(&mut self) -> Result<SonValue, ParseError> {
        let mut value_array: Vec<SonValue> = Vec::new();
        while let Some(token) = self.lexer.next() {
            match token.get_type() {
                TokenType::LeftParen => {}
                TokenType::RightParen => {}
                TokenType::LeftBrace => {
                    let object = self.parse_object()?;
                    value_array.push(object);
                }
                TokenType::RightBrace => {}
                TokenType::LeftBracket => {
                    let array = self.parse_array()?;
                    value_array.push(array);
                }
                TokenType::RightBracket => break,
                TokenType::Comma => {}
                TokenType::Dot => {}
                TokenType::Colon => {}
                TokenType::Negative => {}
                TokenType::True
                | TokenType::False
                | TokenType::Null
                | TokenType::IntegerLiteral
                | TokenType::FloatLiteral
                | TokenType::StringLiteral
                | TokenType::CharLiteral => {
                    let value = self.parse_value(token)?;
                    value_array.push(value);
                }
                TokenType::Identifier => {}
                TokenType::Error => {}
                TokenType::EOF => {}
            }
        }

        return Ok(SonValue::Array(value_array));
    }
}
