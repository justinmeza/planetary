extern crate proc_macro;

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{Data, DataStruct, Fields, parse_macro_input, DeriveInput};

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Type::Path, TypePath};

fn generate_serialization_for_type(
    field_name: &Option<syn::Ident>,
    field_type: &TypePath,
) -> proc_macro2::TokenStream {
    if field_type.path.is_ident("i32") || field_type.path.is_ident("bool") {
        quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    } else if field_type.path.is_ident("String") {
        quote! {
            format!(
                "{}: \"{}\"",
                stringify!(#field_name),
                self.#field_name.replace("\\", "\\\\").replace(":", "\\:")
                    .replace("\"", "\\\"").replace(",", "\\,")
                // self.#field_name //.replace("\\", "\\\\").replace(",", "\\,")
            )
        }
    } else {
        panic!("Unsupported type!");
    }
}

#[proc_macro_derive(Serializable)]
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    // let name = &input.ident;
    // let fields = if let syn::Data::Struct(syn::DataStruct {
    //     fields: syn::Fields::Named(FieldsNamed { named, .. }),
    //     ..
    // }) = &input.data
    // {
    //     named
    // } else {
    //     panic!("Only named fields are supported");
    // };

    // let serialize_fields = fields.iter().map(|f| {
    //     let name = &f.ident;
    //     quote! {
    //         let #name = format!("{}: {:?}", stringify!(#name), &self.#name);
    //     }
    // });

    // let field_names = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    // let joined_fields = quote! { #(#field_names),* };

    // // let field_values = field_names.iter().map(|name| {
    // //     quote! { format!("{:?}", &self.#name) }
    // // }).collect::<Vec<_>>();
    // // let joined_values = quote! { #(#field_values),* };

    // // let joined_fields = quote! { #(#name),* };
    // let gen = quote! {
    //     impl normalization::Serializable for #name {
    //         fn serialize(&self) -> String {
    //             #(#serialize_fields;)*
    //             // format!("{} {{ {} }}", stringify!(#name), vec![#(#name),*].join(", "))
    //             // format!("{} {{ {} }}", stringify!(#name), #joined_fields)
    //             // format!("{} {{ {:?} }}", stringify!(#name), (#joined_fields))
    //             // number, flag
    //             // format!("{} {{ {} }}", stringify!(#name), #joined_fields)
    //             format!("{} {{ {:?} }}", stringify!(#name), (#joined_fields))
    //         }
    //     }
    // };

    // let input = parse_macro_input!(input as DeriveInput);
    // let name = &input.ident;

    // let gen = match &input.data {
    //     Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
    //         let field_names = fields.named.iter().map(|f| &f.ident);
    //         let field_strings = field_names.clone().map(|name| format!("{}", name.as_ref().unwrap()));

    //         quote! {
    //             impl #name {
    //                 pub fn serialize(&self) -> String {
    //                     let mut s = String::from("{");
    //                     #(
    //                         s.push_str(&format!("\"{}\": {},", #field_strings, self.#field_names));
    //                     )*
    //                     s.pop();  // Remove trailing comma
    //                     s.push('}');
    //                     s
    //                 }
    //             }
    //         }
    //     },
    //     _ => panic!("Only named structs are supported!"),
    // };

    // let input = parse_macro_input!(input as DeriveInput);
    // let name = &input.ident;

    // let gen = match &input.data {
    //     Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
    //         let serialization_logic = fields.named.iter().map(|f| {
    //             let field_name = &f.ident;
    //             let field_type = &f.ty;
    //             match field_type {
    //                 syn::Type::Path(type_path) if type_path.path.is_ident("i32") => {
    //                     quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    //                 }
    //                 syn::Type::Path(type_path) if type_path.path.is_ident("bool") => {
    //                     quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    //                 }
    //                 syn::Type::Path(type_path) if type_path.path.is_ident("String") => {
    //                     // quote! { format!("{}: \"{}\"", stringify!(#field_name), self.#field_name) }
    //                     quote! { format!("{}: \"{}\"", stringify!(#field_name), self.#field_name.replace("\\", "\\\\").replace(",", "\\,")) }
    //                 }
    //                 _ => panic!("Unsupported type!"),
    //             }
    //         });

    //         quote! {
    //             impl #name {
    //                 pub fn serialize(&self) -> String {
    //                     let parts = vec![#(#serialization_logic),*];
    //                     format!("{{{}}}", parts.join(","))
    //                 }
    //             }
    //         }
    //     },
    //     _ => panic!("Only named structs are supported!"),
    // };
    // gen.into()

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

fn generate_field_deserialization(
    field_name: &Option<Ident>,
    field_type: &TypePath,
) -> proc_macro2::TokenStream {
    let field_name_str = match field_name {
        Some(ident) => ident.to_string(),
        None => panic!("Field has no name"),
    };
    let field_str = proc_macro2::Literal::string(&field_name_str);
    if let Some(ident) = field_type.path.get_ident() {
        match ident.to_string().as_str() {
            "i32" | "bool" => {
                quote! {
                    println!("{:?}", map);
                    println!("{:?}", #field_str);
                    let #field_name: #field_type = map.get(#field_str)
                        .ok_or(NormalizationError::MissingField)?
                        .parse()
                        .map_err(|_| NormalizationError::ParseFailure)?;
                        // .map_err(|_| concat!("Failed to parse ", #field_str, " field"))?;
                }
            }
            "String" => {
                quote! {
                    println!("{:?}", map);
                    println!("{:?}", #field_str);
                    let raw_string = map.get(#field_str)
                        .ok_or(NormalizationError::MissingField)?;
                        // .splitn(2, ':').last().ok_or(NormalizationError::InvalidFormat)?;
                    // let raw_string = map.get(#field_str)
                    //     .ok_or(NormalizationError::MissingField)?
                    //     .splitn(2, ':').last().ok_or(NormalizationError::InvalidFormat)?;
                        // .splitn(2, ':').last().ok_or(NormalizationError::InvalidFormat)?;
                        // .replace("\\,", ",").replace("\\\\", "\\");
                    // let #field_name: String = raw_string.trim_matches('\"').to_string();
                    // println!("{:?}", map.get(#field_str).ok_or(NormalizationError::InvalidFormat)?.splitn(2, ':'));
                    println!("{:?}", raw_string);
                    let #field_name: String = raw_string.trim_matches('\"').replace("\\:", ":")
                        .replace("\\\"", "\"").replace("\\\\", "\\").replace("\\,", ",");
                }
            }
            _ => panic!("Unsupported field type!"),
        }
    } else {
        panic!("Unsupported field type!")
    }
}

#[proc_macro_derive(Deserializable)]
pub fn derive_deserializable(input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    // let name = &input.ident;
    // let fields = if let syn::Data::Struct(syn::DataStruct {
    //     fields: syn::Fields::Named(FieldsNamed { named, .. }),
    //     ..
    // }) = &input.data
    // {
    //     named
    // } else {
    //     panic!("Only named fields are supported");
    // };

    // let deserialize_fields = fields.iter().map(|f| {
    //     let name = &f.ident;
    //     let ty = &f.ty;
    //     quote! {
    //         let #name: #ty = parts.next().ok_or(normalization::NormalizationError::InvalidInput)?.split(": ").skip(1).next().ok_or(normalization::NormalizationError::InvalidInput)?.parse().map_err(|_| normalization::NormalizationError::InvalidInput)?;
    //     }
    // });

    // let assignments = fields.iter().map(|f| {
    //     let name = &f.ident;
    //     quote! { #name: #name }
    // }).collect::<Vec<_>>();

    // let gen = quote! {
    //     impl normalization::Deserializable for #name {
    //         fn deserialize(input: &str) -> Result<Self, normalization::NormalizationError> {
    //             if !input.starts_with(stringify!(#name)) {
    //                 return Err(normalization::NormalizationError::InvalidInput);
    //             }
    //             let content = &input[stringify!(#name).len() + 2..input.len() - 2];
    //             let mut parts = content.split(", ");
    //             #(#deserialize_fields)*
    //             // Ok(Self { #(#name),* })
    //             Ok(Self { #(#assignments),* })
    //         }
    //     }
    // };
    // gen.into()
    // let input = syn::parse_macro_input!(input as DeriveInput);
    // let name = &input.ident;

    // match &input.data {
    //     Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
    //         let deserialize_fields = fields.named.iter().map(|f| {
    //             let field_name = &f.ident;
    //             let field_type = &f.ty;

    //             match field_type {
    //                 syn::Type::Path(type_path) if type_path.path.is_ident("i32") => {
    //                     quote! {
    //                         println!("{:?}", map);
    //                         let #field_name: i32 = map.get(stringify!(#field_name))
    //                             .ok_or("Missing field")?
    //                             .parse()
    //                             .map_err(|_| "Failed to parse i32 field")?;
    //                     }
    //                 },
    //                 syn::Type::Path(type_path) if type_path.path.is_ident("bool") => {
    //                     quote! {
    //                         println!("{:?}", map);
    //                         let #field_name: bool = map.get(stringify!(#field_name))
    //                             .ok_or("Missing field")?
    //                             .parse()
    //                             .map_err(|_| "Failed to parse bool field")?;
    //                     }
    //                 },
    //                 syn::Type::Path(type_path) if type_path.path.is_ident("String") => {
    //                     quote! {
    //                         println!("{:?}", map);
    //                         // let #field_name: String = map.get(stringify!(#field_name))
    //                         //     .ok_or("Missing field")
    //                         //     .map(|s| s.to_string())
    //                         //     .ok_or("Failed to parse String field")?;
    //                         // let #field_name: String = map.get(stringify!(#field_name))
    //                         //     .ok_or("Missing String field")?
    //                         //     .to_string();

    //                         // let raw_string = map.get(stringify!(#field_name)).ok_or("Missing field")?;
    //                         let raw_string = map.get(stringify!(#field_name)).ok_or("Missing field")?.splitn(2, ':').last().ok_or("Invalid format")?.replace("\\,", ",").replace("\\\\", "\\");
    //                         let #field_name: String = raw_string.trim_matches('\"').to_string();
    //                         // let #name = parts.next().ok_or("Missing field")?.splitn(2, ':').last().ok_or("Invalid format")?;
    //                         // let #name = parts.next().ok_or("Missing field")?.splitn(2, ':').last().ok_or("Invalid format")?.replace("\\,", ",").replace("\\\\", "\\");
    //                     }
    //                 },
    //                 _ => panic!("Unsupported field type! Only i32, bool, and String are supported."),
    //             }
    //         });

    //         let field_names = fields.named.iter().map(|f| &f.ident);

    //         let expanded = quote! {
    //             impl #name {
    //                 pub fn deserialize(input: &str) -> Result<Self, &'static str> {
    //                     let parts: Vec<&str> = input.trim_matches(|c| c == '{' || c == '}').split(',').collect();
    //                     let mut map = std::collections::HashMap::new();
    //                     for part in parts.iter() {
    //                         let kv: Vec<&str> = part.split(':').map(|s| s.trim()).collect();
    //                         if kv.len() == 2 {
    //                             map.insert(kv[0], kv[1]);
    //                         }
    //                     }
    //                     #(#deserialize_fields)*
    //                     Ok(Self { #(#field_names),* })
    //                 }
    //             }
    //         };

    //         expanded.into()
    //     },
    //     _ => panic!("Only named structs are supported!"),
    // }

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
                    // pub fn deserialize(input: &str) -> Result<Self, &'static str> {
                    pub fn deserialize(input: &str) -> Result<Self, NormalizationError> {
                        // println!("{}", input);
                        let trimmed = input.trim_matches(|c| c == '{' || c == '}');
                        let parts: Vec<&str> = {
                            let mut segments = Vec::new();
                            let mut start = 0;
                            let mut escaped = false;
                            for (i, char) in trimmed.chars().enumerate() {
                                match char {
                                    '\\' => escaped = true,
                                    ',' if !escaped => {
                                        segments.push(&trimmed[start..i]);
                                        start = i + 1;
                                    }
                                    _ => escaped = false,
                                }
                            }
                            // println!("{:?}", segments);
                            segments.push(&trimmed[start..]);
                            segments.into_iter().map(|s| s.trim()).collect::<Vec<_>>()
                            // segments.into_iter().collect()
                        };
                        // let parts: Vec<&str> = split_on_unescaped_commas(input.trim_matches(|c| c == '{' || c == '}')).into_iter().collect();
                        // let parts: Vec<&str> = input.trim_matches(|c| c == '{' || c == '}')
                        //     .split(',').collect();
                        println!("{:?}", parts);
                        let mut map = std::collections::HashMap::new();
                        for part in parts.iter() {
                            // let kv: Vec<&str> = split_on_unescaped_colon(part).into_iter().map(|s| s.trim()).collect();
                            // let kv: Vec<&str> = part.split(':').map(|s| s.trim()).collect();
                            let kv: Vec<&str> = {
                                let mut parts = Vec::new();
                                let mut start = 0;
                                let mut escaped = false;
                                for (i, char) in part.chars().enumerate() {
                                    match char {
                                        '\\' => escaped = true,
                                        ':' if !escaped => {
                                            parts.push(&part[start..i]);
                                            start = i + 1;
                                        }
                                        _ => escaped = false,
                                    }
                                }
                                parts.push(&part[start..]);
                                parts.into_iter().map(|s| s.trim()).collect::<Vec<_>>()
                            };
                            println!("{:?}", kv);
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
