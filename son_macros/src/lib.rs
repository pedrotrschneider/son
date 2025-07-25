use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[proc_macro_derive(Deserialize)]
pub fn set_field_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let name = &input.ident;

    // Get the fields of the struct
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Deserialize can only be derived for structs with named fields"),
        },
        _ => panic!("Deserialize can only be derived for structs"),
    };

    // Generate the match arms for the set_field method
    let match_arms = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &f.ty;

        // Helper function to get the inner type of a Vec<T>
        let get_vec_inner_type = |ty: &Type| -> Option<&Type> {
            if let Type::Path(type_path) = ty {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                return Some(inner_ty);
                            }
                        }
                    }
                }
            }
            None
        };

        if quote!(#field_type).to_string() == quote!(i8).to_string()
            || quote!(#field_type).to_string() == quote!(i16).to_string()
            || quote!(#field_type).to_string() == quote!(i32).to_string()
            || quote!(#field_type).to_string() == quote!(i64).to_string()
            || quote!(#field_type).to_string() == quote!(i128).to_string()
            || quote!(#field_type).to_string() == quote!(isize).to_string()
        {
            quote! {
                #field_name_str => if let SonValue::Integer(v) = value {
                    self.#field_name = v as #field_type;
                    Ok(())
                } else {
                    Err("Invalid type for field".to_string())
                }
            }
        } else if quote!(#field_type).to_string() == quote!(u8).to_string()
            || quote!(#field_type).to_string() == quote!(u16).to_string()
            || quote!(#field_type).to_string() == quote!(u32).to_string()
            || quote!(#field_type).to_string() == quote!(u64).to_string()
            || quote!(#field_type).to_string() == quote!(u128).to_string()
            || quote!(#field_type).to_string() == quote!(usize).to_string()
        {
            quote! {
                #field_name_str => if let SonValue::Integer(v) = value {
                    if v < 0 {
                        Err("Expected positive value".to_string())
                    } else {
                        self.#field_name = v as #field_type;
                        Ok(())
                    }
                } else {
                    Err("Invalid type for field".to_string())
                }
            }
        } else if quote!(#field_type).to_string() == quote!(f32).to_string()
            || quote!(#field_type).to_string() == quote!(f64).to_string()
        {
            quote! {
                #field_name_str => if let SonValue::Float(v) = value {
                    self.#field_name = v as #field_type;
                    Ok(())
                } else {
                    Err("Invalid type for field".to_string())
                }
            }
        } else if quote!(#field_type).to_string() == quote!(bool).to_string() {
            quote! {
                #field_name_str => if let SonValue::Bool(v) = value {
                    self.#field_name = v;
                    Ok(())
                } else {
                    Err("Invalid type for field".to_string())
                }
            }
        } else if quote!(#field_type).to_string() == quote!(String).to_string() {
            quote! {
                #field_name_str => if let SonValue::String(v) = value {
                    self.#field_name = v;
                    Ok(())
                } else {
                    Err("Invalid type for field".to_string())
                }
            }
        } else if quote!(#field_type).to_string() == quote!(char).to_string() {
            quote! {
                #field_name_str => if let SonValue::Char(v) = value {
                    self.#field_name = v;
                    Ok(())
                } else {
                    Err("Invalid type for field".to_string())
                }
            }
        } else {
            quote! {
                #field_name_str => {
                    if path.len() == 1 {
                        Err("Invalid field".to_string())
                    } else {
                        self.#field_name.set_field(&path[1..], value)
                    }
                }
            }
        }
    });

    let expanded = quote! {
        impl Deserialize for #name {
            fn set_field(&mut self, path: &[&str], value: SonValue) -> Result<(), String> {
                if path.is_empty() {
                    return Err("Path cannot be empty".to_string());
                }

                return match path[0] {
                    #(#match_arms)*
                    _ => Err("Field not found".to_string()),
                    // _ => Err(format!("Field '{}' not found on struct '{}'", path[0], stringify!(#name))),
                }
            }
        }
    };

    return TokenStream::from(expanded);
}
