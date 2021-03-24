extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

#[derive(Debug)]
struct EnvField {
    ident: Ident,
    ty: Type,
    is_option: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(env))]
struct EnvAttributes {
    deserializer: Ident,
}

#[proc_macro_derive(EnvConfig, attributes(env))]
pub fn derive_env_config(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let builder_name = Ident::new(
        &*format!("Envconfig{}Intermediary", struct_name),
        struct_name.span(),
    );
    let fields = get_fields(&ast.data);
    let opt_struct = build_opt_struct(&builder_name, &fields);
    let opt_impl = build_impl_opt(&builder_name, &struct_name, &fields);
    //let load_impl = build_load_impl(&struct_name, &args.deserializer, &builder_name);
    let expanded = quote! {
        #opt_struct
        #opt_impl
        //#load_impl
    };
    proc_macro::TokenStream::from(expanded)
}

fn get_fields(data: &Data) -> Vec<EnvField> {
    let mut treated_field: Vec<EnvField> = vec![];
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.clone() {
                    let ident = field.ident.expect("A field does not contain a name.");
                    if let Some(ty) = extract_option(&ident, &field.ty) {
                        treated_field.push(EnvField {
                            ident,
                            ty,
                            is_option: true,
                        })
                    } else {
                        treated_field.push(EnvField {
                            ident,
                            ty: field.ty,
                            is_option: false,
                        });
                    }
                }
            }
            Fields::Unnamed(_) => panic!("This crate does not support unnamed fields yet."),
            Fields::Unit => panic!("This struct is empty!"),
        },
        Data::Enum(_) | Data::Union(_) => panic!("This trait can only be used on structs."),
    }
    treated_field
}

fn extract_option(name: &Ident, ty: &Type) -> Option<Type> {
    match ty {
        Type::Path(ref data) => {
            for segment in &data.path.segments {
                if segment.ident.to_string() == "Option" {
                    match &segment.arguments {
                        PathArguments::AngleBracketed(elem) => {
                            for arg in &elem.args {
                                match arg {
                                    GenericArgument::Type(ty) => return Some(ty.clone()),
                                    _ => panic!(
                                        "The type of {} is not supported yet. (Inside option)",
                                        name
                                    ),
                                }
                            }
                        }
                        _ => panic!("The type of {} is not supported yet. (After option)", name),
                    }
                }
            }
            return None;
        }
        _ => panic!("The type of {} is not supported yet.", name),
    }
}

fn filter_optional(fields: &Vec<EnvField>) -> (Vec<&EnvField>, Vec<&EnvField>) {
    let optional: Vec<&EnvField> = fields.iter().filter(|e| e.is_option).collect();
    let mandatory: Vec<&EnvField> = fields.iter().filter(|e| !e.is_option).collect();
    (optional, mandatory)
}

fn build_opt_struct(name: &Ident, fields: &Vec<EnvField>) -> proc_macro2::TokenStream {
    let fields: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|EnvField { ident, ty, .. }| quote!(#ident: Option<#ty>))
        .collect();

    quote! {
        #[doc(hidden)]
        struct #name {
            #(#fields),*
        }
    }
}

fn build_load_impl(
    struct_name: &Ident,
    deserialize_function: &Ident,
    builder_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl proc_config::EnvConfig for #struct_name {
            fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Box<Self>, proc_config::Error> {
                let data = std::fs::read_to_string(path)?;
                match #deserialize_function::<#builder_name>(data) {
                    Ok(e) => unimplemented!(),
                    Err(_) => return Err(proc_config::Error::ParsingError())
                }
            }
        }
    }
}

fn build_impl_opt(
    tmp_name: &Ident,
    final_name: &Ident,
    fields: &Vec<EnvField>,
) -> proc_macro2::TokenStream {
    let (optional, mandatory) = filter_optional(fields);
    let optional: Vec<proc_macro2::TokenStream> = optional
        .iter()
        .map(|EnvField { ident, .. }| quote!(#ident: self.#ident))
        .collect();
    let mandatory_names: Vec<&proc_macro2::Ident> = mandatory.iter().map(|e| &e.ident).collect();
    let mandatory_strings: Vec<String> = mandatory.iter().map(|e| e.ident.to_string()).collect();
    let mandatory: Vec<proc_macro2::TokenStream> = mandatory
        .iter()
        .map(|EnvField { ident, .. }| {
            quote! {
                #ident: self.#ident.unwrap()
            }
        })
        .collect();
    let tokens = quote! {
        impl #tmp_name {
            fn finalize(self) -> Result<#final_name, proc_config::Error> {
                let mut invalid_fields: Vec<String> = vec![];
                #(
                if self.#mandatory_names.is_none() {
                    invalid_fields.push(#mandatory_strings.to_string());
                }
                )*
                if invalid_fields.len() == 0 {
                    std::result::Result::Ok(#final_name {
                        #(#optional,)*
                        #(#mandatory,)*
                    })
                } else {
                    std::result::Result::Err(proc_config::Error::MissingFields(invalid_fields))
                }

            }
        }
    };
    tokens
}
