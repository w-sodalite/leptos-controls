mod control;
mod expand;
mod field;
mod options;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Controls, attributes(controls, field))]
pub fn derive_controls(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    expand::derive(input).into()
}
