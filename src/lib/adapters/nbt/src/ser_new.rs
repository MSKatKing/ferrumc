use crate::tag::NbtTag;
use crate::NBTError;
use serde::ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

pub struct RawNbtSerializer;

impl<'a> Serializer for &'a mut RawNbtSerializer {
    type Ok = NbtTag;
    type Error = NBTError;

    type SerializeSeq = RawNbtSeqSerializer<'a>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = RawNbtMapSerializer<'a>;
    type SerializeStruct = RawNbtMapSerializer<'a>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Byte(if v { 1 } else { 0 }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Byte(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Long(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i8(v as _)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i16(v as _)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as _)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Byte(v as _))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::ByteArray(v.to_vec()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(RawNbtSeqSerializer { items: Vec::new(), ser: self })
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(RawNbtMapSerializer { items: Vec::new(), ser: self })
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(RawNbtMapSerializer { items: Vec::new(), ser: self })
    }

    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(NBTError::TypeNotSupported)
    }
}

pub struct RawNbtSeqSerializer<'a> {
    ser: &'a mut RawNbtSerializer,
    items: Vec<NbtTag>,
}

impl<'a> SerializeSeq for RawNbtSeqSerializer<'a> {
    type Ok = NbtTag;
    type Error = NBTError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.items.push(value.serialize(&mut *self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let nbt_type = self.items.get(0).and_then(|tag| Some(tag.tag_type())).unwrap_or_default();
        for tag in &self.items {
            if nbt_type != tag.tag_type() {
                return Err(NBTError::ListTypeMismatch);
            }
        }

        match nbt_type as u8 {
            0 => unreachable!(),
            1 => Ok(NbtTag::ByteArray(self.items.into_iter().map(|tag| {
                let NbtTag::Byte(i) = tag else { unreachable!() };
                i as u8
            }).collect())),
            2 => Ok(NbtTag::List { nbt_type, list: self.items }),
            3 => Ok(NbtTag::IntArray(self.items.into_iter().map(|tag| {
                let NbtTag::Int(i) = tag else { unreachable!() };
                i
            }).collect())),
            4 => Ok(NbtTag::LongArray(self.items.into_iter().map(|tag| {
                let NbtTag::Long(i) = tag else { unreachable!() };
                i
            }).collect())),
            5..=12 => Ok(NbtTag::List { nbt_type, list: self.items }),
            _ => Err(NBTError::InvalidTagType(nbt_type as u8)),
        }
    }
}

pub struct RawNbtMapSerializer<'a> {
    ser: &'a mut RawNbtSerializer,
    items: Vec<(String, NbtTag)>,
}

impl<'a> SerializeMap for RawNbtMapSerializer<'a> {
    type Ok = NbtTag;
    type Error = NBTError;

    fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let NbtTag::String(name) = key.serialize(&mut *self.ser)? else {
            return Err(NBTError::TypeNotSupported);
        };

        self.items.push((name, value.serialize(&mut *self.ser)?));

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound { inner: self.items })
    }
}

impl<'a> SerializeStruct for RawNbtMapSerializer<'a> {
    type Ok = NbtTag;
    type Error = NBTError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.items.push((key.to_string(), value.serialize(&mut *self.ser)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound { inner: self.items })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::raw::tests::COMP_NBT;
    use crate::RawNbt;
    use super::*;

    #[derive(Serialize)]
    struct Foo {
        byte: i8,
        short: i16,
        int: i32,
        long: i64,
        float: f32,
        double: f64,
        byte_arr: Vec<u8>,
        str: String,
        tag_arr: Vec<f32>,
        compound: Bar,
        int_arr: Vec<i32>,
        long_arr: Vec<i64>,
    }

    #[derive(Serialize)]
    struct Bar {
        hi: i8,
    }

    #[test]
    fn test_serialization() {
        let foobar = Foo {
            byte: 50,
            short: 50,
            int: 50,
            long: 50,
            float: 50.0,
            double: 50.0,
            byte_arr: vec![50; 5],
            str: "hello".to_string(),
            tag_arr: vec![50.0, 59.0],
            compound: Bar {
                hi: 0,
            },
            int_arr: vec![50; 5],
            long_arr: vec![50; 5],
        };

        let a = RawNbt::serialize(foobar);

        assert!(a.is_ok());

        let a = a.unwrap();

        assert_eq!(&a, COMP_NBT.deref())
    }
}