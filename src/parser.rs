use crate::lexer::Lexer;
use crate::serialization::SonValue;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Parser {
    lexer: Lexer,
    negate_flag: bool,
}

impl Parser {
    pub fn from_file_to_son_object(file_path: &str) -> SonValue {
        let mut parser = Self {
            lexer: Lexer::new(file_path),
            negate_flag: false,
        };
        return parser.parse();
    }

    pub fn parse(&mut self) -> SonValue {
        if let Some(token) = self.lexer.next() {
            return match token.get_type() {
                TokenType::LeftBrace => self.parse_object(),
                TokenType::LeftBracket => self.parse_array(),
                _ => panic!("SON files can only begin with either {{ or ["),
            };
        }
        return SonValue::Null;
    }

    fn parse_value(&mut self, token: Token) -> SonValue {
        match token.get_type() {
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
                return son_value;
            }
            _ => {}
        }

        return SonValue::Null;
    }

    fn parse_object(&mut self) -> SonValue {
        let mut object_map: HashMap<String, SonValue> = HashMap::new();

        let mut field_name = String::new();
        while let Some(token) = self.lexer.next() {
            match token.get_type() {
                TokenType::LeftParen => {}
                TokenType::RightParen => {}
                TokenType::LeftBrace => {
                    let object = self.parse_object();
                    object_map.insert(field_name, object);
                    field_name = String::new();
                }
                TokenType::RightBrace => break,
                TokenType::LeftBracket => {
                    let array = self.parse_array();
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
                    let value = self.parse_value(token);
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
                TokenType::EOF => {}
            }
        }

        return SonValue::Object(object_map);
    }

    fn parse_array(&mut self) -> SonValue {
        let mut value_array: Vec<SonValue> = Vec::new();
        while let Some(token) = self.lexer.next() {
            match token.get_type() {
                TokenType::LeftParen => {}
                TokenType::RightParen => {}
                TokenType::LeftBrace => {
                    let object = self.parse_object();
                    value_array.push(object);
                }
                TokenType::RightBrace => {}
                TokenType::LeftBracket => {
                    let array = self.parse_array();
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
                    let value = self.parse_value(token);
                    value_array.push(value);
                }
                TokenType::Identifier => {}
                TokenType::Error => {}
                TokenType::EOF => {}
            }
        }

        return SonValue::Array(value_array);
    }
}
