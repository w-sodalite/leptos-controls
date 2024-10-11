use std::borrow::Cow;

pub trait Field {
    ///
    /// 字段名称
    ///
    fn label(&self) -> &'static str;

    ///
    /// 字段是否必须
    ///
    fn required(&self) -> bool;

    ///
    /// 字段校验
    ///
    fn validate(&self) -> Option<Cow<'static, str>>;

    ///
    /// 设置默认值
    ///
    fn set_default(&self);
}
