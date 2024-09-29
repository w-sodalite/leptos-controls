pub trait FieldMeta {
    ///
    /// 标签
    ///
    const LABEL: &'static str;

    ///
    /// 是否必须
    ///
    const REQUIRED: bool;
}
