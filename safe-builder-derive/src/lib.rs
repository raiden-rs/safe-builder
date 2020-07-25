mod expander;

use expander::*;
use proc_macro::TokenStream;

use syn::*;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_safe_builder(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(expand(input))
}
