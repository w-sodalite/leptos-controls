use crate::field::Field;
use crate::meta::FieldMeta;
use leptos::*;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct SignalField<M, T>
where
    M: FieldMeta<Type = T>,
    T: Clone + 'static,
{
    pub(crate) value: Signal<T>,
    _mark: PhantomData<M>,
}

impl<M, T> SignalField<M, T>
where
    M: FieldMeta<Type = T>,
    T: Clone + 'static,
{
    pub fn new(value: T) -> Self {
        Self {
            value: Signal::derive(move || value.clone()),
            _mark: PhantomData,
        }
    }
}

impl<M, T> Field for SignalField<M, T>
where
    T: Clone + 'static,
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

    fn set_default(&self) {}
}

impl<M, T> Clone for SignalField<M, T>
where
    T: Clone + 'static,
    M: FieldMeta<Type = T>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, T> Copy for SignalField<M, T>
where
    T: Clone + 'static,
    M: FieldMeta<Type = T>,
{
}

impl<M, T> SignalWithUntracked for SignalField<M, T>
where
    T: Clone + 'static,
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
impl<M, T> SignalWith for SignalField<M, T>
where
    T: Clone + 'static,
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

impl<M, T> SignalGetUntracked for SignalField<M, T>
where
    T: Clone + 'static,
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

impl<M, T> SignalGet for SignalField<M, T>
where
    T: Clone + 'static,
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
