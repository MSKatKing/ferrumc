use crate::raw::RawNbt;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use tokio::io::{AsyncRead, AsyncWrite};

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

impl<T: Serialize + DeserializeOwned + Clone> NetEncode for Nbt<T> {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        RawNbt::encode(self.inner.clone())
            .encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        RawNbt::encode(self.inner.clone())
            .encode_async(writer, opts).await
    }
}

impl<T: Serialize + DeserializeOwned> NetDecode for Nbt<T> {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        Ok(Self {
            inner: <RawNbt as NetDecode>::decode(reader, opts)?
                .decode()
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        Ok(Self {
            inner: <RawNbt as NetDecode>::decode_async(reader, opts)
                .await?
                .decode()
        })
    }
}