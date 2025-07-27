use crate::Value;
use std::collections::HashMap;

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
