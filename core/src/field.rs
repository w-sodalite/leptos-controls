use std::borrow::Cow;

pub trait Field {
    fn label(&self) -> &'static str;
    fn required(&self) -> bool;
    fn validate(&self) -> Option<Cow<'static, str>>;
    fn set_default(&self);
}