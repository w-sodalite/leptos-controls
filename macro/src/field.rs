use crate::options::FormOptions;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

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
            let ty = field.ty();
            let field_ident = field.ident();
            let field_struct_ident = field.struct_ident(ident);
            let label = field.label();
            let required = field.validate().is_some();
            let message = field.message();
            let validate = match field.validate() {
                Some(validate) => {
                    let validate = Path::from_string(validate).unwrap();
                    let error = match message {
                        Some(message) => quote! {
                            std::borrow::Cow::from(#message)
                        },
                        None => quote! {
                            std::borrow::Cow::from(concat!(#label, "校验失败!"))
                        }
                    };
                    quote! {
                        move |v|{
                          if #validate(v) {
                                None
                            }else{
                                Some(#error)
                            }
                        }
                    }
                }
                None => quote! {
                    move |v| None
                },
            };
            quote! {
                impl leptos_controls::FieldMeta for #field_struct_ident {
                    type Type = #ty;
                    const LABEL: &'static str = #label;
                    const REQUIRED: bool = #required;
                    const VALIDATE: fn(&Self::Type) -> Option<std::borrow::Cow<'static, str>> = #validate;
                }
            }
        });

        quote! {
            #(#field_struct_tokens)*
            #(#impl_field_meta_tokens)*
        }
    }
}
