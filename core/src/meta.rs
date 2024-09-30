use std::borrow::Cow;

pub trait FieldMeta {
    ///
    /// 字段类型
    ///
    type Type;

    ///
    /// 标签
    ///
    const LABEL: &'static str;

    ///
    /// 是否必须
    ///
    const REQUIRED: bool;

    ///
    /// 校验函数
    ///
    const VALIDATE: fn(&Self::Type) -> Option<Cow<'static, str>>;
}
