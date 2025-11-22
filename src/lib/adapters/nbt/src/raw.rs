use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use crate::tag::{NbtTag, NbtTagType, TagEntry};

#[derive(Debug, PartialEq)]
pub struct RawNbt {
    root: Vec<(String, NbtTag)>,
}

impl RawNbt {
    pub fn new() -> RawNbt {
        RawNbt { root: Vec::new() }
    }

    pub fn put_tag(&mut self, name: impl Into<String>, tag: NbtTag) {
        self.root.push((name.into(), tag));
    }

    pub fn deserialize<T: DeserializeOwned>(self) -> T {
        todo!()
    }

    pub fn serialize<T: Serialize>(_data: T) -> Self {
        todo!()
    }
}

impl NetEncode for RawNbt {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        writer.write_all(&[NbtTagType::Compound as u8])?;
        for (tag_name, tag) in &self.root {
            writer.write_all(&(tag.tag_type() as u8).to_be_bytes())?;
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
            writer.write_all(&(tag.tag_type() as u8).to_be_bytes()).await?;
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
        let start = u8::decode(reader, opts)?;
        if start != 0x0A { return Err(NetDecodeError::InvalidEnumVariant); }
        let mut tags = Vec::new();

        loop {
            let TagEntry(name, tag) = TagEntry::decode(reader, opts)?;
            if tag.tag_type() == NbtTagType::End { break; }

            tags.push((name, tag));
        }

        Ok(RawNbt { root: tags })
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let start = u8::decode_async(reader, opts).await?;
        if start != 0x0A { return Err(NetDecodeError::InvalidEnumVariant); }
        let mut tags = Vec::new();

        loop {
            let TagEntry(name, tag) = TagEntry::decode_async(reader, opts).await?;
            if tag.tag_type() == NbtTagType::End { break; }

            tags.push((name, tag));
        }

        Ok(RawNbt { root: tags })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    const NBT_BYTES: &'_ [u8] = &[
        0x0A, 0x01, 0x00, 0x04, 0x62, 0x79, 0x74, 0x65, 0x32, 0x02,
        0x00, 0x05, 0x73, 0x68, 0x6F, 0x72, 0x74, 0x00, 0x32, 0x03, 0x00, 0x03,
        0x69, 0x6E, 0x74, 0x00, 0x00, 0x00, 0x32, 0x04, 0x00, 0x04, 0x6C, 0x6F,
        0x6E, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x05, 0x00,
        0x05, 0x66, 0x6C, 0x6F, 0x61, 0x74, 0x42, 0x48, 0x00, 0x00, 0x06, 0x00,
        0x06, 0x64, 0x6F, 0x75, 0x62, 0x6C, 0x65, 0x40, 0x49, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x07, 0x00, 0x08, 0x62, 0x79, 0x74, 0x65, 0x5F, 0x61,
        0x72, 0x72, 0x00, 0x00, 0x00, 0x05, 0x32, 0x32, 0x32, 0x32, 0x32, 0x08,
        0x00, 0x03, 0x73, 0x74, 0x72, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
        0x09, 0x00, 0x07, 0x74, 0x61, 0x67, 0x5F, 0x61, 0x72, 0x72, 0x05, 0x00,
        0x00, 0x00, 0x02, 0x42, 0x48, 0x00, 0x00, 0x42, 0x6C, 0x00, 0x00, 0x0A,
        0x00, 0x08, 0x63, 0x6F, 0x6D, 0x70, 0x6F, 0x75, 0x6E, 0x64, 0x01, 0x00,
        0x02, 0x68, 0x69, 0x00, 0x00, 0x0B, 0x00, 0x07, 0x69, 0x6E, 0x74, 0x5F,
        0x61, 0x72, 0x72, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x32, 0x00,
        0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x32, 0x00,
        0x00, 0x00, 0x32, 0x0C, 0x00, 0x08, 0x6C, 0x6F, 0x6E, 0x67, 0x5F, 0x61,
        0x72, 0x72, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00
    ];

    #[tokio::test]
    async fn test_encode_nbt_async() {
        let mut writer = Vec::new();
        let mut nbt = RawNbt::new();
        nbt.put_tag("byte", NbtTag::Byte(50));
        nbt.put_tag("short", NbtTag::Short(50));
        nbt.put_tag("int", NbtTag::Int(50));
        nbt.put_tag("long", NbtTag::Long(50));
        nbt.put_tag("float", NbtTag::Float(50.0));
        nbt.put_tag("double", NbtTag::Double(50.0));
        nbt.put_tag("byte_arr", NbtTag::ByteArray(vec![50; 5]));
        nbt.put_tag("str", NbtTag::String("hello".to_string()));
        nbt.put_tag("tag_arr", NbtTag::List { nbt_type: NbtTagType::Float, list: vec![NbtTag::Float(50.0), NbtTag::Float(59.0)] });
        nbt.put_tag("compound", NbtTag::Compound { inner: vec![("hi".to_string(), NbtTag::Byte(0))] });
        nbt.put_tag("int_arr", NbtTag::IntArray(vec![50; 5]));
        nbt.put_tag("long_arr", NbtTag::LongArray(vec![50; 5]));

        nbt.encode_async(&mut writer, &NetEncodeOpts::None).await.expect("NBT encode failed");

        assert_eq!(writer, NBT_BYTES)
    }

    #[test]
    fn test_encode_nbt() {
        let mut writer = Vec::new();
        let mut nbt = RawNbt::new();
        nbt.put_tag("byte", NbtTag::Byte(50));
        nbt.put_tag("short", NbtTag::Short(50));
        nbt.put_tag("int", NbtTag::Int(50));
        nbt.put_tag("long", NbtTag::Long(50));
        nbt.put_tag("float", NbtTag::Float(50.0));
        nbt.put_tag("double", NbtTag::Double(50.0));
        nbt.put_tag("byte_arr", NbtTag::ByteArray(vec![50; 5]));
        nbt.put_tag("str", NbtTag::String("hello".to_string()));
        nbt.put_tag("tag_arr", NbtTag::List { nbt_type: NbtTagType::Float, list: vec![NbtTag::Float(50.0), NbtTag::Float(59.0)] });
        nbt.put_tag("compound", NbtTag::Compound { inner: vec![("hi".to_string(), NbtTag::Byte(0))] });
        nbt.put_tag("int_arr", NbtTag::IntArray(vec![50; 5]));
        nbt.put_tag("long_arr", NbtTag::LongArray(vec![50; 5]));

        nbt.encode(&mut writer, &NetEncodeOpts::None).expect("NBT encode failed");

        assert_eq!(writer, NBT_BYTES)
    }

    #[tokio::test]
    async fn test_decode_nbt_async() {
        let mut reader = Cursor::new(NBT_BYTES);

        let raw_nbt = <RawNbt as NetDecode>::decode_async(&mut reader, &NetDecodeOpts::default()).await.expect("failed to decode nbt");
        let mut comparison = RawNbt::new();
        comparison.put_tag("byte", NbtTag::Byte(50));
        comparison.put_tag("short", NbtTag::Short(50));
        comparison.put_tag("int", NbtTag::Int(50));
        comparison.put_tag("long", NbtTag::Long(50));
        comparison.put_tag("float", NbtTag::Float(50.0));
        comparison.put_tag("double", NbtTag::Double(50.0));
        comparison.put_tag("byte_arr", NbtTag::ByteArray(vec![50; 5]));
        comparison.put_tag("str", NbtTag::String("hello".to_string()));
        comparison.put_tag("tag_arr", NbtTag::List { nbt_type: NbtTagType::Float, list: vec![NbtTag::Float(50.0), NbtTag::Float(59.0)] });
        comparison.put_tag("compound", NbtTag::Compound { inner: vec![("hi".to_string(), NbtTag::Byte(0))] });
        comparison.put_tag("int_arr", NbtTag::IntArray(vec![50; 5]));
        comparison.put_tag("long_arr", NbtTag::LongArray(vec![50; 5]));

        assert_eq!(raw_nbt, comparison)
    }

    #[test]
    fn test_decode_nbt() {
        let mut reader = Cursor::new(NBT_BYTES);

        let raw_nbt = <RawNbt as NetDecode>::decode(&mut reader, &NetDecodeOpts::default()).expect("failed to decode nbt");
        let mut comparison = RawNbt::new();
        comparison.put_tag("byte", NbtTag::Byte(50));
        comparison.put_tag("short", NbtTag::Short(50));
        comparison.put_tag("int", NbtTag::Int(50));
        comparison.put_tag("long", NbtTag::Long(50));
        comparison.put_tag("float", NbtTag::Float(50.0));
        comparison.put_tag("double", NbtTag::Double(50.0));
        comparison.put_tag("byte_arr", NbtTag::ByteArray(vec![50; 5]));
        comparison.put_tag("str", NbtTag::String("hello".to_string()));
        comparison.put_tag("tag_arr", NbtTag::List { nbt_type: NbtTagType::Float, list: vec![NbtTag::Float(50.0), NbtTag::Float(59.0)] });
        comparison.put_tag("compound", NbtTag::Compound { inner: vec![("hi".to_string(), NbtTag::Byte(0))] });
        comparison.put_tag("int_arr", NbtTag::IntArray(vec![50; 5]));
        comparison.put_tag("long_arr", NbtTag::LongArray(vec![50; 5]));

        assert_eq!(raw_nbt, comparison)
    }
}