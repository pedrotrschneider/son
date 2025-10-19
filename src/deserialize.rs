use crate::{
    error::DeserializationError,
    value::Value,
};
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
            _ => Err(default_error(Value::String(String::new()), son)),
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

impl Deserialize for u128 {}
impl FromSon for u128 {
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Integer(i) => Ok(i as u128),
            _ => Err(default_error(Value::Integer(0), son)),
        }
    }
}

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

impl<T> Deserialize for Vec<T> where T: Deserialize {}
impl<T> FromSon for Vec<T>
where
    T: Deserialize,
{
    fn from_son(son: Value) -> Result<Self, DeserializationError> {
        match son {
            Value::Array(a) => Ok(a.iter().map(|v| T::from_son(v.clone()).unwrap()).collect()),
            _ => Err(default_error(Value::Array(Vec::new()), son)),
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
