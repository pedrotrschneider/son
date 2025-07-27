use crate::{DeserializationError, Deserialize, FromSon, Printer, Serialize, ToSon};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Float(f64),
    Integer(i128),
    String(String),
    Char(char),
    Enum(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn get_type(&self) -> String {
        return match self {
            Value::Null => "Null".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Float(_) => "Float".to_string(),
            Value::Integer(_) => "Integer".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Char(_) => "Char".to_string(),
            Value::Enum(_) => "Enum".to_string(),
            Value::Array(_) => "Array".to_string(),
            Value::Object(_) => "Object".to_string(),
        };
    }

    pub fn negate(&self) -> Value {
        return match self {
            Value::Bool(b) => Value::Bool(!b),
            Value::Float(f) => Value::Float(-f),
            Value::Integer(i) => Value::Integer(-i),
            _ => self.clone(),
        };
    }
}

impl Deserialize for Value {}
impl FromSon for Value {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        return Ok(son);
    }
}

impl Serialize for Value {}
impl ToSon for Value {
    fn to_son(&self) -> Value {
        return self.clone();
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let printer = Printer::new("    ".to_string());
        write!(f, "{}", printer.son_to_string(&self))
    }
}
