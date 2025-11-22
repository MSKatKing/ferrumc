use std::fmt::Debug;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::AsyncWrite;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::encode::errors::NetEncodeError;
use crate::RawNbt;
use crate::ser::raw::NbtSerializationError;

pub struct Nbt<T: Serialize + DeserializeOwned> {
    inner: T,
}

impl<T: Serialize + DeserializeOwned + Debug> Debug for Nbt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nbt {{ {:?} }}", self.inner)
    }
}

impl<T: Serialize + DeserializeOwned> From<T> for Nbt<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Serialize + DeserializeOwned> From<RawNbt> for Nbt<T> {
    fn from(_value: RawNbt) -> Self {
        todo!("implement deserializer for RawNbt")
    }
}

impl<T: Serialize + DeserializeOwned> TryFrom<Nbt<T>> for RawNbt {
    type Error = NbtSerializationError;

    fn try_from(value: Nbt<T>) -> Result<Self, Self::Error> {
        Self::from_data(&value.inner)
    }
}

impl<T: Serialize + DeserializeOwned> NetEncode for Nbt<T> {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        RawNbt::from_data(&self.inner).map_err(|err| NetEncodeError::ExternalError(Box::new(err)))?
            .encode(writer, opts)
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        RawNbt::from_data(&self.inner).map_err(|err| NetEncodeError::ExternalError(Box::new(err)))?
            .encode_async(writer, opts).await
    }
}

impl<T: Serialize + DeserializeOwned> Deref for Nbt<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: Serialize + DeserializeOwned> DerefMut for Nbt<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde::{Deserialize, Serialize};
    use ferrumc_macros::NetEncode;
    use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
    use crate::nbt::Nbt;

    const DATA: [u8; 118] = [
        0x0A, 0x01, 0x00, 0x01, 0x61, 0x00, 0x0B, 0x00, 0x01, 0x62, 0x00, 0x00, 0x00,
        0x0A, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x05, 0x0A, 0x00, 0x01, 0x63, 0x01, 0x00, 0x01,
        0x61, 0x00, 0x0B, 0x00, 0x01, 0x62, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x00, 0x01, 0x0A, 0x00, 0x01, 0x63, 0x00, 0x00, 0x00
    ];

    #[derive(Debug, Serialize, Deserialize)]
    struct Foo {
        a: i8,
        b: Vec<i32>,
        c: Bar,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Bar {
        a: i8,
        b: Vec<i32>,
        c: HashMap<String, i32>,
    }
    #[test]
    fn test_net_encode() {
        let foo = Nbt::from(Foo {
            a: 0,
            b: vec![5; 10],
            c: Bar {
                a: 0,
                b: vec![1; 10],
                c: HashMap::new(),
            }
        });

        let mut buf = Vec::new();
        foo.encode(&mut buf, &NetEncodeOpts::default()).unwrap();

        assert_eq!(buf, DATA);
    }

    #[derive(NetEncode)]
    struct TestPacket {
        data: Nbt<Foo>,
    }

    #[test]
    fn test_net_encode_packet() {
        let foo = TestPacket {
            data: Nbt::from(
                Foo {
                    a: 0,
                    b: vec![5; 10],
                    c: Bar {
                        a: 0,
                        b: vec![1; 10],
                        c: HashMap::new(),
                    }
                }
            ),
        };

        let mut buf = Vec::new();
        foo.encode(&mut buf, &NetEncodeOpts::default()).unwrap();
        assert_eq!(buf, DATA)
    }
}