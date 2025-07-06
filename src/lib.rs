use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

mod enum_trait_matrix;
mod parse;

#[proc_macro_attribute]
pub fn enum_trait_matrix(_: TokenStream, tokens: TokenStream) -> TokenStream {
    let module = parse_macro_input!(tokens as ItemMod);
    enum_trait_matrix::enum_trait_matrix(module).into()
}

#[cfg(feature = "fn_style")]
#[proc_macro]
pub fn enum_trait_matrix_fn_style(tokens: TokenStream) -> TokenStream {
    let module = parse_macro_input!(tokens as ItemMod);
    enum_trait_matrix::enum_trait_matrix(module).into()
}
