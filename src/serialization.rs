use crate::error::DeserializationError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SonValue {
    Null,
    Bool(bool),
    Float(f64),
    Integer(i128),
    String(String),
    Char(char),
    Enum(String),
    Array(Vec<SonValue>),
    Object(HashMap<String, SonValue>),
}

impl SonValue {
    pub fn get_type(&self) -> String {
        return match self {
            SonValue::Null => "Null".to_string(),
            SonValue::Bool(_) => "Bool".to_string(),
            SonValue::Float(_) => "Float".to_string(),
            SonValue::Integer(_) => "Integer".to_string(),
            SonValue::String(_) => "String".to_string(),
            SonValue::Char(_) => "Char".to_string(),
            SonValue::Enum(_) => "Enum".to_string(),
            SonValue::Array(_) => "Array".to_string(),
            SonValue::Object(_) => "Object".to_string(),
        };
    }

    pub fn to_string(&self) -> String {
        let printer = SonPrinter::new("    ".to_string());
        return printer.son_to_string(&self);
    }

    pub fn negate(&self) -> SonValue {
        return match self {
            SonValue::Bool(b) => SonValue::Bool(!b),
            SonValue::Float(f) => SonValue::Float(-f),
            SonValue::Integer(i) => SonValue::Integer(-i),
            _ => self.clone(),
        };
    }
}

pub trait FromSon: Sized {
    fn from_son(son: SonValue) -> Result<Self, DeserializationError>;
}

pub trait Deserialize: FromSon {}

pub trait ToSon {
    fn to_son(&self) -> SonValue;
}

pub trait Serialize: ToSon {}

impl Serialize for String {}
impl ToSon for String {
    fn to_son(&self) -> SonValue {
        SonValue::String(self.clone())
    }
}

impl Serialize for &str {}
impl ToSon for &str {
    fn to_son(&self) -> SonValue {
        SonValue::String(self.to_string())
    }
}

impl Serialize for bool {}
impl ToSon for bool {
    fn to_son(&self) -> SonValue {
        SonValue::Bool(*self)
    }
}

impl Serialize for i32 {}
impl ToSon for i32 {
    fn to_son(&self) -> SonValue {
        SonValue::Integer(*self as i128)
    }
}

impl Serialize for f64 {}
impl ToSon for f64 {
    fn to_son(&self) -> SonValue {
        SonValue::Float(*self)
    }
}

impl Serialize for char {}
impl ToSon for char {
    fn to_son(&self) -> SonValue {
        SonValue::Char(*self)
    }
}

impl<T> ToSon for Vec<T>
where
    T: ToSon,
{
    fn to_son(&self) -> SonValue {
        SonValue::Array(self.iter().map(|v| v.to_son()).collect())
    }
}

pub struct SonPrinter {
    indentation: String,
}

impl SonPrinter {
    pub fn new(indentation: String) -> Self {
        return Self { indentation };
    }

    pub fn son_to_string(&self, son: &SonValue) -> String {
        return self.son_to_string_inner(son, 0);
    }

    fn son_to_string_inner(&self, son: &SonValue, indent: u32) -> String {
        let mut string = String::new();

        let print_indent = |string: &mut String, indent: u32| {
            for _ in 0..indent {
                string.push_str(&self.indentation);
            }
        };

        match son {
            SonValue::Null => string.push_str("null"),
            SonValue::Bool(b) => string.push_str(&b.to_string()),
            SonValue::Float(f) => string.push_str(&f.to_string()),
            SonValue::Integer(i) => string.push_str(&i.to_string()),
            SonValue::String(s) => {
                string.push('\"');
                string.push_str(&s);
                string.push('\"');
            }
            SonValue::Char(c) => {
                string.push('\"');
                string.push_str(&c.to_string());
                string.push('\"');
            }
            SonValue::Enum(s) => string.push_str(&s),
            SonValue::Array(a) => {
                string.push_str("[\n");
                for value in a {
                    print_indent(&mut string, indent + 1);
                    string.push_str(&self.son_to_string_inner(&value, indent + 1));
                }
                print_indent(&mut string, indent);
                string.push(']');
            }
            SonValue::Object(o) => {
                string.push_str("{\n");

                for (key, value) in o.iter() {
                    print_indent(&mut string, indent + 1);
                    string.push_str(&key.to_string());
                    string.push_str(": ");
                    string.push_str(&self.son_to_string_inner(&value, indent + 1));
                }
                print_indent(&mut string, indent);
                string.push('}');
            }
        };
        string.push_str(",\n");
        return string;
    }
}
