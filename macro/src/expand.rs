use crate::control::ControlStruct;
use crate::field::FieldStruct;
use crate::options::FormOptions;
use darling::{Error, FromDeriveInput};
use proc_macro2::TokenStream;
use syn::DeriveInput;

///
/// 处理错误信息
///
/// # Arguments
///
/// * `error`: 错误信息
///
/// returns: TokenStream
///
fn fallback(error: Error) -> TokenStream {
    error.write_errors()
}

///
/// 解析DeriveInput对象生成代码
///
/// # Arguments
///
/// * `input`: 需要处理的结构体
///
/// returns: TokenStream
///
pub fn derive(input: DeriveInput) -> TokenStream {
    try_expand(&input).unwrap_or_else(fallback)
}

fn try_expand(input: &DeriveInput) -> Result<TokenStream, Error> {
    let options = FormOptions::from_derive_input(input).and_then(FormOptions::validate)?;
    let field_enum = TokenStream::from(FieldStruct::new(&options));
    let control_struct = TokenStream::from(ControlStruct::new(&options));
    Ok(TokenStream::from_iter([field_enum, control_struct]))
}
