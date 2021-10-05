extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumMateInfo)]
pub fn derive(input: TokenStream) -> TokenStream {
    input
}
