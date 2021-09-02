use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod encoder;

#[proc_macro_derive(Encoder)]
pub fn derive_encoder(input: TokenStream) -> TokenStream {
    encoder::derive_encoder(parse_macro_input!(input as DeriveInput))
}