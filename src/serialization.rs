use crate::error::DeserializationError;
use crate::value::Value;
use std::collections::HashMap;

pub trait Deserialize: FromSon {}
pub trait FromSon: Sized {
    fn from_son(son: Value) -> Result<Self, DeserializationError>;
}

fn default_error(expected: Value, found: Value) -> DeserializationError {
    return DeserializationError::UnexpectedType {
        expected: expected.get_type(),
        found: found.get_type(),
    };
}

// String types

impl Deserialize for String {}
impl FromSon for String {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        return match son {
            Value::String(s) => Ok(s),
            _ => Err(default_error(Value::String("".to_string()), son)),
        };
    }
}

impl Deserialize for char {}
impl FromSon for char {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        return match son {
            Value::Char(c) => Ok(c),
            _ => Err(default_error(Value::Char(' '), son)),
        };
    }
}

// Boolean types

impl Deserialize for bool {}
impl FromSon for bool {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        return match son {
            Value::Bool(b) => Ok(b),
            _ => Err(default_error(Value::Bool(false), son)),
        };
    }
}

// Integer types

impl Deserialize for i8 {}
impl FromSon for i8 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as i8),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for i16 {}
impl FromSon for i16 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as i16),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for i32 {}
impl FromSon for i32 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as i32),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for i64 {}
impl FromSon for i64 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as i64),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for i128 {}
impl FromSon for i128 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for isize {}
impl FromSon for isize {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as isize),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

// Unsigned integer types

impl Deserialize for u8 {}
impl FromSon for u8 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as u8),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for u16 {}
impl FromSon for u16 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as u16),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for u32 {}
impl FromSon for u32 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as u32),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for u64 {}
impl FromSon for u64 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as u64),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

// NOTE: You cannot deserialize u128 because they are internally stored as i128

impl Deserialize for usize {}
impl FromSon for usize {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as usize),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

// Floating point types

impl Deserialize for f32 {}
impl FromSon for f32 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as f32),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

impl Deserialize for f64 {}
impl FromSon for f64 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as f64),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

// Vector

impl Deserialize for Vec<Value> {}
impl FromSon for Vec<Value> {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Array(a) => Ok(a.clone()),
            _ => Err(default_error(Value::Array(vec![]), son)),
        }
    }
}

// HashMap

impl Deserialize for HashMap<String, Value> {}
impl FromSon for HashMap<String, Value> {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Object(o) => Ok(o.clone()),
            _ => Err(default_error(Value::Object(HashMap::new()), son)),
        }
    }
}

pub trait Serialize: ToSon {}
pub trait ToSon {
    fn to_son(&self) -> Value;
}

// String types

impl Serialize for String {}
impl ToSon for String {
    fn to_son(&self) -> Value {
        return Value::String(self.clone());
    }
}

impl Serialize for &str {}
impl ToSon for &str {
    fn to_son(&self) -> Value {
        return Value::String(self.to_string());
    }
}

impl Serialize for char {}
impl ToSon for char {
    fn to_son(&self) -> Value {
        return Value::Char(*self);
    }
}

// Boolean types

impl Serialize for bool {}
impl ToSon for bool {
    fn to_son(&self) -> Value {
        return Value::Bool(*self);
    }
}

// Integer types

impl Serialize for i8 {}
impl ToSon for i8 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for i16 {}
impl ToSon for i16 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for i32 {}
impl ToSon for i32 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for i64 {}
impl ToSon for i64 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for i128 {}
impl ToSon for i128 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self);
    }
}

impl Serialize for isize {}
impl ToSon for isize {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

// Unsigned integer types

impl Serialize for u8 {}
impl ToSon for u8 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for u16 {}
impl ToSon for u16 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for u32 {}
impl ToSon for u32 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

impl Serialize for u64 {}
impl ToSon for u64 {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

// NOTE: You cannot safely serialize u128 because they are internally stored as i128

impl Serialize for usize {}
impl ToSon for usize {
    fn to_son(&self) -> Value {
        return Value::Integer(*self as i128);
    }
}

// Floating point types

impl Serialize for f32 {}
impl ToSon for f32 {
    fn to_son(&self) -> Value {
        return Value::Float(*self as f64);
    }
}

impl Serialize for f64 {}
impl ToSon for f64 {
    fn to_son(&self) -> Value {
        return Value::Float(*self);
    }
}

// Vector

impl<T> Serialize for Vec<T> where T: Serialize {}
impl<T> ToSon for Vec<T>
where
    T: Serialize,
{
    fn to_son(&self) -> Value {
        return Value::Array(self.iter().map(|v| v.to_son()).collect());
    }
}

// HashMap

impl<K, V> Serialize for HashMap<K, V>
where
    K: ToString,
    V: Serialize,
{
}
impl<K, V> ToSon for HashMap<K, V>
where
    K: ToString,
    V: Serialize,
{
    fn to_son(&self) -> Value {
        let mut hash_map: HashMap<String, Value> = HashMap::new();
        for (key, value) in self.iter() {
            hash_map.insert(key.to_string(), value.to_son());
        }
        return Value::Object(hash_map);
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
        string.push('\n');
        return string;
    }
}
