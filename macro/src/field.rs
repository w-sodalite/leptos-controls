use crate::options::FormOptions;
use proc_macro2::TokenStream;
use quote::quote;

pub struct FieldStruct<'a> {
    options: &'a FormOptions,
}

impl<'a> FieldStruct<'a> {
    pub fn new(options: &'a FormOptions) -> Self {
        Self { options }
    }
}

impl<'a> From<FieldStruct<'a>> for TokenStream {
    fn from(value: FieldStruct<'a>) -> Self {
        let options = value.options;
        let ident = options.ident();
        let vis = options.vis();
        // 字段结构体
        let field_struct_tokens = options.fields().iter().map(|field| {
            let field_struct_ident = field.struct_ident(ident);
            quote! {
               #[doc(hidden)]
               #[derive(Clone,Copy)]
               #vis struct #field_struct_ident;
            }
        });

        // 实现FieldMeta
        let impl_field_meta_tokens = options.fields().iter().map(|field| {
            let field_struct_ident = field.struct_ident(ident);
            let label = field.label();
            let required = field.validate().is_some();
            quote! {
                impl leptos_controls::FieldMeta for #field_struct_ident {
                    const LABEL: &'static str = #label;
                    const REQUIRED: bool = #required;
                }
            }
        });

        quote! {
            #(#field_struct_tokens)*
            #(#impl_field_meta_tokens)*
        }
    }
}
