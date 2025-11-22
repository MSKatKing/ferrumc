use std::io::Write;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use crate::raw::RawNbt;
use crate::ser::NbtSerializationOptions;

mod str;
mod bin;

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