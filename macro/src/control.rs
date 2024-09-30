use crate::options::FormOptions;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

pub struct ControlStruct<'a> {
    options: &'a FormOptions,
}

impl<'a> ControlStruct<'a> {
    pub fn new(options: &'a FormOptions) -> Self {
        Self { options }
    }
}

impl<'a> From<ControlStruct<'a>> for TokenStream {
    fn from(value: ControlStruct<'a>) -> Self {
        let options = value.options;
        let ident = options.ident();
        let vis = options.vis();
        let control_struct_ident = options.control_struct_ident();
        let field_tokens = options.field_tokens();

        // 控制器字段
        let field_with_type_tokens = options.fields().iter().map(|field| {
            let ty = field.ty();
            let field_ident = field.ident();
            let field_struct_ident = field.struct_ident(ident);
            quote! {
                #field_ident: leptos_controls::RwSignalField<#field_struct_ident, #ty>
            }
        });

        // 创建RwSignalField
        let set_signal_tokens = options.fields().iter().map(|field| {
            let field_ident = field.ident();
            quote! {
                let #field_ident = leptos_controls::RwSignalField::new(#field_ident);
            }
        });

        // rest函数
        let fn_reset_tokens = options.fields().iter().map(|field| {
            let field_ident = field.ident();
            quote! {
                self.#field_ident.set_default();
            }
        });

        // snapshot函数
        let get_untracked_tokens = options.fields().iter().map(|field| {
            let ty = field.ty();
            let field_ident = field.ident();
            let field_struct_ident = field.struct_ident(ident);
            quote! {
                let #field_ident = <leptos_controls::RwSignalField<#field_struct_ident,#ty> as leptos::SignalGetUntracked>::get_untracked(&#field_ident);
            }
        });

        // validate函数
        let fn_validate_tokens = options
            .fields()
            .iter()
            .filter(|field| field.validate().is_some())
            .map(|field| {
                let ty = field.ty();
                let field_ident = field.ident();
                let field_struct_ident = field.struct_ident(ident);
                let label = field.label();
                let message = field.message();
                let validate = field.validate().unwrap();
                let validate_method = Path::from_string(validate).unwrap();
                let error = match message {
                    Some(message) => quote! {
                            std::borrow::Cow::from(#message)
                        },
                    None => quote! {
                            std::borrow::Cow::from(concat!(#label, "校验失败!"))
                        }
                };
                quote! {
                        if #validate_method(&<leptos_controls::RwSignalField<#field_struct_ident,#ty> as leptos::SignalGetUntracked>::get_untracked(&self.#field_ident)) {
                            None
                        }else{
                            Some(#error)
                        }
                    }
            }).collect::<Vec<_>>();
        let fn_validate_body = if fn_validate_tokens.is_empty() {
            quote! {
                vec![]
            }
        } else {
            quote! {
                #[allow(unused_variables)]
                    let #control_struct_ident { #(#field_tokens,)* .. } = *self;
                    vec![#(#fn_validate_tokens,)*].into_iter().flatten().collect()
            }
        };

        quote! {
            #[derive(Clone, Copy)]
            #vis struct #control_struct_ident {
                #(#field_with_type_tokens,)*
            }

            impl #control_struct_ident {
                pub fn new(value: #ident) -> Self {
                  let #ident { #(#field_tokens,)*.. }  = value;
                    #(#set_signal_tokens)*
                    #control_struct_ident {
                        #(#field_tokens,)*
                    }
                }
                pub fn set_default(&self) {
                    #(#fn_reset_tokens)*
                }
                pub fn snapshot(&self) -> #ident {
                    let #control_struct_ident { #(#field_tokens,)* .. } = *self;
                    #(#get_untracked_tokens)*
                    #ident{
                        #(#field_tokens,)*
                    }
                }
                pub fn validate(&self) -> Vec<std::borrow::Cow<'static,str>> {
                    #fn_validate_body
                }
            }
        }
    }
}
