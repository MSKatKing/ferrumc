use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWrite};
use tokio::io::{AsyncRead, AsyncWriteExt};

#[derive(Debug, PartialEq)]
pub enum NbtTag {
    End,
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
            &NbtTag::End => NbtTagType::End,
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

impl NetEncode for NbtTag {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        match self {
            Self::End => {}
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
                writer.write_all(&(str.len() as u16).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(str.as_bytes()).map_err(NetEncodeError::from)?;
            },
            Self::List { nbt_type, list } => {
                writer.write_all(&(*nbt_type as u8).to_be_bytes()).map_err(NetEncodeError::from)?;
                writer.write_all(&(list.len() as u32).to_be_bytes()).map_err(NetEncodeError::from)?;
                for tag in list {
                    <Self as NetEncode>::encode(tag, writer, opts)?;
                }
            },
            Self::Compound { inner } => {
                for (tag_name, tag) in inner.iter() {
                    writer.write_all(&(tag.tag_type() as u8).to_be_bytes()).map_err(NetEncodeError::from)?;
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
        match self {
            Self::End => {},
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
                writer.write_all(&(str.len() as u16).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(str.as_bytes()).await.map_err(NetEncodeError::from)?;
            },
            Self::List { nbt_type, list } => {
                writer.write_all(&(*nbt_type as u8).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                writer.write_all(&(list.len() as u32).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                for tag in list {
                    Box::pin(tag.encode_async(writer, opts)).await?;
                }
            },
            Self::Compound { inner } => {
                for (tag_name, tag) in inner.iter() {
                    writer.write_all(&(tag.tag_type() as u8).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                    writer.write_all(&(tag_name.len() as u16).to_be_bytes()).await.map_err(NetEncodeError::from)?;
                    writer.write_all(tag_name.as_bytes()).await.map_err(NetEncodeError::from)?;
                    Box::pin(tag.encode_async(writer, opts)).await?;
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

pub(crate) struct TagEntry(pub String, pub NbtTag);

impl NetDecode for TagEntry {
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        fn decode_tag<R: Read>(id: u8, reader: &mut R, opts: &NetDecodeOpts) -> Result<NbtTag, NetDecodeError> {
            match id {
                0 => Ok(NbtTag::End),
                1 => Ok(NbtTag::Byte(i8::decode(reader, opts)?)),
                2 => Ok(NbtTag::Short(i16::decode(reader, opts)?)),
                3 => Ok(NbtTag::Int(i32::decode(reader, opts)?)),
                4 => Ok(NbtTag::Long(i64::decode(reader, opts)?)),
                5 => Ok(NbtTag::Float(f32::decode(reader, opts)?)),
                6 => Ok(NbtTag::Double(f64::decode(reader, opts)?)),
                7 => {
                    let len = u32::decode(reader, opts)?;
                    let mut buf = vec![0u8; len as usize];
                    reader.read_exact(&mut buf).map_err(NetDecodeError::from)?;
                    Ok(NbtTag::ByteArray(buf))
                },
                8 => {
                    let len = u16::decode(reader, opts)?;
                    let mut buf = vec![0u8; len as usize];
                    reader.read_exact(&mut buf).map_err(NetDecodeError::from)?;
                    Ok(NbtTag::String(String::from_utf8(buf)?))
                }
                9 => {
                    let nbt_type = u8::decode(reader, opts)?;
                    let len = u32::decode(reader, opts)?;
                    let mut list = Vec::with_capacity(len as usize);

                    for _ in 0..len {
                        list.push(decode_tag(nbt_type, reader, opts)?);
                    }

                    Ok(NbtTag::List { nbt_type: NbtTagType::from(nbt_type), list })
                },
                10 => {
                    let mut tags = Vec::new();

                    loop {
                        let TagEntry(name, tag) = TagEntry::decode(reader, opts)?;
                        if tag.tag_type() == NbtTagType::End { break; }

                        tags.push((name, tag));
                    }

                    Ok(NbtTag::Compound { inner: tags })
                },
                11 => {
                    let len = u32::decode(reader, opts)?;
                    let mut buf = Vec::with_capacity(len as _);

                    for _ in 0..len {
                        buf.push(i32::decode(reader, opts)?);
                    }

                    Ok(NbtTag::IntArray(buf))
                },
                12 => {
                    let len = u32::decode(reader, opts)?;
                    let mut buf = Vec::with_capacity(len as _);

                    for _ in 0..len {
                        buf.push(i64::decode(reader, opts)?);
                    }

                    Ok(NbtTag::LongArray(buf))
                },
                id => Err(NetDecodeError::ExternalError(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, id.to_string()))))
            }
        }

        let id = u8::decode(reader, opts)?;
        if id == 0 { return Ok(TagEntry(String::new(), NbtTag::End)); }

        let name_len = u16::decode(reader, opts)?;
        let mut name = vec![0; name_len as usize];
        reader.read_exact(&mut name).map_err(NetDecodeError::from)?;
        let name = String::from_utf8(name).map_err(NetDecodeError::from)?;

        Ok(TagEntry(name, decode_tag(id, reader, opts)?))
    }

    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        async fn decode_tag<R: AsyncRead + Unpin>(id: u8, reader: &mut R, opts: &NetDecodeOpts) -> Result<NbtTag, NetDecodeError> {
            match id {
                0 => Ok(NbtTag::End),
                1 => Ok(NbtTag::Byte(i8::decode_async(reader, opts).await?)),
                2 => Ok(NbtTag::Short(i16::decode_async(reader, opts).await?)),
                3 => Ok(NbtTag::Int(i32::decode_async(reader, opts).await?)),
                4 => Ok(NbtTag::Long(i64::decode_async(reader, opts).await?)),
                5 => Ok(NbtTag::Float(f32::decode_async(reader, opts).await?)),
                6 => Ok(NbtTag::Double(f64::decode_async(reader, opts).await?)),
                7 => {
                    let len = u32::decode_async(reader, opts).await?;
                    let mut buf = vec![0u8; len as usize];
                    reader.read_exact(&mut buf).await.map_err(NetDecodeError::from)?;
                    Ok(NbtTag::ByteArray(buf))
                },
                8 => {
                    let len = u16::decode_async(reader, opts).await?;
                    let mut buf = vec![0u8; len as usize];
                    reader.read_exact(&mut buf).await.map_err(NetDecodeError::from)?;
                    Ok(NbtTag::String(String::from_utf8(buf)?))
                }
                9 => {
                    let nbt_type = u8::decode_async(reader, opts).await?;
                    let len = u32::decode_async(reader, opts).await?;
                    let mut list = Vec::with_capacity(len as usize);

                    for _ in 0..len {
                        list.push(Box::pin(decode_tag(nbt_type, reader, opts)).await?);
                    }

                    Ok(NbtTag::List { nbt_type: NbtTagType::from(nbt_type), list })
                },
                10 => {
                    let mut tags = Vec::new();

                    loop {
                        let TagEntry(name, tag) = Box::pin(TagEntry::decode_async(reader, opts)).await?;
                        if tag.tag_type() == NbtTagType::End { break; }

                        tags.push((name, tag));
                    }

                    Ok(NbtTag::Compound { inner: tags })
                },
                11 => {
                    let len = u32::decode_async(reader, opts).await?;
                    let mut buf = Vec::with_capacity(len as _);

                    for _ in 0..len {
                        buf.push(i32::decode_async(reader, opts).await?);
                    }

                    Ok(NbtTag::IntArray(buf))
                },
                12 => {
                    let len = u32::decode_async(reader, opts).await?;
                    let mut buf = Vec::with_capacity(len as _);

                    for _ in 0..len {
                        buf.push(i64::decode_async(reader, opts).await?);
                    }

                    Ok(NbtTag::LongArray(buf))
                },
                id => Err(NetDecodeError::ExternalError(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, id.to_string()))))
            }
        }

        let id = u8::decode_async(reader, opts).await?;
        if id == 0 { return Ok(TagEntry(String::new(), NbtTag::End)); }

        let name_len = u16::decode_async(reader, opts).await?;
        let mut name = vec![0; name_len as usize];
        reader.read_exact(&mut name).await.map_err(NetDecodeError::from)?;
        let name = String::from_utf8(name).map_err(NetDecodeError::from)?;

        Ok(TagEntry(name, decode_tag(id, reader, opts).await?))
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Debug, Copy, Default)]
pub enum NbtTagType {
    #[default]
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

impl From<u8> for NbtTagType {
    fn from(value: u8) -> Self {
        assert!(value <= 12); // TODO: implement TryFrom instead of From

        match value {
            0 => Self::End,
            1 => Self::Byte,
            2 => Self::Short,
            3 => Self::Int,
            4 => Self::Long,
            5 => Self::Float,
            6 => Self::Double,
            7 => Self::ByteArray,
            8 => Self::String,
            9 => Self::List,
            10 => Self::Compound,
            11 => Self::IntArray,
            12 => Self::LongArray,
            _ => unreachable!(),
        }
    }
}