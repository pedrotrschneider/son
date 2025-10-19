use son::{DeserializationError, Deserialize, FromSon, Serialize, ToSon, Value};
use std::char;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum PhoneType {
    Home,
    Office,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct Address {
    street_address: String,
    city: String,
    state: String,
    postal_code: String,
    latitude: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct PhoneNumber {
    ty: PhoneType,
    number: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct Client {
    first_name: String,
    last_name: String,
    age: i32,
    is_alive: bool,
    is_retired: bool,
    initial: char,
    address: Address,
    phone_numbers: Vec<PhoneNumber>,
}

#[test]
fn test_from_file() {
    let result = son::from_file::<Value>("tests/son/test.son");
    assert!(result.is_ok(), "{}", result.err().unwrap());
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct TestPrimitives {
    my_string: String,
    my_char: char,
    my_bool: bool,
    my_f32: f32,
    my_f64: f64,
    my_i8: i8,
    my_i16: i16,
    my_i32: i32,
    my_i64: i64,
    my_i128: i128,
    my_isize: isize,
    my_u8: u8,
    my_u16: u16,
    my_u32: u32,
    my_u64: u64,
    my_usize: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct NestedVecs {
    level1: Vec<String>,
    level2: Vec<Vec<i32>>,
    level3: Vec<Vec<Vec<bool>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct NestedObjectLevel1 {
    data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct NestedObjectLevel2 {
    id: u32,
    level1: NestedObjectLevel1,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct NestedObjectLevel3 {
    is_active: bool,
    level2: NestedObjectLevel2,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct NestedObjectLevel4 {
    name: String,
    level3: NestedObjectLevel3,
}

#[test]
fn test_primitives_serialization_deserialization() {
    let data = TestPrimitives {
        my_string: "Hello, SON!".to_string(),
        my_char: 'S',
        my_bool: true,
        my_f32: 3.14f32,
        my_f64: std::f64::consts::E,
        my_i8: -8,
        my_i16: -16,
        my_i32: -32,
        my_i64: -64,
        my_i128: -128,
        my_isize: -1,
        my_u8: 8,
        my_u16: 16,
        my_u32: 32,
        my_u64: 64,
        my_usize: 1,
    };

    let son_string = data.to_son().to_string();
    let deserialized_data: TestPrimitives = son::from_str(&son_string).unwrap();

    assert_eq!(data, deserialized_data);
}

#[test]
fn test_vec_of_primitives_serialization_deserialization() {
    let data = vec!["string1".to_string(), "string2".to_string(), "string3".to_string()];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<String> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);

    let data = vec!['a', 'b', 'c'];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<char> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);

    let data = vec![true, false, true];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<bool> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);

    let data = vec![1.0f32, 2.0f32, 3.0f32];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<f32> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);

    let data = vec![1.0f64, 2.0f64, 3.0f64];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<f64> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);

    let data = vec![1i8, 2i8, 3i8];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<i8> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);

    let data = vec![1u8, 2u8, 3u8];
    let son_string = data.to_son().to_string();
    let deserialized_data: Vec<u8> = son::from_str(&son_string).unwrap();
    assert_eq!(data, deserialized_data);
}

#[test]
fn test_nested_vecs_serialization_deserialization() {
    let data = NestedVecs {
        level1: vec!["one".to_string(), "two".to_string()],
        level2: vec![vec![1, 2], vec![3, 4, 5]],
        level3: vec![vec![vec![true, false], vec![true]]],
    };

    let son_string = data.to_son().to_string();
    println!("{}", son_string);
    let deserialized_data: NestedVecs = son::from_str(&son_string).unwrap();

    assert_eq!(data, deserialized_data);
}

#[test]
fn test_nested_objects_serialization_deserialization() {
    let data = NestedObjectLevel4 {
        name: "Level 4".to_string(),
        level3: NestedObjectLevel3 {
            is_active: true,
            level2: NestedObjectLevel2 {
                id: 123,
                level1: NestedObjectLevel1 {
                    data: "Level 1 Data".to_string(),
                },
            },
        },
    };

    let son_string = data.to_son().to_string();
    let deserialized_data: NestedObjectLevel4 = son::from_str(&son_string).unwrap();

    assert_eq!(data, deserialized_data);
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct TestPhoneType {
    phone_type: PhoneType,
}

#[test]
fn test_enum_serialization_deserialization() {
    let home = TestPhoneType {
        phone_type: PhoneType::Home,
    };
    let office = TestPhoneType {
        phone_type: PhoneType::Office,
    };

    let son_home = home.to_son().to_string();
    let son_office = office.to_son().to_string();

    let deserialized_home: TestPhoneType = son::from_str(&son_home).unwrap();
    let deserialized_office: TestPhoneType = son::from_str(&son_office).unwrap();

    assert_eq!(home, deserialized_home);
    assert_eq!(office, deserialized_office);
}

#[test]
fn test_complex_client_serialization_deserialization() {
    let client = Client {
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        age: 25,
        is_alive: true,
        is_retired: false,
        initial: 'J',
        address: Address {
            street_address: "10 Downing St".to_string(),
            city: "London".to_string(),
            state: "England".to_string(),
            postal_code: "SW1A 2AA".to_string(),
            latitude: 51.5007,
        },
        phone_numbers: vec![
            PhoneNumber {
                ty: PhoneType::Home,
                number: "+44 20 7946 0123".to_string(),
            },
            PhoneNumber {
                ty: PhoneType::Office,
                number: "+44 20 7946 0456".to_string(),
            },
        ],
    };

    let son_string = client.to_son().to_string();
    let deserialized_client: Client = son::from_str(&son_string).unwrap();

    assert_eq!(client, deserialized_client);
}

#[test]
fn test_deserialization() {
    // Testing string deserialization
    {
        let string_value = Value::String("Hello, World!".to_string());
        let result = String::from_son(string_value);
        match result {
            Ok(s) => assert_eq!(s, "Hello, World!"),
            Err(e) => panic!("[String] Expected Ok but got Err: {}", e),
        }
    }

    // Testing char deserialization
    {
        let char_value = Value::Char('A');
        let result = char::from_son(char_value);
        match result {
            Ok(c) => assert_eq!(c, 'A'),
            Err(e) => panic!("[Char] Expected Ok but got Err: {}", e),
        }
    }

    // Testing bool deserialization
    {
        let bool_value = Value::Bool(true);
        let result = bool::from_son(bool_value);
        match result {
            Ok(b) => assert_eq!(b, true),
            Err(e) => panic!("[Bool] Expected Ok but got Err: {}", e),
        }
    }

    // Testing i8 deserialization
    {
        let i8_value = Value::Integer(42);
        let result = i8::from_son(i8_value);
        match result {
            Ok(i) => assert_eq!(i, 42i8),
            Err(e) => panic!("[i8] Expected Ok but got Err: {}", e),
        }
    }

    // Testing i16 deserialization
    {
        let i16_value = Value::Integer(1000);
        let result = i16::from_son(i16_value);
        match result {
            Ok(i) => assert_eq!(i, 1000i16),
            Err(e) => panic!("[i16] Expected Ok but got Err: {}", e),
        }
    }

    // Testing i32 deserialization
    {
        let i32_value = Value::Integer(100000);
        let result = i32::from_son(i32_value);
        match result {
            Ok(i) => assert_eq!(i, 100000i32),
            Err(e) => panic!("[i32] Expected Ok but got Err: {}", e),
        }
    }

    // Testing i64 deserialization
    {
        let i64_value = Value::Integer(9223372036854775807);
        let result = i64::from_son(i64_value);
        match result {
            Ok(i) => assert_eq!(i, 9223372036854775807i64),
            Err(e) => panic!("[i64] Expected Ok but got Err: {}", e),
        }
    }

    // Testing i128 deserialization
    {
        let i128_value = Value::Integer(123456789012345);
        let result = i128::from_son(i128_value);
        match result {
            Ok(i) => assert_eq!(i, 123456789012345i128),
            Err(e) => panic!("[i128] Expected Ok but got Err: {}", e),
        }
    }

    // Testing isize deserialization
    {
        let isize_value = Value::Integer(12345);
        let result = isize::from_son(isize_value);
        match result {
            Ok(i) => assert_eq!(i, 12345isize),
            Err(e) => panic!("[isize] Expected Ok but got Err: {}", e),
        }
    }

    // Testing u8 deserialization
    {
        let u8_value = Value::Integer(255);
        let result = u8::from_son(u8_value);
        match result {
            Ok(i) => assert_eq!(i, 255u8),
            Err(e) => panic!("[u8] Expected Ok but got Err: {}", e),
        }
    }

    // Testing u16 deserialization
    {
        let u16_value = Value::Integer(65535);
        let result = u16::from_son(u16_value);
        match result {
            Ok(i) => assert_eq!(i, 65535u16),
            Err(e) => panic!("[u16] Expected Ok but got Err: {}", e),
        }
    }

    // Testing u32 deserialization
    {
        let u32_value = Value::Integer(4294967295);
        let result = u32::from_son(u32_value);
        match result {
            Ok(i) => assert_eq!(i, 4294967295u32),
            Err(e) => panic!("[u32] Expected Ok but got Err: {}", e),
        }
    }

    // Testing u64 deserialization
    {
        let u64_value = Value::Integer(18446744073709551615);
        let result = u64::from_son(u64_value);
        match result {
            Ok(i) => assert_eq!(i, 18446744073709551615u64),
            Err(e) => panic!("[u64] Expected Ok but got Err: {}", e),
        }
    }

    // Testing u128 deserialization
    {
        let u128_value = Value::Integer(987654321098765);
        let result = u128::from_son(u128_value);
        match result {
            Ok(i) => assert_eq!(i, 987654321098765u128),
            Err(e) => panic!("[u128] Expected Ok but got Err: {}", e),
        }
    }

    // Testing usize deserialization
    {
        let usize_value = Value::Integer(54321);
        let result = usize::from_son(usize_value);
        match result {
            Ok(i) => assert_eq!(i, 54321usize),
            Err(e) => panic!("[usize] Expected Ok but got Err: {}", e),
        }
    }

    // Testing f32 deserialization
    {
        let f32_value = Value::Integer(42);
        let result = f32::from_son(f32_value);
        match result {
            Ok(f) => assert_eq!(f, 42.0f32),
            Err(e) => panic!("[f32] Expected Ok but got Err: {}", e),
        }
    }

    // Testing f64 deserialization
    {
        let f64_value = Value::Integer(123);
        let result = f64::from_son(f64_value);
        match result {
            Ok(f) => assert_eq!(f, 123.0f64),
            Err(e) => panic!("[f64] Expected Ok but got Err: {}", e),
        }
    }

    // Testing Vec<T> deserialization
    {
        let vec_value = Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
        let result = Vec::<i32>::from_son(vec_value);
        match result {
            Ok(v) => assert_eq!(v, vec![1, 2, 3]),
            Err(e) => panic!("[Vec<i32>] Expected Ok but got Err: {}", e),
        }
    }

    // Testing HashMap deserialization
    {
        let mut expected_map = HashMap::new();
        expected_map.insert("key1".to_string(), Value::Integer(42));
        expected_map.insert("key2".to_string(), Value::String("value".to_string()));

        let hashmap_value = Value::Object(expected_map.clone());
        let result = HashMap::<String, Value>::from_son(hashmap_value);
        match result {
            Ok(m) => {
                assert_eq!(m.len(), 2);
                assert_eq!(m.get("key1"), Some(&Value::Integer(42)));
                assert_eq!(m.get("key2"), Some(&Value::String("value".to_string())));
            }
            Err(e) => panic!("[HashMap] Expected Ok but got Err: {}", e),
        }
    }
}

#[test]
fn test_deserialization_errors() {
    // Testing string deserialization error
    {
        let not_a_string = Value::Bool(false);
        let string_error = String::from_son(not_a_string);
        match string_error {
            Ok(_) => panic!("[String] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::String(String::new()).get_type());
                    assert_eq!(found, Value::Bool(false).get_type());
                }
                _ => panic!("[String] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing char deserialization error
    {
        let not_a_char = Value::String("Hello".to_string());
        let char_error = char::from_son(not_a_char);
        match char_error {
            Ok(_) => panic!("[Char] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Char(' ').get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[Char] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing bool deserialization error
    {
        let not_a_bool = Value::Integer(42);
        let bool_error = bool::from_son(not_a_bool);
        match bool_error {
            Ok(_) => panic!("[Bool] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Bool(false).get_type());
                    assert_eq!(found, Value::Integer(0).get_type());
                }
                _ => panic!("[Bool] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing i8 deserialization error
    {
        let not_an_integer = Value::String("not a number".to_string());
        let i8_error = i8::from_son(not_an_integer);
        match i8_error {
            Ok(_) => panic!("[i8] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[i8] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing i16 deserialization error
    {
        let not_an_integer = Value::Bool(true);
        let i16_error = i16::from_son(not_an_integer);
        match i16_error {
            Ok(_) => panic!("[i16] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Bool(false).get_type());
                }
                _ => panic!("[i16] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing i32 deserialization error
    {
        let not_an_integer = Value::Char('x');
        let i32_error = i32::from_son(not_an_integer);
        match i32_error {
            Ok(_) => panic!("[i32] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Char(' ').get_type());
                }
                _ => panic!("[i32] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing i64 deserialization error
    {
        let not_an_integer = Value::Array(vec![]);
        let i64_error = i64::from_son(not_an_integer);
        match i64_error {
            Ok(_) => panic!("[i64] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Array(vec![]).get_type());
                }
                _ => panic!("[i64] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing i128 deserialization error
    {
        let not_an_integer = Value::String("123".to_string());
        let i128_error = i128::from_son(not_an_integer);
        match i128_error {
            Ok(_) => panic!("[i128] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[i128] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing isize deserialization error
    {
        let not_an_integer = Value::Bool(false);
        let isize_error = isize::from_son(not_an_integer);
        match isize_error {
            Ok(_) => panic!("[isize] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Bool(false).get_type());
                }
                _ => panic!("[isize] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing u8 deserialization error
    {
        let not_an_integer = Value::String("not a number".to_string());
        let u8_error = u8::from_son(not_an_integer);
        match u8_error {
            Ok(_) => panic!("[u8] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[u8] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing u16 deserialization error
    {
        let not_an_integer = Value::Char('y');
        let u16_error = u16::from_son(not_an_integer);
        match u16_error {
            Ok(_) => panic!("[u16] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Char(' ').get_type());
                }
                _ => panic!("[u16] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing u32 deserialization error
    {
        let not_an_integer = Value::Bool(true);
        let u32_error = u32::from_son(not_an_integer);
        match u32_error {
            Ok(_) => panic!("[u32] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Bool(false).get_type());
                }
                _ => panic!("[u32] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing u64 deserialization error
    {
        let not_an_integer = Value::Array(vec![]);
        let u64_error = u64::from_son(not_an_integer);
        match u64_error {
            Ok(_) => panic!("[u64] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Array(vec![]).get_type());
                }
                _ => panic!("[u64] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing u128 deserialization error
    {
        let not_an_integer = Value::String("456".to_string());
        let u128_error = u128::from_son(not_an_integer);
        match u128_error {
            Ok(_) => panic!("[u128] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[u128] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing usize deserialization error
    {
        let not_an_integer = Value::Char('z');
        let usize_error = usize::from_son(not_an_integer);
        match usize_error {
            Ok(_) => panic!("[usize] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Char(' ').get_type());
                }
                _ => panic!("[usize] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing f32 deserialization error
    {
        let not_a_float = Value::String("3.14".to_string());
        let f32_error = f32::from_son(not_a_float);
        match f32_error {
            Ok(_) => panic!("[f32] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[f32] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing f64 deserialization error
    {
        let not_a_float = Value::Bool(false);
        let f64_error = f64::from_son(not_a_float);
        match f64_error {
            Ok(_) => panic!("[f64] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Integer(0).get_type());
                    assert_eq!(found, Value::Bool(false).get_type());
                }
                _ => panic!("[f64] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing Vec<T> deserialization error
    {
        let not_an_array = Value::Integer(42);
        let vec_error = Vec::<i32>::from_son(not_an_array);
        match vec_error {
            Ok(_) => panic!("[Vec<i32>] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Array(vec![]).get_type());
                    assert_eq!(found, Value::Integer(0).get_type());
                }
                _ => panic!("[Vec<i32>] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }

    // Testing HashMap deserialization error
    {
        let not_an_object = Value::String("not an object".to_string());
        let hashmap_error = HashMap::<String, Value>::from_son(not_an_object);
        match hashmap_error {
            Ok(_) => panic!("[HashMap] Expected and Err but got Ok"),
            Err(e) => match e {
                DeserializationError::UnexpectedType { expected, found } => {
                    assert_eq!(expected, Value::Object(HashMap::new()).get_type());
                    assert_eq!(found, Value::String(String::new()).get_type());
                }
                _ => panic!("[HashMap] Expected DeserializationError::UnexpectedType but got {}", e),
            },
        }
    }
}

#[test]
fn test_serialization() {
    // Testing &str serialization
    {
        let son_str = "Hello".to_son();
        match son_str {
            Value::String(s) => assert_eq!(s, "Hello"),
            _ => panic!("[&str] Expected Value::String but got {}", son_str),
        }
    }

    // Testing HashMap<String, i32> serialization
    {
        let mut map = HashMap::new();
        map.insert("age".to_string(), 30);
        map.insert("score".to_string(), 100);

        let result = map.to_son();

        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 2);
                assert_eq!(obj.get("age"), Some(&Value::Integer(30)));
                assert_eq!(obj.get("score"), Some(&Value::Integer(100)));
            }
            _ => panic!("[HashMap<String, i32>] Expected Value::Object but got {:?}", result),
        }
    }

    // Testing HashMap<&str, String> serialization
    {
        let mut map = HashMap::new();
        map.insert("name", "Alice".to_string());
        map.insert("city", "New York".to_string());

        let result = map.to_son();

        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 2);
                assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
                assert_eq!(obj.get("city"), Some(&Value::String("New York".to_string())));
            }
            _ => panic!("[HashMap<&str, String>] Expected Value::Object but got {:?}", result),
        }
    }

    // Testing HashMap<i32, bool> serialization (non-string keys)
    {
        let mut map = HashMap::new();
        map.insert(1, true);
        map.insert(2, false);
        map.insert(3, true);

        let result = map.to_son();

        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 3);
                assert_eq!(obj.get("1"), Some(&Value::Bool(true)));
                assert_eq!(obj.get("2"), Some(&Value::Bool(false)));
                assert_eq!(obj.get("3"), Some(&Value::Bool(true)));
            }
            _ => panic!("[HashMap<i32, bool>] Expected Value::Object but got {:?}", result),
        }
    }

    // Testing empty HashMap serialization
    {
        let map: HashMap<String, i32> = HashMap::new();

        let result = map.to_son();

        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 0);
                assert!(obj.is_empty());
            }
            _ => panic!("[HashMap (empty)] Expected Value::Object but got {:?}", result),
        }
    }

    // Testing nested HashMap serialization
    {
        let mut inner_map = HashMap::new();
        inner_map.insert("inner_key".to_string(), 42);

        let mut outer_map = HashMap::new();
        outer_map.insert("data".to_string(), inner_map);

        let result = outer_map.to_son();

        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 1);
                match obj.get("data") {
                    Some(Value::Object(inner_obj)) => {
                        assert_eq!(inner_obj.len(), 1);
                        assert_eq!(inner_obj.get("inner_key"), Some(&Value::Integer(42)));
                    }
                    _ => panic!("[Nested HashMap] Expected nested Value::Object"),
                }
            }
            _ => panic!("[Nested HashMap] Expected Value::Object but got {:?}", result),
        }
    }

    // Testing HashMap with Vec values
    {
        let mut map = HashMap::new();
        map.insert("numbers".to_string(), vec![1, 2, 3]);
        map.insert("more_numbers".to_string(), vec![4, 5, 6]);

        let result = map.to_son();

        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 2);
                match obj.get("numbers") {
                    Some(Value::Array(arr)) => {
                        assert_eq!(arr.len(), 3);
                        assert_eq!(arr[0], Value::Integer(1));
                        assert_eq!(arr[1], Value::Integer(2));
                        assert_eq!(arr[2], Value::Integer(3));
                    }
                    _ => panic!("[HashMap with Vec] Expected Value::Array for 'numbers'"),
                }
                match obj.get("more_numbers") {
                    Some(Value::Array(arr)) => {
                        assert_eq!(arr.len(), 3);
                        assert_eq!(arr[0], Value::Integer(4));
                        assert_eq!(arr[1], Value::Integer(5));
                        assert_eq!(arr[2], Value::Integer(6));
                    }
                    _ => panic!("[HashMap with Vec] Expected Value::Array for 'more_numbers'"),
                }
            }
            _ => panic!("[HashMap with Vec] Expected Value::Object but got {:?}", result),
        }
    }
}
