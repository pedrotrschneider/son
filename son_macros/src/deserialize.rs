use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let from_son_impl = match &input.data {
        Data::Struct(data) => {
            let fields = match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => panic!("Deserialize can only be derived for structs with named fields"),
            };

            let field_deserializers = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();
                let field_type = &f.ty;
                let conversion_logic = generate_conversion_logic(field_type, quote!(field_value));

                quote! {
                    let #field_name = {
                        let field_value = map.remove(#field_name_str)
                            .ok_or_else(|| DeserializationError::MissingField { field: #field_name_str.to_string() })?;
                        #conversion_logic?
                    };
                }
            });

            let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());

            quote! {
                let mut map = if let Value::Object(map) = son {
                    map
                } else {
                    return Err(DeserializationError::UnexpectedType {
                        expected: "Object".to_string(),
                        found: son.get_type().to_string(),
                    });
                };

                #(#field_deserializers)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
        Data::Enum(data) => {
            let deserialize_arms = data.variants.iter().map(|v| {
                let variant_ident = &v.ident;
                let variant_name_str = variant_ident.to_string();

                match &v.fields {
                    Fields::Unit => {
                        quote! {
                            #variant_name_str => Ok(Self::#variant_ident)
                        }
                    }
                    _ => {
                        panic!("Deserialize derive for enums currently only supports unit variants.");
                    }
                }
            });

            quote! {
                if let Value::Enum(s) = son {
                    match s.as_str() {
                        #(#deserialize_arms,)*
                        _ => Err(DeserializationError::UnknownVariant { variant: s.to_string(), enum_name: stringify!(#name).to_string() })
                    }
                } else {
                    Err(DeserializationError::UnexpectedType {
                        expected: "Enum".to_string(),
                        found: son.get_type().to_string(),
                    })
                }
            }
        }
        Data::Union(_) => panic!("Deserialize can only be derived for structs and enums"),
    };

    let expanded = quote! {
        impl FromSon for #name {
            fn from_son(son: Value) -> Result<Self, DeserializationError> {
                #from_son_impl
            }
        }

        impl Deserialize for #name {}
    };

    TokenStream::from(expanded)
}

/// A helper function to generate the token stream for converting a Value
/// into a specific Rust type. (This function is unchanged)
fn generate_conversion_logic(ty: &Type, value_accessor: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // Helper to get type identifier string (e.g., "String", "i32", "Vec")
    let get_type_ident_str = |t: &Type| -> Option<String> {
        if let Type::Path(type_path) = t {
            type_path.path.segments.last().map(|s| s.ident.to_string())
        } else {
            None
        }
    };

    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let inner_logic = generate_conversion_logic(inner_ty, quote!(v));
                        return quote! {
                            match #value_accessor {
                                Value::Null => Ok(None),
                                v => #inner_logic.map(Some),
                            }
                        };
                    }
                }
            }
        }
    }

    let type_ident_str = get_type_ident_str(ty).unwrap_or_default();

    return match type_ident_str.as_str() {
        "String" => quote! {
            if let Value::String(v) = #value_accessor { Ok(v) } else { Err(DeserializationError::UnexpectedType { expected: "String".to_string(), found: #value_accessor.get_type().to_string() }) }
        },
        "char" => quote! {
            if let Value::Char(v) = #value_accessor { Ok(v) } else { Err(DeserializationError::UnexpectedType { expected: "Char".to_string(), found: #value_accessor.get_type().to_string() }) }
        },
        "bool" => quote! {
            if let Value::Bool(v) = #value_accessor { Ok(v) } else { Err(DeserializationError::UnexpectedType { expected: "Bool".to_string(), found: #value_accessor.get_type().to_string() }) }
        },
        "f32" | "f64" => quote! {
            if let Value::Float(v) = #value_accessor { Ok(v as #ty) } else { Err(DeserializationError::UnexpectedType { expected: "Float".to_string(), found: #value_accessor.get_type().to_string() }) }
        },
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => quote! {
            if let Value::Integer(v) = #value_accessor { Ok(v as #ty) } else { Err(DeserializationError::UnexpectedType { expected: "Integer".to_string(), found: #value_accessor.get_type().to_string() }) }
        },
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => quote! {
            if let Value::Integer(v) = #value_accessor {
                if v < 0 { Err(DeserializationError::InvalidValue{ message: "Cannot assign a negative integer to an unsigned type".to_string() }) } else { Ok(v as #ty) }
            } else { Err(DeserializationError::UnexpectedType { expected: "Integer".to_string(), found: #value_accessor.get_type().to_string() }) }
        },
        "Vec" => {
            let inner_ty = if let Type::Path(type_path) = ty {
                if let PathArguments::AngleBracketed(args) = &type_path.path.segments.last().unwrap().arguments {
                    if let Some(GenericArgument::Type(inner)) = args.args.first() {
                        Some(inner)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(inner_ty) = inner_ty {
                let inner_conversion = generate_conversion_logic(inner_ty, quote!(item));
                quote! {
                    if let Value::Array(arr) = #value_accessor {
                        arr.into_iter()
                           .map(|item| #inner_conversion)
                           .collect::<Result<Vec<_>, _>>()
                    } else {
                        Err(DeserializationError::UnexpectedType { expected: "Array".to_string(), found: #value_accessor.get_type().to_string() })
                    }
                }
            } else {
                quote! { Err("Unsupported Vec type".to_string()) }
            }
        }
        _ => quote! {
            #ty::from_son(#value_accessor).map_err(|e| e.into())
        },
    };
}
