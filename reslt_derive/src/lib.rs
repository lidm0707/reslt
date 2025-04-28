extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod field_accessible_logic;
use field_accessible_logic::expand_field_accessible;


mod utils;

#[proc_macro_derive(FieldAccessible)]
pub fn field_accessible_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let expanded = expand_field_accessible(input);
    TokenStream::from(expanded)
}
