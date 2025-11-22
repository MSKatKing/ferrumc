use thiserror::Error;
use crate::NbtTag;

mod raw;

#[derive(Debug, Error)]
pub enum NbtDeserializationError {
    #[error("Type is not supported in Nbt")]
    TypeNotSupported,
    #[error("Unit struct expected empty compound, but found entries in compound")]
    NonEmptyUnitStruct,
    #[error("Expected tag {0}, found tag {1}")]
    TypeMismatch(String, String),
    #[error("{0}")]
    Other(String),
}

impl NbtDeserializationError {
    pub fn type_mismatch(expected: &str, found: &NbtTag) -> Self {
        Self::TypeMismatch(expected.to_string(), found.to_string())
    }
}

impl serde::de::Error for NbtDeserializationError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Other(msg.to_string())
    }
}