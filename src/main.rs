mod error;
mod lexer;
mod parser;
mod serialization;
mod token;
mod util;

use crate::error::DeserializationError;
use crate::parser::SonParser;
use crate::serialization::{Deserialize, FromSon, Serialize, SonPrinter, SonValue, ToSon};
use son_macros::{Deserialize, Serialize};

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

fn main() {
    let input_son = SonParser::from_file_to_son_object("src/test.son").unwrap();
    let client = Client::from_son(input_son).unwrap();
    println!("{:#?}", client);
}
