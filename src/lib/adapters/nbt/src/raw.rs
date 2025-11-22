use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use crate::tag::{NbtTag, NbtTagType};

pub struct RawNbt {
    root: Vec<(String, NbtTag)>,
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
        writer.write_all(&[NbtTagType::Compound as u8])?;
        for (tag_name, tag) in &self.root {
            writer.write_all(&(tag_name.len() as u16).to_be_bytes())?;
            writer.write_all(tag_name.as_bytes())?;
            tag.encode(writer, opts)?;
        }
        writer.write_all(&[NbtTagType::End as u8])?;

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(&[NbtTagType::Compound as u8]).await?;
        for (tag_name, tag) in &self.root {
            writer.write_all(&(tag_name.len() as u16).to_be_bytes()).await?;
            writer.write_all(tag_name.as_bytes()).await?;
            tag.encode_async(writer, opts).await?;
        }
        writer.write_all(&[NbtTagType::End as u8]).await?;

        Ok(())
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