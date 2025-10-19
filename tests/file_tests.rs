use son::error::ParseStep;
use son::{DeserializationError, Deserialize, FromSon, ParseError, Serialize, ToSon, Value};

// --- Invalid Files ---
const INVALID_DOUBLE_QUOTE_CHAR_FILE: &str = "tests/son/invalid_double_quote_char.son";
const INVALID_DUPLICATE_KEYS_FILE: &str = "tests/son/invalid_duplicate_keys.son";
const INVALID_EMPTY_FILE: &str = "tests/son/invalid_empty_file.son";
const INVALID_MISSING_BRACE_FILE: &str = "tests/son/invalid_missing_brace.son";
const INVALID_MISSING_CLOSING_BRACE_FILE: &str = "tests/son/invalid_missing_closing_brace.son";
const INVALID_MISSING_COLON_FILE: &str = "tests/son/invalid_missing_colon.son";
const INVALID_MISSING_VALUE_FILE: &str = "tests/son/invalid_missing_value.son";
const INVALID_MIXED_TYPES_ARRAY_FILE: &str = "tests/son/invalid_mixed_types_array.son";
const INVALID_MULTIPLE_CHARS_FILE: &str = "tests/son/invalid_multiple_chars.son";
const INVALID_ONLY_WHITESPACES_FILE: &str = "tests/son/invalid_only_whitespaces.son";
const INVALID_SINGLE_QUOTE_STRING_FILE: &str = "tests/son/invalid_single_quote_string.son";
const INVALID_UNQUOTED_STRING_FILE: &str = "tests/son/invalid_unquoted_string.son";
const INVALID_INCORRECT_START_FILE: &str = "tests/son/invalid_incorrect_start.son";
const INVALID_UNEXPECTED_EOF_VALUE: &str = "tests/son/invalid_unexpected_eof_value.son";
const INVALID_UNEXPECTED_TOKEN_ON_OBJECT_IDENTIFIER: &str =
    "tests/son/invalid_unexpected_token_on_object_identifier.son";

// --- Valid Files ---
const VALID_ARRAYS_FILE: &str = "tests/son/valid_arrays.son";
const VALID_ARRAY_OF_OBJECTS_FILE: &str = "tests/son/valid_array_of_objects.son";
const VALID_BASIC_TYPES_FILE: &str = "tests/son/valid_basic_types.son";
const VALID_COMMENTS_FILE: &str = "tests/son/valid_comments.son";
const VALID_COMPLEX_NESTED_FILE: &str = "tests/son/valid_complex_nested.son";
const VALID_DEEPLY_NESTED_OBJECT_FILE: &str = "tests/son/valid_deeply_nested_object.son";
const VALID_EMPTY_STRUCTURES_FILE: &str = "tests/son/valid_empty_structures.son";
const VALID_MIXED_COMMAS_FILE: &str = "tests/son/valid_mixed_commas.son";
const VALID_NESTED_OBJECTS_FILE: &str = "tests/son/valid_nested_objects.son";
const VALID_NUMBER_EDGE_CASES_FILE: &str = "tests/son/valid_number_edge_cases.son";
const VALID_OPTIONAL_COMMAS_FILE: &str = "tests/son/valid_optional_commas.son";
const VALID_ROOT_ARRAY_FILE: &str = "tests/son/valid_root_array.son";
const VALID_SINGLE_VALUE_OBJECT_FILE: &str = "tests/son/valid_single_value_object.son";
const VALID_STRING_EDGE_CASES_FILE: &str = "tests/son/valid_string_edge_cases.son";
const VALID_WHITESPACE_VARIATIONS_FILE: &str = "tests/son/valid_whitespace_variations.son";

// ====================
// TEST FILE 1: valid_basic_types.son
// ====================
// Tests all basic types with commas

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct BasicTypes {
    string_field: String,
    char_field: char,
    bool_true: bool,
    bool_false: bool,
    int_positive: i32,
    int_negative: i32,
    int_zero: i32,
    float_value: f64,
    float_negative: f64,
}

#[test]
fn test_valid_basic_types() {
    let basic_types = BasicTypes {
        string_field: "Hello World".to_string(),
        char_field: 'A',
        bool_true: true,
        bool_false: false,
        int_positive: 42,
        int_negative: -100,
        int_zero: 0,
        float_value: 3.14,
        float_negative: -2.5,
    };

    let result = son::from_file::<BasicTypes>(VALID_BASIC_TYPES_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), basic_types);
    let to_son = basic_types.to_son();
    let from_son = son::from_file::<Value>(VALID_BASIC_TYPES_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 2: valid_optional_commas.son (valid_no_commas.son)
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NoCommas {
    name: String,
    age: i32,
    active: bool,
    score: f64,
}

#[test]
fn test_valid_optional_commas() {
    let no_commas = NoCommas {
        name: "Alice".to_string(),
        age: 30,
        active: true,
        score: 95.5,
    };

    let result = son::from_file::<NoCommas>(VALID_OPTIONAL_COMMAS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), no_commas);
    let to_son = no_commas.to_son();
    let from_son = son::from_file::<Value>(VALID_OPTIONAL_COMMAS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 3: valid_mixed_commas.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct MixedCommas {
    first: String,
    second: String,
    third: String,
    fourth: String,
}

#[test]
fn test_valid_mixed_commas() {
    let mixed_commas = MixedCommas {
        first: "John".to_string(),
        second: "Doe".to_string(),
        third: "Smith".to_string(),
        fourth: "Johnson".to_string(),
    };

    let result = son::from_file::<MixedCommas>(VALID_MIXED_COMMAS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), mixed_commas);
    let to_son = mixed_commas.to_son();
    let from_son = son::from_file::<Value>(VALID_MIXED_COMMAS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 4: valid_nested_objects.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Privacy {
    public: bool,
    friends_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Settings {
    theme: String,
    notifications: bool,
    privacy: Privacy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NestedObjects {
    user: User,
    settings: Settings,
}

#[test]
fn test_valid_nested_objects() {
    let nested_objects = NestedObjects {
        user: User {
            name: "Bob".to_string(),
            age: 25,
        },
        settings: Settings {
            theme: "dark".to_string(),
            notifications: true,
            privacy: Privacy {
                public: false,
                friends_only: true,
            },
        },
    };

    let result = son::from_file::<NestedObjects>(VALID_NESTED_OBJECTS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), nested_objects);
    let to_son = nested_objects.to_son();
    let from_son = son::from_file::<Value>(VALID_NESTED_OBJECTS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 5: valid_arrays.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Arrays {
    numbers: Vec<i32>,
    strings: Vec<String>,
    booleans: Vec<bool>,
    mixed_commas: Vec<i32>,
}

#[test]
fn test_valid_arrays() {
    let arrays = Arrays {
        numbers: vec![1, 2, 3, 4, 5],
        strings: vec!["hello".to_string(), "world".to_string(), "test".to_string()],
        booleans: vec![true, false, true],
        mixed_commas: vec![1, 2, 3, 4, 5],
    };

    let result = son::from_file::<Arrays>(VALID_ARRAYS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), arrays);
    let to_son = arrays.to_son();
    let from_son = son::from_file::<Value>(VALID_ARRAYS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 6: valid_array_of_objects.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct UserList {
    users: Vec<Person>,
}

#[test]
fn test_valid_array_of_objects() {
    let user_list = UserList {
        users: vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
            },
        ],
    };

    let result = son::from_file::<UserList>(VALID_ARRAY_OF_OBJECTS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), user_list);
    let to_son = user_list.to_son();
    let from_son = son::from_file::<Value>(VALID_ARRAY_OF_OBJECTS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 7: valid_empty_structures.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Empty {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NestedEmpty {
    inner: Empty,
    arr: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct EmptyStructures {
    empty_object: Empty,
    empty_array: Vec<Value>,
    nested_empty: NestedEmpty,
}

#[test]
fn test_valid_empty_structures() {
    let empty_structures = EmptyStructures {
        empty_object: Empty {},
        empty_array: Vec::new(),
        nested_empty: NestedEmpty {
            inner: Empty {},
            arr: Vec::new(),
        },
    };

    let result = son::from_file::<EmptyStructures>(VALID_EMPTY_STRUCTURES_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), empty_structures);
    let to_son = empty_structures.to_son();
    let from_son = son::from_file::<Value>(VALID_EMPTY_STRUCTURES_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 8: valid_root_array.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Item {
    id: i32,
    name: String,
}

#[test]
fn test_valid_root_array() {
    let root_array = vec![
        Item {
            id: 1,
            name: "Item 1".to_string(),
        },
        Item {
            id: 2,
            name: "Item 2".to_string(),
        },
        Item {
            id: 3,
            name: "Item 3".to_string(),
        },
    ];

    let result = son::from_file::<Vec<Item>>(VALID_ROOT_ARRAY_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), root_array);
    let to_son = root_array.to_son();
    let from_son = son::from_file::<Value>(VALID_ROOT_ARRAY_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 9: valid_string_edge_cases.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct StringEdgeCases {
    empty_string: String,
    with_spaces: String,
    with_quotes: String,
    with_newline: String,
    with_tab: String,
    unicode: String,
}

#[test]
fn test_valid_string_edge_cases() {
    let string_edge_cases = StringEdgeCases {
        empty_string: "".to_string(),
        with_spaces: "hello world".to_string(),
        with_quotes: "He said \"hello\"".to_string(),
        with_newline: "line1\nline2".to_string(),
        with_tab: "tab\there".to_string(),
        unicode: "Hello 世界 🌍".to_string(),
    };

    let result = son::from_file::<StringEdgeCases>(VALID_STRING_EDGE_CASES_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), string_edge_cases);
    let to_son = string_edge_cases.to_son();
    let from_son = son::from_file::<Value>(VALID_STRING_EDGE_CASES_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 10: valid_number_edge_cases.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NumberEdgeCases {
    max_i64: i64,
    min_i64: i64,
    zero: i32,
    negative_zero: i32,
    large_float: f64,
    small_float: f64,
}

#[test]
fn test_valid_number_edge_cases() {
    let number_edge_cases = NumberEdgeCases {
        max_i64: 9223372036854775807,
        min_i64: -9223372036854775808,
        zero: 0,
        negative_zero: 0,
        large_float: 123456.789012,
        small_float: 0.000001,
    };

    let result = son::from_file::<NumberEdgeCases>(VALID_NUMBER_EDGE_CASES_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), number_edge_cases);
    let to_son = number_edge_cases.to_son();
    let from_son = son::from_file::<Value>(VALID_NUMBER_EDGE_CASES_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 11: valid_complex_nested.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Department {
    name: String,
    floor: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Employee {
    id: i32,
    name: String,
    department: Department,
    skills: Vec<String>,
    active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Metadata {
    created: String,
    version: i32,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Company {
    company: String,
    employees: Vec<Employee>,
    metadata: Metadata,
}

#[test]
fn test_valid_complex_nested() {
    let company = Company {
        company: "TechCorp".to_string(),
        employees: vec![
            Employee {
                id: 1,
                name: "Alice".to_string(),
                department: Department {
                    name: "Engineering".to_string(),
                    floor: 3,
                },
                skills: vec!["Rust".to_string(), "Python".to_string(), "Go".to_string()],
                active: true,
            },
            Employee {
                id: 2,
                name: "Bob".to_string(),
                department: Department {
                    name: "Sales".to_string(),
                    floor: 1,
                },
                skills: vec!["Communication".to_string(), "Negotiation".to_string()],
                active: false,
            },
        ],
        metadata: Metadata {
            created: "2024-01-01".to_string(),
            version: 2,
            tags: vec!["production".to_string(), "stable".to_string()],
        },
    };

    let result = son::from_file::<Company>(VALID_COMPLEX_NESTED_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), company);
    let to_son = company.to_son();
    let from_son = son::from_file::<Value>(VALID_COMPLEX_NESTED_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 12: valid_whitespace_variations.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct WhitespaceTest {
    compact: Person,
    spaced: Person,
    tabbed: Person,
}

#[test]
fn test_valid_whitespace_variations() {
    let whitespace_test = WhitespaceTest {
        compact: Person {
            name: "Alice".to_string(),
            age: 30,
        },
        spaced: Person {
            name: "Bob".to_string(),
            age: 25,
        },
        tabbed: Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    };

    let result = son::from_file::<WhitespaceTest>(VALID_WHITESPACE_VARIATIONS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), whitespace_test);
    let to_son = whitespace_test.to_son();
    let from_son = son::from_file::<Value>(VALID_WHITESPACE_VARIATIONS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 13: valid_single_value_object.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SingleValue {
    value: i32,
}

#[test]
fn test_valid_single_value_object() {
    let single_value = SingleValue { value: 42 };

    let result = son::from_file::<SingleValue>(VALID_SINGLE_VALUE_OBJECT_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), single_value);
    let to_son = single_value.to_son();
    let from_son = son::from_file::<Value>(VALID_SINGLE_VALUE_OBJECT_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 14: valid_deeply_nested.son
// ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Level5 {
    deep_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Level4 {
    level5: Level5,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Level3 {
    level4: Level4,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Level2 {
    level3: Level3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Level1 {
    level2: Level2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DeeplyNested {
    level1: Level1,
}

#[test]
fn test_valid_deeply_nested_object() {
    let deeply_nested = DeeplyNested {
        level1: Level1 {
            level2: Level2 {
                level3: Level3 {
                    level4: Level4 {
                        level5: Level5 {
                            deep_value: "found it!".to_string(),
                        },
                    },
                },
            },
        },
    };

    let result = son::from_file::<DeeplyNested>(VALID_DEEPLY_NESTED_OBJECT_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), deeply_nested);
    let to_son = deeply_nested.to_son();
    let from_son = son::from_file::<Value>(VALID_DEEPLY_NESTED_OBJECT_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 15: valid_comment_style.son
// ====================

#[test]
fn test_valid_comment_style() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };

    let result = son::from_file::<User>(VALID_COMMENTS_FILE);
    assert!(result.is_ok(), "{:?}", result);
    assert_eq!(result.unwrap(), user);
    let to_son = user.to_son();
    let from_son = son::from_file::<Value>(VALID_COMMENTS_FILE);
    assert!(from_son.is_ok());
    assert_eq!(from_son.unwrap(), to_son);
}

// ====================
// TEST FILE 11: invalid_missing_brace.son
// ====================
// Missing closing brace - should fail parsing

#[test]
fn test_invalid_missing_brace() {
    let result = son::from_file::<User>(INVALID_MISSING_BRACE_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedEOF(_) => assert!(true),
            _ => panic!("Expected ParseError::UnexpectedEOF but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 16: invalid_missing_bracket.son
// ====================
// Missing closing bracket - should fail parsing

#[test]
fn test_invalid_missing_closing_brace() {
    let result = son::from_file::<Arrays>(INVALID_MISSING_CLOSING_BRACE_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedEOF(_) => assert!(true),
            ParseError::UnexpectedToken { .. } => assert!(true),
            _ => panic!(
                "Expected ParseError::UnexpectedEOF or UnexpectedToken but got {:?}",
                parse_error
            ),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 17: invalid_missing_colon.son
// ====================
// Missing colon after key - should fail parsing

#[test]
fn test_invalid_missing_colon() {
    let result = son::from_file::<User>(INVALID_MISSING_COLON_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedToken { .. } => assert!(true),
            _ => panic!("Expected ParseError::UnexpectedToken but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 18: invalid_missing_value.son
// ====================
// Missing value after colon - should fail parsing

#[test]
fn test_invalid_missing_value() {
    let result = son::from_file::<User>(INVALID_MISSING_VALUE_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedToken { .. } => assert!(true),
            ParseError::UnexpectedEOF(_) => assert!(true),
            _ => panic!(
                "Expected ParseError::UnexpectedToken or UnexpectedEOF but got {:?}",
                parse_error
            ),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 19: invalid_unquoted_string.son
// ====================
// Unquoted string value - should fail parsing

#[test]
fn test_invalid_unquoted_string() {
    let result = son::from_file::<User>(INVALID_UNQUOTED_STRING_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::DeserializationError(deserialization_error) => match deserialization_error {
            DeserializationError::UnexpectedType { expected, found } => {
                assert_eq!(expected, "String");
                assert_eq!(found, "Enum");
            }
            _ => panic!("Expected UnexpectedType error but got {:?}", deserialization_error),
        },
        _ => panic!("Expected DeserializationError but got {:?}", error),
    }
}

// ====================
// TEST FILE 20: invalid_single_quote_string.son
// ====================
// Single quotes for string instead of char - should fail parsing

#[test]
fn test_invalid_single_quote_string() {
    let result = son::from_file::<User>(INVALID_SINGLE_QUOTE_STRING_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::ErrorToken(_, _) => assert!(true),
            _ => panic!("Expected ParseError::ErrorToken but got {:?}", parse_error),
        },
        son::Error::DeserializationError(deserialization_error) => match deserialization_error {
            DeserializationError::UnexpectedType { .. } => assert!(true),
            _ => panic!(
                "Expected DeserializationError::UnexpectedType but got {:?}",
                deserialization_error
            ),
        },
        _ => panic!("Expected ParseError or DeserializationError but got {:?}", error),
    }
}

// ====================
// TEST FILE 21: invalid_double_quote_char.son
// ====================
// Double quotes for char - should fail parsing

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct AgeAndInitial {
    initial: char,
    age: i32,
}

#[test]
fn test_invalid_double_quote_char() {
    let result = son::from_file::<AgeAndInitial>(INVALID_DOUBLE_QUOTE_CHAR_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::DeserializationError(deserialization_error) => match deserialization_error {
            DeserializationError::UnexpectedType { .. } => assert!(true),
            _ => panic!(
                "Expected DeserializationError::UnexpectedType but got {:?}",
                deserialization_error
            ),
        },
        _ => panic!("Expected DeserializationError but got {:?}", error),
    }
}

// ====================
// TEST FILE 22: invalid_multiple_chars.son
// ====================
// Multiple characters in char literal - should fail parsing

#[test]
fn test_invalid_multiple_chars() {
    let result = son::from_file::<AgeAndInitial>(INVALID_MULTIPLE_CHARS_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::ErrorToken(_, _) => assert!(true),
            _ => panic!("Expected ParseError::ErrorToken but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 23: invalid_mixed_types_array.son
// ====================
// Mixed types in array - should fail deserialization

#[test]
fn test_invalid_mixed_types_array() {
    // If your implementation supports Value type arrays, this might succeed
    // But if deserializing to a typed Vec, it should fail
    let result = son::from_file::<Vec<u32>>(INVALID_MIXED_TYPES_ARRAY_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::DeserializationError(deserialization_error) => match deserialization_error {
            DeserializationError::UnexpectedType { .. } => assert!(true),
            DeserializationError::InvalidValue { .. } => assert!(true),
            _ => panic!(
                "Expected DeserializationError::UnexpectedType or InvalidValue but got {:?}",
                deserialization_error
            ),
        },
        _ => panic!("Expected DeserializationError but got {:?}", error),
    }
}

// ====================
// TEST FILE 24: invalid_duplicate_keys.son
// ====================
// Duplicate keys - behavior depends on implementation
// Most implementations would just overwrite, so this might not be an error
// Test as informational

#[test]
fn test_invalid_duplicate_keys() {
    // This test documents the behavior rather than asserting an error
    // HashMap typically overwrites duplicate keys
    let result = son::from_file::<User>(INVALID_DUPLICATE_KEYS_FILE);

    // If your implementation allows duplicates (overwrites), check the final value
    if result.is_ok() {
        let user = result.unwrap();
        // The last occurrence should win
        assert_eq!(user.name, "Bob");
    } else {
        // If your implementation rejects duplicates
        let error = result.err().unwrap();
        match error {
            son::Error::ParseError(parse_error) => match parse_error {
                ParseError::UnexpectedToken { .. } => assert!(true),
                _ => panic!("Expected ParseError::UnexpectedToken but got {:?}", parse_error),
            },
            son::Error::DeserializationError(deserialization_error) => match deserialization_error {
                DeserializationError::Custom(_) => assert!(true),
                _ => panic!(
                    "Expected DeserializationError::Custom but got {:?}",
                    deserialization_error
                ),
            },
            _ => panic!("Expected ParseError or DeserializationError but got {:?}", error),
        }
    }
}

// ====================
// TEST FILE 25: invalid_empty_file.son
// ====================
// Empty file - should fail parsing

#[test]
fn test_invalid_empty_file() {
    let result = son::from_file::<User>(INVALID_EMPTY_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedEOF(_) => assert!(true),
            _ => panic!("Expected ParseError::UnexpectedEOF but got {:?}", parse_error),
        },
        son::Error::IOError(_) => assert!(true), // Empty file might be IO error
        _ => panic!("Expected ParseError or IOError but got {:?}", error),
    }
}

// ====================
// TEST FILE 26: invalid_only_whitespaces.son
// ====================
// Only whitespace - should fail parsing

#[test]
fn test_invalid_only_whitespaces() {
    let result = son::from_file::<User>(INVALID_ONLY_WHITESPACES_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedEOF(_) => assert!(true),
            _ => panic!("Expected ParseError::UnexpectedEOF but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 27: invalid_incorrect_start.son
// ====================
// SON files can only begin with '{' for objects and '[' for arrays - should fail parsing

#[test]
fn test_invalid_incorrect_start() {
    let result = son::from_file::<User>(INVALID_INCORRECT_START_FILE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedToken { step, .. } => assert_eq!(step, ParseStep::Start),
            _ => panic!("Expected ParseError::UnexpectedToken but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 28: invalid_unexpected_eof_value.son
// ====================
// Unexpected EOF when parsing value - should fail parsing

#[test]
fn test_invalid_unexpected_eof_value() {
    let result = son::from_file::<User>(INVALID_UNEXPECTED_EOF_VALUE);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedEOF(_) => assert!(true),
            _ => panic!("Expected ParseError::UnexpectedEOF but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}

// ====================
// TEST FILE 28: invalid_unexpected_token_on_object_identifier.son
// ====================
// Unexpected token when expecting identifier for object - should fail parsing

#[test]
fn test_invalid_unexpected_token_on_object_identifier() {
    let result = son::from_file::<User>(INVALID_UNEXPECTED_TOKEN_ON_OBJECT_IDENTIFIER);
    assert!(result.is_err(), "Expected error but got {:?}", result);
    let error = result.err().unwrap();
    match error {
        son::Error::ParseError(parse_error) => match parse_error {
            ParseError::UnexpectedToken { .. } => assert!(true),
            _ => panic!("Expected ParseError::UnexpectedToken but got {:?}", parse_error),
        },
        _ => panic!("Expected ParseError but got {:?}", error),
    }
}
