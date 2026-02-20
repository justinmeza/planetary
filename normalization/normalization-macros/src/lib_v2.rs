extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Type::Path, TypePath, PathArguments, GenericArgument};

// fn generate_serialization_for_type(
//     field_name: &Option<syn::Ident>,
//     field_type: &TypePath,
// ) -> proc_macro2::TokenStream {
//     if field_type.path.is_ident("i32") || field_type.path.is_ident("bool") {
//         quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
//     } else if field_type.path.is_ident("String") {
//         quote! {
//             format!(
//                 "{}: \"{}\"",
//                 stringify!(#field_name),
//                 self.#field_name.replace("\\", "\\\\").replace(":", "\\:")
//                     .replace("\"", "\\\"").replace(",", "\\,")
//             )
//         }
//     } else {
//         quote! {
//             format!("{}: {}", stringify!(#field_name), self.#field_name.serialize())
//         }
//     }
// }

fn generate_serialization_for_type(
    field_name: &Option<syn::Ident>,
    field_type: &TypePath,
) -> proc_macro2::TokenStream {
    // Check for basic types and their references
    let basic_type_or_ref = |ident: &str|
        field_type.path.is_ident(ident) ||
        (field_type.path.segments.len() == 2 && field_type.path.segments[0].ident == "Ref" && field_type.path.segments[1].ident == ident);

    if basic_type_or_ref("i32") {
        quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    } else if basic_type_or_ref("bool") {
        quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    } else if basic_type_or_ref("String") {
        quote! {
            format!(
                "{}: \"{}\"",
                stringify!(#field_name),
                self.#field_name.replace("\\", "\\\\").replace("\"", "\\\"")
            )
        }
    } else if let Some(segment) = field_type.path.segments.last() {
        if segment.ident == "Vec" {
            // Handle vector serialization
            quote! {
                format!(
                    "{}: [{}]",
                    stringify!(#field_name),
                    self.#field_name.iter().map(|item| format!("{}", item)).collect::<Vec<_>>().join(", ")
                )
            }
        } else {
            // Serialization for custom types with a serialize method
            quote! {
                format!("{}: {}", stringify!(#field_name), self.#field_name.serialize())
            }
        }
    } else {
        // Fallback for unsupported types
        panic!("Unsupported field type!")
    }
}

#[proc_macro_derive(Serializable)]
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => {
            let serialization_logic = named.iter().map(|f| {
                let field_name = &f.ident;
                match &f.ty {
                    Path(type_path) => generate_serialization_for_type(field_name, type_path),
                    _ => panic!("Unsupported type!"),
                }
            });

            quote! {
                impl #name {
                    pub fn serialize(&self) -> String {
                        let parts = vec![#(#serialization_logic),*];
                        format!("{{{}}}", parts.join(","))
                    }
                }
            }
        }
        _ => panic!("Only named structs are supported!"),
    };

    gen.into()
}

// fn generate_field_deserialization(
//     field_name: &Option<Ident>,
//     field_type: &TypePath,
// ) -> proc_macro2::TokenStream {
//     let field_name_str = match field_name {
//         Some(ident) => ident.to_string(),
//         None => panic!("Field has no name"),
//     };
//     let field_str = proc_macro2::Literal::string(&field_name_str);
//     if let Some(ident) = field_type.path.get_ident() {
//         match ident.to_string().as_str() {
//             "i32" | "bool" => {
//                 quote! {
//                     println!("{:?}", map);
//                     println!("{:?}", #field_str);
//                     let #field_name: #field_type = map.get(#field_str)
//                         .ok_or(NormalizationError::MissingField)?
//                         .parse()
//                         .map_err(|_| NormalizationError::ParseFailure)?;
//                 }
//             }
//             "String" => {
//                 quote! {
//                     println!("{:?}", map);
//                     println!("{:?}", #field_str);
//                     let raw_string = map.get(#field_str)
//                         .ok_or(NormalizationError::MissingField)?;
//                     println!("{:?}", raw_string);
//                     let #field_name: String = raw_string.trim_matches('\"').replace("\\:", ":")
//                         .replace("\\\"", "\"").replace("\\\\", "\\").replace("\\,", ",");
//                 }
//             }
//             _ => {
//                 quote! {
//                     println!("{:?}", map);
//                     println!("{:?}", #field_str);
//                     let serialized_struct = map.get(#field_str)
//                         .ok_or(NormalizationError::MissingField)?;
//                     let #field_name = #ident::deserialize(serialized_struct)?;
//                 }
//             }
//             // _ => panic!("Unsupported field type!"),
//         }
//     } else {
//         panic!("Unsupported field type!")
//     }
// }

fn generate_field_deserialization(
    field_name: &Option<Ident>,
    field_type: &TypePath,
) -> proc_macro2::TokenStream {
    let field_name_str = match field_name {
        Some(ident) => ident.to_string(),
        None => panic!("Field has no name"),
    };
    let field_str = proc_macro2::Literal::string(&field_name_str);

    // Check if the type is one of the basic types (i32, bool, String), their references, or a Vec
    if let Some(ident) = field_type.path.get_ident() {
        match ident.to_string().as_str() {
            "i32" | "bool" => {
                quote! {
                    let #field_name: #field_type = map.get(#field_str)
                        .ok_or(NormalizationError::MissingField)?
                        .parse()
                        .map_err(|_| NormalizationError::ParseFailure)?;
                }
            }
            "String" => {
                quote! {
                    let raw_string = map.get(#field_str)
                        .ok_or(NormalizationError::MissingField)?;
                    let #field_name: String = raw_string.trim_matches('\"').replace("\\:", ":")
                        .replace("\\\"", "\"").replace("\\\\", "\\").replace("\\,", ",");
                }
            }
            // Add cases for references to basic types if needed
            _ => {
                quote! {
                    let serialized_struct = map.get(#field_str)
                        .ok_or(NormalizationError::MissingField)?;
                    let #field_name = #ident::deserialize(serialized_struct)?;
                }
            }
        }
    } else if let Some(segment) = field_type.path.segments.last() {
        if segment.ident == "Vec" {
            // Handling for Vec types
            if let PathArguments::AngleBracketed(inner_args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_type)) = inner_args.args.first() {
                    quote! {
                        println!("{:?}", map);
                        let serialized_vec = map.get(#field_str)
                            .ok_or(NormalizationError::MissingField)?
                            .trim_start_matches('[') // Remove leading '['
                            .trim_end_matches(']');  // Remove trailing ']'

                        // Splitting by ',' and then trimming each element to handle spaces
                        let #field_name: Vec<#inner_type> = serialized_vec
                            .split(',')              // Split by ','
                            .map(|s| s.trim())       // Trim spaces around elements
                            .map(|element| element.parse::<#inner_type>()
                                .map_err(|_| NormalizationError::ParseFailure))
                            .collect::<Result<Vec<_>, _>>()?;
                        }
                } else {
                    panic!("Vec type without inner type")
                }
            } else {
                panic!("Unsupported field type!")
            }
        } else {
            panic!("Unsupported field type!")
        }
    } else {
        panic!("Unsupported field type!")
    }
}

#[proc_macro_derive(Deserializable)]
pub fn derive_deserializable(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let deserialize_fields = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                match &f.ty {
                    Path(type_path) => generate_field_deserialization(field_name, type_path),
                    _ => panic!("Unsupported type!"),
                }
            });

            let field_names = fields.named.iter().map(|f| &f.ident);

            let expanded = quote! {
                impl #name {
                    pub fn deserialize(input: &str) -> Result<Self, NormalizationError> {
                        let trimmed = input.trim_matches(|c| c == '{' || c == '}');
                        let mut parts: Vec<&str> = Vec::new();
                        let mut start = 0;
                        let mut escaped = false;
                        let mut in_vector = false;

                        for (i, char) in trimmed.chars().enumerate() {
                            if escaped {
                                escaped = false;
                                continue;
                            }

                            match char {
                                '\\' => escaped = true,
                                '[' => in_vector = true,
                                ']' => in_vector = false,
                                ',' if !in_vector => {
                                    parts.push(&trimmed[start..i]);
                                    start = i + 1;
                                }
                                _ => (),
                            }
                        }
                        parts.push(&trimmed[start..]);

                        let mut map = std::collections::HashMap::new();
                        for part in parts.iter() {
                            let kv: Vec<&str> = part.splitn(2, ':').map(str::trim).collect();
                            if kv.len() == 2 {
                                map.insert(kv[0], kv[1]);
                            }
                        }

                        #(#deserialize_fields)*
                        Ok(Self { #(#field_names),* })
                    }
                }
            };

            expanded.into()
        }
        _ => panic!("Only named structs are supported!"),
    }
}

// #[proc_macro_derive(Deserializable)]
// pub fn derive_deserializable(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;

//     match &input.data {
//         Data::Struct(DataStruct {
//             fields: Fields::Named(fields),
//             ..
//         }) => {
//             let deserialize_fields = fields.named.iter().map(|f| {
//                 let field_name = &f.ident;
//                 match &f.ty {
//                     Path(type_path) => generate_field_deserialization(field_name, type_path),
//                     _ => panic!("Unsupported type!"),
//                 }
//             });

//             let field_names = fields.named.iter().map(|f| &f.ident);

//             let expanded = quote! {
//                 impl #name {
//                     pub fn deserialize(input: &str) -> Result<Self, NormalizationError> {
//                         let trimmed = input.trim_matches(|c| c == '{' || c == '}');
//                         let mut parts: Vec<&str> = Vec::new();
//                         let mut start = 0;
//                         let mut escaped = false;
//                         let mut nesting = 0;

//                         for (i, char) in trimmed.chars().enumerate() {
//                             if escaped {
//                                 escaped = false;
//                                 continue;
//                             }

//                             match char {
//                                 '\\' => escaped = true,
//                                 '{' if !escaped => nesting += 1,
//                                 '}' if !escaped => nesting -= 1,
//                                 ',' if !escaped && nesting == 0 => {
//                                     parts.push(&trimmed[start..i]);
//                                     start = i + 1;
//                                 }
//                                 _ => (),
//                             }
//                         }
//                         parts.push(&trimmed[start..]);

//                         let mut map = std::collections::HashMap::new();
//                         for part in parts.iter() {
//                             let kv: Vec<&str> = part.splitn(2, ':').map(str::trim).collect();
//                             if kv.len() == 2 {
//                                 map.insert(kv[0], kv[1]);
//                             }
//                         }
//                         println!("{:?}", map);

//                         #(#deserialize_fields)*
//                         Ok(Self { #(#field_names),* })
//                     }
//                 }
//             };

//             expanded.into()
//         }
//         _ => panic!("Only named structs are supported!"),
//     }
// }
