use proc_macro::TokenStream;

mod deserialize;
mod serialize;

#[proc_macro_derive(Deserialize)]
pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    return deserialize::deserialize_derive(input);
}

#[proc_macro_derive(Serialize)]
pub fn serialize_derive(input: TokenStream) -> TokenStream {
    return serialize::serialize_derive(input);
}
