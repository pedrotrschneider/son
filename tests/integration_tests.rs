use son::{DeserializationError, Deserialize, FromSon, Serialize, ToSon, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PhoneType {
    Home,
    Office,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Address {
    street_address: String,
    city: String,
    state: String,
    postal_code: String,
    latitude: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PhoneNumber {
    ty: PhoneType,
    number: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
fn test() {
    let result = son::from_file::<Value>("src/test.son");
    assert!(result.is_ok(), "{}", result.err().unwrap());
    println!("{}", result.unwrap().to_string());
    assert!(false);
}
