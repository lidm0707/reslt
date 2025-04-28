use quote::quote;
use syn::{Data, DeriveInput};

pub fn expand_field_accessible(input: DeriveInput) -> proc_macro::TokenStream {
    let name = input.ident;

    match input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().filter_map(|field| field.ident.as_ref());
            let matches = fields.map(|field| {
                quote! {
                    stringify!(#field) => Some(self.#field.to_string()),
                }
            });

            quote! {
                impl FieldAccessible for #name {
                    fn get_field(&self, field_name: &str) -> Option<String> {
                        match field_name {
                            #(#matches)*
                            _ => None,
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("FieldAccessible can only be derived for structs"),
    }
}
