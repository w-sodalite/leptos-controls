use darling::{ast, Error, FromDeriveInput, FromField, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;
use syn::{Type, Visibility};

const CONTROL_IDENT: &str = "Controls";

#[derive(FromDeriveInput)]
#[darling(attributes(controls, field), supports(struct_named))]
pub struct ControlOptions {
    ///
    /// 可见性
    ///
    vis: Visibility,

    ///
    /// 类型名称
    ///
    ident: Ident,

    ///
    /// 字段集合
    ///
    data: ast::Data<(), ControlFieldOptions>,

    ///
    /// 校验方法
    ///
    #[darling(default)]
    validate: Option<syn::Path>,
}

impl ControlOptions {
    pub fn vis(&self) -> &Visibility {
        &self.vis
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn fields(&self) -> &[ControlFieldOptions] {
        match &self.data {
            ast::Data::Enum(_) => unreachable!(),
            ast::Data::Struct(fields) => &fields.fields,
        }
    }

    pub fn validate(&self) -> Option<&syn::Path> {
        self.validate.as_ref()
    }

    pub fn control_struct_ident(&self) -> Ident {
        format_ident!("{}{}", self.ident, CONTROL_IDENT)
    }

    pub fn field_tokens(&self) -> Vec<TokenStream> {
        self.fields()
            .iter()
            .map(|field| field.ident())
            .map(|ident| quote! {#ident})
            .collect()
    }

    ///
    /// 校验当前类型是否满足
    ///
    pub fn verify(self) -> Result<Self, Error> {
        match &self.data {
            ast::Data::Enum(_) => Err(Error::unexpected_type("Enum")),
            ast::Data::Struct(fields) => {
                if fields.fields.iter().any(|field| field.ident.is_none()) {
                    Err(Error::custom("Exists empty ident!"))
                } else {
                    Ok(self)
                }
            }
        }
    }
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct ControlAttributes {
    validate: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(field))]
pub struct ControlFieldOptions {
    ///
    /// 可见性
    ///
    vis: Visibility,

    ///
    /// 字段名称
    ///
    ident: Option<Ident>,

    ///
    /// 字段类型
    ///
    ty: Type,

    ///
    /// 是否忽略
    ///
    #[darling(default)]
    readonly: bool,

    ///
    /// 字段标签
    ///
    #[darling(default)]
    label: Option<String>,

    ///
    /// 校验方法
    ///
    validate: Option<String>,

    ///
    /// 校验信息
    ///
    message: Option<String>,
}

impl ControlFieldOptions {
    pub fn vis(&self) -> &Visibility {
        &self.vis
    }

    pub fn ident(&self) -> &Ident {
        self.ident.as_ref().expect("Ident is not exists!")
    }

    pub fn struct_ident(&self, parent: &Ident) -> Ident {
        let lit = format!("{}", self.ident());
        let lit = lit
            .split('_')
            .map(|v| {
                if v.len() == 1 {
                    v.to_uppercase()
                } else {
                    let (first, other) = v.split_at(1);
                    format!("{}{}", first.to_uppercase(), other)
                }
            })
            .collect::<Vec<_>>()
            .join("");
        format_ident!("{}{}", parent, lit)
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn readonly(&self) -> bool {
        self.readonly
    }

    pub fn label(&self) -> Cow<'_, str> {
        match self.label {
            Some(ref label) => Cow::from(label),
            None => {
                let ident = self.ident();
                Cow::from(ident.to_string())
            }
        }
    }

    pub fn validate(&self) -> Option<&str> {
        self.validate.as_deref()
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
