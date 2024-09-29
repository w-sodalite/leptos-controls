use crate::options::FormOptions;
use proc_macro2::TokenStream;
use quote::quote;

pub struct FieldEnum<'a> {
    options: &'a FormOptions,
}

impl<'a> FieldEnum<'a> {
    pub fn new(options: &'a FormOptions) -> Self {
        Self { options }
    }
}

impl<'a> From<FieldEnum<'a>> for TokenStream {
    fn from(value: FieldEnum<'a>) -> Self {
        let options = value.options;
        let vis = options.vis();
        // 字段结构体
        let field_struct_tokens = options.fields().iter().map(|field| {
            let pascal_case_ident = field.struct_ident();
            quote! {
               #[doc(hidden)]
               #[derive(Clone,Copy)]
               #vis struct #pascal_case_ident;
            }
        });

        // 实现FieldMeta
        let impl_field_meta_tokens = options.fields().iter().map(|field| {
            let pascal_case_ident = field.struct_ident();
            let label = field.label();
            let required = field.validate().is_some();
            quote! {
                impl leptos_controls::FieldMeta for #pascal_case_ident {
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
