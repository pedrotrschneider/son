use son::{
    DeserializationError,
    Deserialize,
    FromSon,
    Serialize,
    ToSon,
    Value,
};

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
    let result = son::from_file::<Value>("src/test.son");
    assert!(result.is_ok(), "{}", result.err().unwrap());
}

// --- New Test Cases for Native Types and Nested Structures ---

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
    let data = vec![
        "string1".to_string(),
        "string2".to_string(),
        "string3".to_string(),
    ];
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
