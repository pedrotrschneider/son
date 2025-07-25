use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SonValue {
    Null,
    Bool(bool),
    Float(f64),
    Integer(i128),
    String(String),
    Char(char),
    Array(Vec<SonValue>),
    Object(HashMap<String, SonValue>),
}

impl SonValue {
    pub fn negate(&self) -> SonValue {
        return match self {
            SonValue::Bool(b) => SonValue::Bool(!b),
            SonValue::Float(f) => SonValue::Float(-f),
            SonValue::Integer(i) => SonValue::Integer(-i),
            _ => self.clone(),
        };
    }
}

pub trait Deserialize: Default {
    fn set_field(&mut self, path: &[&str], value: SonValue) -> Result<(), String>;
}
