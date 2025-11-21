use std::ops::{Deref, DerefMut};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A facade for reading and writing structs in Nbt format
pub struct Nbt<T: Serialize + DeserializeOwned> {
    inner: T,
}

impl<T: Serialize + DeserializeOwned> Nbt<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Serialize + DeserializeOwned> Deref for Nbt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Serialize + DeserializeOwned> DerefMut for Nbt<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}