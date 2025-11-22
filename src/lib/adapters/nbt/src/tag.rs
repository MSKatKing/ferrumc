use tokio::io::AsyncWriteExt;
use std::io::Write;
use tokio::io::AsyncWrite;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;

pub enum NbtTag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List {
        nbt_type: NbtTagType,
        list: Vec<NbtTag>,
    },
    Compound {
        inner: Vec<(String, NbtTag)>,
    },
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    pub fn tag_type(&self) -> NbtTagType {
        match self {
            &NbtTag::Byte(_) => NbtTagType::Byte,
            &NbtTag::Short(_) => NbtTagType::Short,
            &NbtTag::Int(_) => NbtTagType::Int,
            &NbtTag::Long(_) => NbtTagType::Long,
            &NbtTag::Float(_) => NbtTagType::Float,
            &NbtTag::Double(_) => NbtTagType::Double,
            &NbtTag::ByteArray(_) => NbtTagType::ByteArray,
            &NbtTag::String(_) => NbtTagType::String,
            &NbtTag::List { .. } => NbtTagType::List,
            &NbtTag::Compound { .. } => NbtTagType::Compound,
            &NbtTag::IntArray(_) => NbtTagType::IntArray,
            &NbtTag::LongArray(_) => NbtTagType::LongArray,
        }
    }
}

impl NbtTag {
    async fn encode<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            Self::Byte(i) => writer.write_all(&i.to_be_bytes()).await.map_err(NetEncodeError::from)?,
            Self::Short(i) => writer.write_all(&i.to_be_bytes()).await.map_err(NetEncodeError::from)?,
            Self::Int(i) => writer.write_all(&i.to_be_bytes()).await.map_err(NetEncodeError::from)?,
            Self::Long(i) => writer.write_all(&i.to_be_bytes()).await.map_err(NetEncodeError::from)?,
            Self::Float(i) => writer.write_all(&i.to_be_bytes()).await.map_err(NetEncodeError::from)?,
            Self::Double(i) => writer.write_all(&i.to_be_bytes()).await.map_err(NetEncodeError::from)?,
            Self::ByteArray(arr) => {
                writer.write_all(&(arr.len() as u32).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(&arr).await.map_err(NetEncodeError::from)?;
            },
            Self::String(str) => {
                writer.write_all(&(str.len() as u32).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(str.as_bytes()).await.map_err(NetEncodeError::from)?;
            },
            Self::List { nbt_type, list } => {
                writer.write_all(&(*nbt_type as u8).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(&(list.len() as u32).to_be_bytes()).await.map_err(NetEncodeError::from)?;
            },
            Self::Compound { inner } => {
                for (tag_name, tag) in inner.iter() {
                    writer.write_all(&(self.tag_type() as u8).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                    writer.write_all(&(tag_name.len() as u16).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                    writer.write_all(tag_name.as_bytes()).await.map_err(NetEncodeError::from)?;
                    Box::pin(tag.encode(writer, opts)).await?;
                }
                writer.write_all(&[0]).await.map_err(NetEncodeError::from)?;
            },
            Self::IntArray(arr) => {
                writer.write_all(&(arr.len() as u32).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(&arr.iter().map(|b| b.to_be_bytes()).flatten().collect::<Vec<u8>>()).await.map_err(NetEncodeError::from)?;
            },
            Self::LongArray(arr) => {
                writer.write_all(&(arr.len() as u32).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(&arr.iter().map(|b| b.to_be_bytes()).flatten().collect::<Vec<u8>>()).await.map_err(NetEncodeError::from)?;
            }
        }

        Ok(())
    }
}

impl NetEncode for NbtTag {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            Self::Byte(i) => writer.write_all(&i.to_be_bytes()).map_err(NetEncodeError::from)?,
            Self::Short(i) => writer.write_all(&i.to_be_bytes()).map_err(NetEncodeError::from)?,
            Self::Int(i) => writer.write_all(&i.to_be_bytes()).map_err(NetEncodeError::from)?,
            Self::Long(i) => writer.write_all(&i.to_be_bytes()).map_err(NetEncodeError::from)?,
            Self::Float(i) => writer.write_all(&i.to_be_bytes()).map_err(NetEncodeError::from)?,
            Self::Double(i) => writer.write_all(&i.to_be_bytes()).map_err(NetEncodeError::from)?,
            Self::ByteArray(arr) => {
                writer.write_all(&(arr.len() as u32).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(&arr).map_err(NetEncodeError::from)?;
            },
            Self::String(str) => {
                writer.write_all(&(str.len() as u32).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(str.as_bytes()).map_err(NetEncodeError::from)?;
            },
            Self::List { nbt_type, list } => {
                writer.write_all(&(*nbt_type as u8).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(&(list.len() as u32).to_be_bytes()).map_err(NetEncodeError::from)?;
            },
            Self::Compound { inner } => {
                for (tag_name, tag) in inner.iter() {
                    writer.write_all(&(self.tag_type() as u8).to_be_bytes()).map_err(NetEncodeError::from)?;
                    writer.write_all(&(tag_name.len() as u16).to_be_bytes()).map_err(NetEncodeError::from)?;
                    writer.write_all(tag_name.as_bytes()).map_err(NetEncodeError::from)?;
                    <Self as NetEncode>::encode(tag, writer, opts)?;
                }
                writer.write_all(&[0])?;
            },
            Self::IntArray(arr) => {
                writer.write_all(&(arr.len() as u32).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(&arr.iter().map(|b| b.to_be_bytes()).flatten().collect::<Vec<u8>>()).map_err(NetEncodeError::from)?;
            },
            Self::LongArray(arr) => {
                writer.write_all(&(arr.len() as u32).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(&arr.iter().map(|b| b.to_be_bytes()).flatten().collect::<Vec<u8>>()).map_err(NetEncodeError::from)?;
            }
        }

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.encode(writer, opts).await
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Debug, Copy)]
pub(crate) enum NbtTagType {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}