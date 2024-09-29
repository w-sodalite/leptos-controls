use crate::{FieldMeta, RwSignalField};
use thaw_utils::Model;

impl<M: FieldMeta, T: Clone + Default + 'static> From<RwSignalField<M, T>> for Model<T> {
    fn from(field: RwSignalField<M, T>) -> Self {
        field.value.into()
    }
}
