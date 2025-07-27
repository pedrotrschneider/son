use crate::error::DeserializationError;
use crate::value::Value;

pub trait FromSon: Sized {
    fn from_son(son: Value) -> Result<Self, DeserializationError>;
}

pub trait Deserialize: FromSon {}

pub trait ToSon {
    fn to_son(&self) -> Value;
}

pub trait Serialize: ToSon {}

impl Serialize for String {}
impl ToSon for String {
    fn to_son(&self) -> Value {
        Value::String(self.clone())
    }
}

impl Serialize for &str {}
impl ToSon for &str {
    fn to_son(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl Serialize for bool {}
impl ToSon for bool {
    fn to_son(&self) -> Value {
        Value::Bool(*self)
    }
}

impl Serialize for i32 {}
impl ToSon for i32 {
    fn to_son(&self) -> Value {
        Value::Integer(*self as i128)
    }
}

impl Serialize for f64 {}
impl ToSon for f64 {
    fn to_son(&self) -> Value {
        Value::Float(*self)
    }
}

impl Serialize for char {}
impl ToSon for char {
    fn to_son(&self) -> Value {
        Value::Char(*self)
    }
}

impl<T> ToSon for Vec<T>
where
    T: ToSon,
{
    fn to_son(&self) -> Value {
        Value::Array(self.iter().map(|v| v.to_son()).collect())
    }
}

pub struct SonPrinter {
    indentation: String,
}

impl SonPrinter {
    pub fn new(indentation: String) -> Self {
        return Self { indentation };
    }

    pub fn son_to_string(&self, son: &Value) -> String {
        return self.son_to_string_inner(son, 0);
    }

    fn son_to_string_inner(&self, son: &Value, indent: u32) -> String {
        let mut string = String::new();

        let print_indent = |string: &mut String, indent: u32| {
            for _ in 0..indent {
                string.push_str(&self.indentation);
            }
        };

        match son {
            Value::Null => string.push_str("null"),
            Value::Bool(b) => string.push_str(&b.to_string()),
            Value::Float(f) => string.push_str(&f.to_string()),
            Value::Integer(i) => string.push_str(&i.to_string()),
            Value::String(s) => {
                string.push('\"');
                string.push_str(&s);
                string.push('\"');
            }
            Value::Char(c) => {
                string.push('\"');
                string.push_str(&c.to_string());
                string.push('\"');
            }
            Value::Enum(s) => string.push_str(&s),
            Value::Array(a) => {
                string.push_str("[\n");
                for value in a {
                    print_indent(&mut string, indent + 1);
                    string.push_str(&self.son_to_string_inner(&value, indent + 1));
                }
                print_indent(&mut string, indent);
                string.push(']');
            }
            Value::Object(o) => {
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
