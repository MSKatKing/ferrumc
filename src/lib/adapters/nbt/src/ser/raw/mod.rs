use std::fmt::Display;
use std::io::Write;
use thiserror::Error;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use crate::raw::RawNbt;
use crate::ser::NbtSerializationOptions;

mod str;
mod bin;
mod custom;

pub use custom::RawNbtSerializer;

#[derive(Debug, Error)]
pub enum NbtSerializationError {
    #[error("Type can not be serialized into NBT")]
    TypeNotSupported,
    #[error("Only Strings can be used as map keys when serializing to NBT")]
    InvalidMapKey,
    #[error("Lists must contain the same type of data")]
    InvalidList,
    #[error("{0}")]
    Other(String),
}

impl serde::ser::Error for NbtSerializationError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Other(msg.to_string())
    }
}

impl NetEncode for RawNbt {
    fn encode<W: Write>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(
            &self.to_binary(NbtSerializationOptions::Network),
        )?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, _opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(
            &self.to_binary(NbtSerializationOptions::Network),
        ).await?;

        Ok(())
    }
}