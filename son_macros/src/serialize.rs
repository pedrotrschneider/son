use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // The logic will now depend on whether we are deriving for a struct or an enum.
    let to_son_impl = match &input.data {
        Data::Struct(data) => {
            let fields = match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Serialize can only be derived for structs with named fields"),
            };

            // Generate the code to insert each field into the HashMap
            let field_serializers = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();
                quote! {
                    map.insert(#field_name_str.to_string(), self.#field_name.to_son());
                }
            });

            quote! {
                let mut map = std::collections::HashMap::new();
                #(#field_serializers)*
                return Value::Object(map);
            }
        }
        Data::Enum(data) => {
            // Generate match arms for each enum variant
            let serialize_arms = data.variants.iter().map(|v| {
                let variant_ident = &v.ident;
                let variant_name_str = variant_ident.to_string();

                // For now, we only support simple "unit" variants (e.g., `MyEnum::Variant`).
                // Variants with data like `MyEnum::Variant(i32)` are not yet supported.
                match &v.fields {
                    Fields::Unit => {
                        quote! {
                            Self::#variant_ident => Value::Enum(#variant_name_str.to_string())
                        }
                    }
                    _ => {
                        panic!("Serialize derive for enums currently only supports unit variants.");
                    }
                }
            });

            quote! {
                match self {
                    #(#serialize_arms),*
                }
            }
        }
        Data::Union(_) => panic!("Serialize can only be derived for structs and enums"),
    };

    let expanded = quote! {
        // Implement the core `ToSon` logic for the struct or enum.
        impl ToSon for #name {
            fn to_son(&self) -> Value {
                #to_son_impl
            }
        }

        // Then, implement the marker trait `Serialize`.
        impl Serialize for #name {}
    };

    TokenStream::from(expanded)
}
