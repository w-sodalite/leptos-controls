use crate::options::FormOptions;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Path, Type};

pub struct Controls<'a> {
    options: &'a FormOptions,
}

impl<'a> Controls<'a> {
    pub fn new(options: &'a FormOptions) -> Self {
        Self { options }
    }
}

impl<'a> From<Controls<'a>> for TokenStream {
    fn from(controls: Controls<'a>) -> Self {
        let options = controls.options;
        let ident = options.ident();
        let vis = options.vis();
        let controls_ident = options.controls_ident();
        let field_enum_ident = options.field_enum_ident();
        let fields_with_comma_tokens = options.fields_with_comma_tokens();

        let view_field_tokens = options.fields().iter().map(|field| {
            let ident = field.ident();
            quote! {
                #ident: #field_enum_ident
            }
        });
        let fn_set_tokens = options.fields().iter().map(|field| {
            let ident = field.ident();
            quote! {
                <#field_enum_ident as thaw_form::FormField>::reset(&self.#ident);
            }
        });

        let mut field_define_tokens = vec![];
        let mut field_set_tokens = vec![];
        let mut field_initialize_tokens = vec![];
        options.fields().iter().for_each(|field| {
            let field_ident = field.ident();
            let field_variant_ident = field.variant_ident();
            let option_field_ident = format_ident!("__{}", field_ident);
            let ty = field.ty();
            field_define_tokens.push(quote! {let mut #option_field_ident = None;});
            field_initialize_tokens.push(quote! {#field_ident: #option_field_ident.unwrap()});
            field_set_tokens.push(quote! {
                if let #field_enum_ident::#field_variant_ident(value) = #field_ident {
                    #option_field_ident = Some(<leptos::RwSignal<#ty as leptos::SignalGet>::get(&value));
                }
            });
        });

        let fn_validate_tokens = options
            .fields()
            .iter()
            .filter(|field| field.required())
            .map(|field| {
                let ident = field.ident();
                let variant_ident = field.variant_ident();
                let ty = field.ty();
                let label = field.label();
                let message = field.message();
                match field.validate() {
                    Some(validate) => {
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
                        if let #field_enum_ident::#variant_ident(value) = #ident {
                             if #validate_method(&<leptos::RwSignal<#ty> as leptos::SignalGetUntracked>::get_untracked(&value)) {
                                None
                            }else{
                                Some(#error)
                            }
                        } else {
                            None
                        }
                    }
                    }
                    None => quote! {None}
                }
            });
        quote! {
            #[derive(Clone, Copy)]
            #vis struct #controls_ident {
                #(#view_field_tokens,)*
            }

            impl #controls_ident {
                pub fn reset(&self) {
                    #(#fn_set_tokens)*
                }
                pub fn snapshot(&self) -> #ident {
                    let #controls_ident { #(#fields_with_comma_tokens,)* .. } = *self;
                    #(#field_define_tokens)*
                    #(#field_set_tokens)*
                    #ident{
                        #(#field_initialize_tokens,)*
                    }
                }
                pub fn validate(&self) -> Vec<std::borrow::Cow<'static,str>> {
                    #[allow(unused_variables)]
                    let #controls_ident { #(#fields_with_comma_tokens,)* .. } = *self;
                    vec![#(#fn_validate_tokens,)*].into_iter().flatten().collect()
                }
            }
        }
    }
}
