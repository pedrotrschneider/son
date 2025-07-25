mod lexer;
mod serialization;
mod token;
mod util;

use crate::lexer::Lexer;
use crate::serialization::Deserialize;
use crate::serialization::SonValue;
use crate::token::TokenType;
use son_macros::Deserialize;

struct Deserializer {}

impl Deserializer {
    pub fn from_file_to<T>(file_path: &str) -> T
    where
        T: Deserialize,
    {
        let mut value = T::default();

        let lexer = Lexer::new(file_path);

        let mut field_path: Vec<String> = Vec::new();
        let mut negative_flag = false;
        for token in lexer {
            match token.get_type() {
                TokenType::LeftParen => {}
                TokenType::RightParen => {
                    field_path.pop();
                }
                TokenType::LeftBrace => {}
                TokenType::RightBrace => {}
                TokenType::LeftBracket => {}
                TokenType::RightBracket => {}
                TokenType::Comma => {}
                TokenType::Dot => {}
                TokenType::Colon => {}
                TokenType::Null => {}
                TokenType::Negative => negative_flag = true,
                TokenType::True
                | TokenType::False
                | TokenType::IntegerLiteral
                | TokenType::FloatLiteral
                | TokenType::StringLiteral
                | TokenType::CharLiteral => {
                    let path = field_path.iter().map(|x| &**x).collect::<Vec<&str>>();
                    let son_value = token.get_value().unwrap();
                    let son_value = if negative_flag { son_value.negate() } else { son_value };
                    let _ = value.set_field(&path, son_value);
                    field_path.pop();
                }
                TokenType::Identifier => field_path.push(token.get_source()),
                TokenType::Error => {}
                TokenType::EOF => {}
            }
        }

        return value;
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct Address {
    street_address: String,
    city: String,
    state: String,
    postal_code: String,
    latitude: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct PhoneNumber {
    ty: String,
    number: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
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
    // let mut lexer = Lexer::new("src/test.son");
    // loop {
    //     let token = lexer.next_token();
    //     match token.get_type() {
    //         TokenType::EOF => break,
    //         _ => println!("token: {:?}", token),
    //     }
    // }

    // let mut client = Client {
    //     first_name: "Pedro".to_owned(),
    //     last_name: "Schneider".to_owned(),
    //     age: 23,
    //     is_alive: true,
    //     is_retired: false,
    //     initial: 'p',
    //     address: Address {
    //         street_address: "Rua Leopoldo Couto Magalhaes Junior".to_owned(),
    //         city: "Sao Paulo".to_owned(),
    //         state: "Sao Paulo".to_owned(),
    //         postal_code: "04542-001".to_owned(),
    //         latitude: 3.14,
    //     },
    // };
    //
    // println!("{:#?}", client);
    //
    // client
    //     .set_field(
    //         &["address", "street_address"],
    //         SonValue::String("Espirito Santo".to_string()),
    //     )
    //     .unwrap();
    //
    // println!("{:#?}", client);

    // println!("{:#?}", Deserializer::from_file_to::<Client>("src/test.son"));
}
