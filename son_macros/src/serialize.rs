use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Serialize can only be derived for structs with named fields"),
        },
        _ => panic!("Serialize can only be derived for structs"),
    };

    // Generate the code to insert each field into the HashMap
    let field_serializers = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        // The logic for converting a field's value to a SonValue is simply
        // to call `.to_son()` on it. We rely on the field's type
        // also implementing the `ToSon` trait. This works for primitives,
        // other structs with `#[derive(Serialize)]`, and Vec<T>.
        quote! {
            map.insert(#field_name_str.to_string(), self.#field_name.to_son());
        }
    });

    let expanded = quote! {
        // First, implement the core `ToSon` logic for the struct.
        impl ToSon for #name {
            fn to_son(&self) -> SonValue {
                let mut map = std::collections::HashMap::new();

                // Insert all the fields into the map.
                #(#field_serializers)*

                SonValue::Object(map)
            }
        }

        // Then, implement the marker trait `Serialize`.
        impl Serialize for #name {}
    };

    TokenStream::from(expanded)
}
