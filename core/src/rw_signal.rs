use crate::field::Field;
use crate::FieldMeta;
use leptos::*;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    pub(crate) value: RwSignal<T>,
    _mark: PhantomData<M>,
}

impl<M, T> RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    pub fn new(value: T) -> Self {
        Self {
            value: create_rw_signal(value),
            _mark: PhantomData,
        }
    }
}

impl<M, T> Field for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    fn label(&self) -> &'static str {
        M::LABEL
    }

    fn required(&self) -> bool {
        M::REQUIRED
    }

    fn validate(&self) -> Option<Cow<'static, str>> {
        M::VALIDATE(&self.get_untracked())
    }

    fn set_default(&self) {
        self.value.set(Default::default());
    }
}

impl<M, T> Default for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<M, T> SignalWithUntracked for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    type Value = T;

    fn with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.value.with_untracked(f)
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.value.try_with_untracked(f)
    }
}
impl<M, T> SignalWith for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    type Value = T;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.value.with(f)
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.value.try_with(f)
    }
}
impl<M, T> SignalUpdateUntracked<T> for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    fn update_untracked(&self, f: impl FnOnce(&mut T)) {
        self.value.update_untracked(f)
    }

    fn try_update_untracked<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        self.value.try_update_untracked(f)
    }
}
impl<M, T> SignalUpdate for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.value.update(f);
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        self.value.try_update(f)
    }
}
impl<M, T> SignalSetUntracked<T> for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    fn set_untracked(&self, new_value: T) {
        self.value.set_untracked(new_value);
    }

    fn try_set_untracked(&self, new_value: T) -> Option<T> {
        self.value.try_set_untracked(new_value)
    }
}
impl<M, T> SignalSet for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.value.set(new_value);
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.value.try_set(new_value)
    }
}
impl<M, T> SignalGetUntracked for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    type Value = T;

    fn get_untracked(&self) -> Self::Value {
        self.value.get_untracked()
    }

    fn try_get_untracked(&self) -> Option<Self::Value> {
        self.value.try_get_untracked()
    }
}
impl<M, T> SignalGet for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    type Value = T;

    fn get(&self) -> Self::Value {
        self.value.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.value.try_get()
    }
}
impl<M, T> Clone for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<M, T> Copy for RwSignalField<M, T>
where
    T: Clone + Default + 'static,
    M: FieldMeta<Type = T>,
{
}
