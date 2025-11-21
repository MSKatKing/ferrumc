use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncWrite};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;

pub struct RawNbt {
    // TODO
}

impl RawNbt {
    pub fn decode<T: DeserializeOwned>(self) -> T {
        todo!()
    }

    pub fn encode<T: Serialize>(data: T) -> Self {
        todo!()
    }
}

impl NetEncode for RawNbt {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        todo!()
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        todo!()
    }
}

impl NetDecode for RawNbt {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        todo!()
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        todo!()
    }
}