mod lexer;
mod parser;
mod serialization;
mod token;
mod util;

use crate::parser::Parser;
use crate::serialization::{Deserialize, FromSon, Serialize, SonPrinter, SonValue, ToSon};
use son_macros::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct Address {
    street_address: String,
    city: String,
    state: String,
    postal_code: String,
    latitude: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct PhoneNumber {
    ty: String,
    number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    let input_son = Parser::from_file_to_son_object("src/test.son");
    let client = Client::from_son(input_son).unwrap();
    let output_son = client.to_son();
    println!();

    let printer = SonPrinter::new(String::from("...."));
    println!("{}", printer.son_to_string(&output_son));
}
