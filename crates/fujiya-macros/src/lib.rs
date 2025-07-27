use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, Lit};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, parse::Parser};
use syn::spanned::Spanned;

#[proc_macro_derive(Vertex, attributes(binding, location))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Vertex can only be derived for structs with named fields"),
        },
        _ => panic!("Vertex can only be derived for structs"),
    };

    let mut bindings = Vec::new();
    let mut attributes = Vec::new();
    let mut binding_counter = 0;
    let mut location_counter = 0;

    for field in fields {
        let field_name = &field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        let span = field.span();

        // Парсинг binding с обработкой ошибок
        let binding: u32 = field.attrs.iter()
            .find(|attr| attr.path().is_ident("binding"))
            .map(|attr| {
                attr.parse_args_with(|input: syn::parse::ParseStream| {
                    input.parse::<syn::LitInt>()?
                        .base10_parse::<u32>()
                })
                .unwrap_or_else(|_| {
                    panic!("Failed to parse binding attribute for field {}", field_name);
                })
            })
            .unwrap_or(binding_counter);

        let location: u32 = field.attrs.iter()
            .find(|attr| attr.path().is_ident("location"))
            .map(|attr| {
                attr.parse_args_with(|input: syn::parse::ParseStream| {
                    input.parse::<syn::LitInt>()?
                        .base10_parse::<u32>()
                })
                .unwrap_or_else(|_| {
                    panic!("Failed to parse location attribute for field {}", field_name);
                })
            })
            .unwrap_or_else(|| {
                let loc = location_counter;
                location_counter += 1;
                loc
            });

        let format = match field_ty {
            Type::Array(arr) => {
                match &*arr.elem {
                    Type::Path(type_path) => {
                        if type_path.path.is_ident("f32") {
                            match arr.len.clone() {
                                Expr::Lit(expr_lit) => {
                                    match expr_lit.lit {
                                        Lit::Int(lit_int) => {
                                            match lit_int.base10_parse::<usize>().unwrap() {
                                                2 => quote! { ash::vk::Format::R32G32B32A32_SFLOAT },
                                                3 => quote! { ash::vk::Format::R32G32B32A32_SFLOAT },
                                                4 => quote! { ash::vk::Format::R32G32B32A32_SFLOAT },
                                                _ => panic!("Unsupported array length for vertex attribute"),
                                            }
                                        }
                                        _ => panic!("Array length must be an integer literal"),
                                    }
                                }
                                _ => panic!("Array length must be a literal"),
                            }
                        } else {
                            panic!("Only f32 arrays are supported as vertex attributes")
                        }
                    }
                    _ => panic!("Vertex attributes must be arrays of primitive types"),
                }
            }
            _ => panic!("Vertex attributes must be arrays"),
        };

        if binding as usize >= bindings.len() {
            bindings.push(quote! {
                ash::vk::VertexInputBindingDescription {
                    binding: #binding,
                    stride: std::mem::size_of::<#field_ty>() as u32,
                    input_rate: ash::vk::VertexInputRate::VERTEX,
                }
            });
        }

        attributes.push(quote! {
            ash::vk::VertexInputAttributeDescription {
                location: #location,
                binding: #binding,
                format: #format,
                offset: 0,
            }
        });

        binding_counter = binding + 1;
    }

    let expanded = quote! {
        impl #name {
            pub fn get_binding_descriptions() -> Vec<ash::vk::VertexInputBindingDescription> {
                vec![#(#bindings),*]
            }

            pub fn get_attribute_descriptions() -> Vec<ash::vk::VertexInputAttributeDescription> {
                vec![#(#attributes),*]
            }
        }
    };

    expanded.into()
}