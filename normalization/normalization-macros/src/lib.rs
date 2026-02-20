extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Type::Path, TypePath, PathArguments, GenericArgument, Type};

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

// fn generate_serialization_for_type(
//     field_name: &Option<syn::Ident>,
//     field_type: &TypePath,
// ) -> proc_macro2::TokenStream {
//     // Check for basic types and their references
//     let basic_type_or_ref = |ident: &str|
//         field_type.path.is_ident(ident) ||
//         (field_type.path.segments.len() == 2 && field_type.path.segments[0].ident == "Ref" && field_type.path.segments[1].ident == ident);

//     if basic_type_or_ref("i32") {
//         quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
//     } else if basic_type_or_ref("bool") {
//         quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
//     } else if basic_type_or_ref("String") {
//         quote! {
//             format!(
//                 "{}: \"{}\"",
//                 stringify!(#field_name),
//                 self.#field_name.replace("\\", "\\\\").replace("\"", "\\\"")
//             )
//         }
//     } else if let Some(segment) = field_type.path.segments.last() {
//         if segment.ident == "Vec" {
//             // Handle vector serialization
//             quote! {
//                 format!(
//                     "{}: [{}]",
//                     stringify!(#field_name),
//                     self.#field_name.iter().map(|item| format!("{}", item)).collect::<Vec<_>>().join(", ")
//                 )
//             }
//         } else {
//             // Serialization for custom types with a serialize method
//             quote! {
//                 format!("{}: {}", stringify!(#field_name), self.#field_name.serialize())
//             }
//         }
//     } else {
//         // Fallback for unsupported types
//         panic!("Unsupported field type!")
//     }
// }

// use proc_macro2::TokenStream as TokenStream2;
// use quote::quote;
// use syn::{Ident, TypePath, Type};

// Function to check if a type is a basic type like i32, bool, or String
// fn is_basic_type(ty: &Type) -> bool {
//     match ty {
//         Type::Path(type_path) => {
//             matches!(type_path.path.get_ident().map(|i| i.to_string().as_str()), Some("i32" | "bool" | "String"))
//         }
//         _ => false,
//     }
// }

fn generate_serialization_for_type(
    field_name: &Option<Ident>,
    field_type: &TypePath,
) -> proc_macro2::TokenStream {
    let field_name_str = field_name.as_ref().unwrap().to_string();
    let field_str = quote::format_ident!("{}", field_name_str);

    // if is_basic_type(&Type::Path(field_type.clone())) {
    //     // Basic type or reference to a basic type: use standard formatting
    //     quote! {
    //         format!("{}: {}", stringify!(#field_str), &self.#field_str)
    //     }
    if field_type.path.is_ident("i32") || field_type.path.is_ident("u32") || field_type.path.is_ident("u64") || field_type.path.is_ident("usize") || field_type.path.is_ident("bool") {
        quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    } else if field_type.path.is_ident("String") {
        quote! {
            format!(
                "{}: \"{}\"",
                stringify!(#field_name),
                self.#field_name.replace("\\", "\\\\").replace(":", "\\:")
                    .replace("\"", "\\\"").replace(",", "\\,")
            )
        }
    } else if let Some(segment) = field_type.path.segments.last() {
        if segment.ident == "Vec" {
            // Vector of types
            if let PathArguments::AngleBracketed(inner_args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_path))) = inner_args.args.first() {
                    // if is_basic_type(&Type::Path(inner_path.clone())) {
                    //     // Serialize vector of basic types
                    //     quote! {
                    //         format!(
                    //             "{}: [{}]",
                    //             stringify!(#field_str),
                    //             self.#field_str.iter().map(|item| format!("{}", item)).collect::<Vec<_>>().join(", ")
                    //         )
                    //     }
                    if inner_path.path.is_ident("i32") || inner_path.path.is_ident("u32") || inner_path.path.is_ident("u64") || inner_path.path.is_ident("usize") || inner_path.path.is_ident("bool") {
                        quote! {
                            format!(
                                "{}: [{}]",
                                stringify!(#field_str),
                                self.#field_str.iter().map(|item| format!("{}", item)).collect::<Vec<_>>().join(",")
                            )
                        }
                    } else if inner_path.path.is_ident("String") {
                        quote! {
                            format!(
                                // "{}: \"{}\"",
                                "{}: [{}]",
                                stringify!(#field_str),
                                self.#field_str.iter().map(|item| format!("\"{}\"", item
                                        .replace("\\", "\\\\").replace(":", "\\:")
                                        .replace("\"", "\\\"").replace(",", "\\,"))).collect::<Vec<_>>().join(",")
                            )
                        }
                    } else {
                        // Serialize vector of custom types
                        quote! {
                            format!(
                                "{}: [{}]",
                                stringify!(#field_str),
                                self.#field_str.iter().map(|item| item.serialize()).collect::<Vec<_>>().join(",")
                            )
                        }
                    }
                } else {
                    quote! {
                        format!("{}: {}", stringify!(#field_str), "Unsupported vector type")
                    }
                }
            } else {
                quote! {
                    format!("{}: {}", stringify!(#field_str), "Unsupported vector type")
                }
            }
        } else {
            // Custom type: use the serialize method
            quote! {
                format!("{}: {}", stringify!(#field_str), self.#field_str.serialize())
            }
        }
    } else {
        // Fallback for unsupported types
        quote! {
            format!("{}: {}", stringify!(#field_str), "Unsupported type")
        }
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

// Function to check if a type is a basic type like i32, bool, or String
fn is_basic_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "i32" | "u32" | "u64" | "usize" | "bool" | "String" => true,
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

// Main function to generate field deserialization logic
fn generate_field_deserialization(
    field_name: &Option<Ident>,
    field_type: &TypePath,
) -> proc_macro2::TokenStream {
    let field_name_str = match field_name {
        Some(ident) => ident.to_string(),
        None => panic!("Field has no name"),
    };
    let field_str = quote::format_ident!("{}", field_name_str);

    if is_basic_type(&Type::Path(field_type.clone())) {
        // Basic type handling
        match field_type.path.segments.last().unwrap().ident.to_string().as_str() {
            "i32" | "u32" | "u64" | "usize" | "bool" => {
                quote! {
                    println!("1: {:?}", map);
                    println!("2: {:?}", #field_name_str);
                    let #field_str: #field_type = map.get(#field_name_str)
                        .ok_or(NormalizationError::MissingField)?
                        .parse()
                        .map_err(|_| NormalizationError::ParseFailure)?;
                }
            }
            "String" => {
                // TODO:  The string (second element) is not showing up in the map for vectors of structs.
                quote! {
                    println!("3: {:?}", map);
                    println!("4: {:?}", #field_name_str);
                    let raw_string = map.get(#field_name_str)
                        .ok_or(NormalizationError::MissingField)?;
                    println!("5: {:?}", raw_string);
                    let #field_str: String = raw_string.trim_matches('\"').replace("\\:", ":")
                        .replace("\\\"", "\"").replace("\\\\", "\\").replace("\\,", ",");
                }
                // quote! {
                //     let #field_str = map.get(#field_name_str)
                //         .ok_or(NormalizationError::MissingField)?
                //         .to_string();
                // }
            }
            _ => unreachable!("Unsupported type!"),
        }
    } else if let Some(segment) = field_type.path.segments.last() {
        if segment.ident == "Vec" {
            if let PathArguments::AngleBracketed(inner_args) = &segment.arguments {
                if let Some(GenericArgument::Type(inner_type)) = inner_args.args.first() {
                    let inner_type_path = if let Type::Path(type_path) = inner_type {
                        type_path
                    } else {
                        panic!("Unsupported inner type in Vec");
                    };

                    let deserialize_vector = if is_basic_type(inner_type) {
                        match inner_type_path.path.segments.last().unwrap().ident.to_string().as_str() {
                            "i32" | "u32" | "u64" | "usize" | "bool" => {
                                // Direct parsing for i32, u32, usize, and bool
                                quote! {
                                    println!("6: Serialized vec: {:?}", serialized_vec);
                                    let elements = serialized_vec
                                        .split(',')
                                        .map(|s| s.trim().parse::<#inner_type_path>()
                                            .map_err(|_| NormalizationError::ParseFailure))
                                        .collect::<Result<Vec<_>, _>>()?;
                                    }
                            }
                            "String" => {
                                // Special handling for String to correctly unescape characters
                                quote! {
                                    println!("7: Serialized vec: {:?}", serialized_vec);
                                    let serialized_vec = map.get(#field_name_str)
                                        .ok_or(NormalizationError::MissingField)?
                                        .trim_start_matches('[')
                                        .trim_end_matches(']');
                                    println!("8: {:?}", serialized_vec);

                                    let mut in_quotes = false;
                                    let mut escape_next_char = false;
                                    let mut element_start = 0;
                                    let mut chars = serialized_vec.chars().enumerate();
                                    println!("9: {:?}", chars);

                                    println!("10: {:?}", serialized_vec.split(','));
                                    let elements = serialized_vec
                                        .split(',')
                                        .map(|s| s.trim_matches('\"').replace("\\:", ":")
                                            .replace("\\\"", "\"").replace("\\\\", "\\").replace("\\,", ","))
                                        .collect::<Vec<String>>();
                                    println!("11: Elements: {:?}", elements);

                                    // let #field_str: Vec<String> = chars.by_ref()
                                    //     .filter_map(|(i, c)| {
                                    //         if escape_next_char {
                                    //             escape_next_char = false;
                                    //             None
                                    //         } else {
                                    //             match c {
                                    //                 '\\' => {
                                    //                     escape_next_char = true;
                                    //                     None
                                    //                 }
                                    //                 '"' => {
                                    //                     in_quotes = !in_quotes;
                                    //                     None
                                    //                 }
                                    //                 ',' if !in_quotes => {
                                    //                     let element = serialized_vec[element_start..i].trim().to_string();
                                    //                     element_start = i + 1;
                                    //                     Some(element)
                                    //                 }
                                    //                 _ => None,
                                    //             }
                                    //         }
                                    //     })
                                    // .chain(std::iter::once(serialized_vec[element_start..].trim().to_string()))
                                    //     .collect();
                                    }
                                // quote! {
                                //     let elements = serialized_vec
                                //         .split(',')
                                //         .map(|s| s.trim_matches('\"').replace("\\:", ":")
                                //             .replace("\\\"", "\"").replace("\\\\", "\\").replace("\\,", ","))
                                //         .collect::<Vec<String>>();
                                //     }
                            }
                            _ => unreachable!("Unsupported type!"),
                        }

                        // // Deserialize vector of basic types
                        // quote! {
                        //     println!("{:?}", map);
                        //     println!("{:?}", #field_name_str);
                        //     let elements = serialized_vec
                        //         .split(',')
                        //         .map(|s| s.parse::<#inner_type_path>()
                        //             .map_err(|_| NormalizationError::ParseFailure))
                        //         .collect::<Result<Vec<_>, _>>()?;
                        // }
                    } else {
                        // Deserialize vector of custom structs
                        quote! {
                            println!("12: Serialized vec: {:?}", serialized_vec);
                            let mut start = 0;
                            let mut nesting_level = 0;
                            let mut is_struct_start = false;
                            let mut struct_strs = Vec::new();
                            let chars = serialized_vec.chars().enumerate();

                            for (i, c) in chars {
                                match c {
                                    '{' => {
                                        if nesting_level == 0 {
                                            start = i; // Start of a new struct
                                            is_struct_start = true;
                                        }
                                        nesting_level += 1;
                                    }
                                    '}' => {
                                        nesting_level -= 1;
                                        if nesting_level == 0 && is_struct_start {
                                            // End of a struct
                                            is_struct_start = false;
                                            struct_strs.push(&serialized_vec[start..=i]);
                                        }
                                    }
                                    _ => (),
                                }
                            }

                            println!("Struct strs: {:?}", struct_strs);
                            // let #field_str: Vec<#inner_type_path> = struct_strs.into_iter()
                            //     .filter_map(|struct_str| {
                            //         match #inner_type_path::deserialize(struct_str) {
                            //             Ok(struct_data) => Some(struct_data),
                            //             Err(_) => None,
                            //         }
                            //     })
                            // .collect();
                            println!("Struct data: {:?}", struct_strs.clone().into_iter()
                                .filter_map(|struct_str| {
                                    match #inner_type_path::deserialize(&struct_str) {
                                        Ok(struct_data) => Some(struct_data),
                                        Err(_) => None,
                                    }
                                }));
                            let elements = struct_strs.clone()
                                .into_iter()
                                .map(|s| #inner_type_path::deserialize(s))
                                .collect::<Result<Vec<_>, _>>()?;
                            // TODO:  Need to get from struct_strs to calling deserialize on each
                            // of them and returning a vector of the correct struct type.
                            // let elements = serialized_vec
                            //     .split(',')
                            //     .map(|s| #inner_type_path::deserialize(s))
                            //     .collect::<Result<Vec<_>, _>>()?;
                        // quote! {
                        //     println!("12: Serialized vec: {:?}", serialized_vec);
                        //     // TODO:  There is a problem deserializing here.
                        //     println!("12.2:  Split elements: {:?}", serialized_vec.trim_start_matches('{').trim_end_matches('}').split(','));
                        //     let elements = serialized_vec
                        //         .trim_start_matches('{')
                        //         .trim_end_matches('}')
                        //         .split(',')
                        //         .map(|s| #inner_type_path::deserialize(s))
                        //         .collect::<Result<Vec<_>, _>>()?;
                        //     println!("12.5:  Elements: {:?}", elements);
                        }
                    };
                    // println!("Deserialize vector: {:?}", deserialize_vector);
                    quote! {
                        let serialized_vec = map.get(#field_name_str)
                            .ok_or(NormalizationError::MissingField)?
                            .trim_start_matches('[')
                            .trim_end_matches(']');
                        println!("13: Serialized vec: {:?}", serialized_vec);
                        #deserialize_vector
                        let #field_str: Vec<#inner_type_path> = elements;
                    }
                } else {
                    panic!("Vec type without inner type");
                }
            } else {
                panic!("Unsupported field type!");
            }
        } else {
            // Handling for non-Vec custom types
            // TODO:  Parse structs with multiple members correctly so they can be handled here.
            quote! {
                let serialized_struct = map.get(#field_name_str)
                    .ok_or(NormalizationError::MissingField)?;
                println!("14: Serialized struct: {:?}", serialized_struct);
                let #field_str = #field_type::deserialize(serialized_struct)?;
            }
        }
    } else {
        panic!("Unsupported field type!");
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
                        let mut nesting_level = 0;  // Track nesting level for structs

                        for (i, char) in trimmed.chars().enumerate() {
                            if escaped {
                                escaped = false;
                                continue;
                            }

                            match char {
                                '\\' => escaped = true,
                                '[' | '{' => nesting_level += 1,
                                ']' | '}' => nesting_level -= 1,
                                ',' if nesting_level == 0 => {
                                    // Split on comma only at the top level (outside of any nested structures)
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

            // let expanded = quote! {
            //     impl #name {
            //         pub fn deserialize(input: &str) -> Result<Self, NormalizationError> {
            //             let trimmed = input.trim_matches(|c| c == '{' || c == '}');
            //             let mut parts: Vec<&str> = Vec::new();
            //             let mut start = 0;
            //             let mut escaped = false;
            //             let mut in_vector = false;
            //             let mut nesting_level = 0;  // Added for tracking nesting level

            //             for (i, char) in trimmed.chars().enumerate() {
            //                 if escaped {
            //                     escaped = false;
            //                     continue;
            //                 }

            //                 match char {
            //                     '\\' => escaped = true,
            //                     '[' => in_vector = true,
            //                     ']' => in_vector = false,
            //                     '{' => nesting_level += 1,  // Increment nesting level
            //                     '}' => nesting_level -= 1,  // Decrement nesting level
            //                     ',' if !in_vector && nesting_level == 0 => {
            //                         // Split on comma only if not in a vector and not in a nested struct
            //                         parts.push(&trimmed[start..i]);
            //                         start = i + 1;
            //                     }
            //                     _ => (),
            //                 }
            //             }
            //             parts.push(&trimmed[start..]);

            //             let mut map = std::collections::HashMap::new();
            //             for part in parts.iter() {
            //                 let kv: Vec<&str> = part.splitn(2, ':').map(str::trim).collect();
            //                 if kv.len() == 2 {
            //                     map.insert(kv[0], kv[1]);
            //                 }
            //             }

            //             #(#deserialize_fields)*
            //             Ok(Self { #(#field_names),* })
            //         }
            //     }
            // };
            // let expanded = quote! {
            //     impl #name {
            //         pub fn deserialize(input: &str) -> Result<Self, NormalizationError> {
            //             let trimmed = input.trim_matches(|c| c == '{' || c == '}');
            //             let mut parts: Vec<&str> = Vec::new();
            //             let mut start = 0;
            //             let mut escaped = false;
            //             let mut in_vector = false;

            //             for (i, char) in trimmed.chars().enumerate() {
            //                 if escaped {
            //                     escaped = false;
            //                     continue;
            //                 }

            //                 match char {
            //                     '\\' => escaped = true,
            //                     '[' => in_vector = true,
            //                     ']' => in_vector = false,
            //                     ',' if !in_vector => {
            //                         parts.push(&trimmed[start..i]);
            //                         start = i + 1;
            //                     }
            //                     _ => (),
            //                 }
            //             }
            //             parts.push(&trimmed[start..]);

            //             let mut map = std::collections::HashMap::new();
            //             for part in parts.iter() {
            //                 let kv: Vec<&str> = part.splitn(2, ':').map(str::trim).collect();
            //                 if kv.len() == 2 {
            //                     map.insert(kv[0], kv[1]);
            //                 }
            //             }

            //             #(#deserialize_fields)*
            //             Ok(Self { #(#field_names),* })
            //         }
            //     }
            // };

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
