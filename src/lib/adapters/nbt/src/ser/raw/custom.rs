use crate::NbtTag;
use serde::ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant};
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use crate::ser::raw::NbtSerializationError;

pub struct RawNbtSerializer;

impl<'a> Serializer for &'a mut RawNbtSerializer {
    type Ok = NbtTag;
    type Error = NbtSerializationError;

    type SerializeSeq = RawNbtSeqSerializer<'a>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = RawNbtMapSerializer<'a>;
    type SerializeStruct = RawNbtMapSerializer<'a>;
    type SerializeStructVariant = RawNbtMapSerializer<'a>;

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
        self.serialize_i8(v as _)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::String(v.to_owned()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::ByteArray(v.to_owned()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(NbtSerializationError::TypeNotSupported)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(NbtSerializationError::TypeNotSupported)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(vec![]))
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(RawNbtSeqSerializer { ser: self, list: vec![] })
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(NbtSerializationError::TypeNotSupported)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(NbtSerializationError::TypeNotSupported)
    }

    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(NbtSerializationError::TypeNotSupported)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(RawNbtMapSerializer { ser: self, map: HashMap::new() })
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(RawNbtMapSerializer { ser: self, map: HashMap::new() })
    }

    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(RawNbtMapSerializer { ser: self, map: HashMap::new() })
    }
}

pub struct RawNbtSeqSerializer<'a> {
    ser: &'a mut RawNbtSerializer,
    list: Vec<NbtTag>,
}

impl<'a> SerializeSeq for RawNbtSeqSerializer<'a> {
    type Ok = NbtTag;
    type Error = NbtSerializationError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.list.push(value.serialize(&mut *self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.list.len() > 0 {
            let id = self.list[0].get_id();
            for tag in &self.list[1..] {
                if tag.get_id() != id {
                    return Err(NbtSerializationError::InvalidList);
                }
            }

            match id {
                1 => {
                    let mut arr = Vec::with_capacity(self.list.len());
                    for tag in self.list {
                        let NbtTag::Byte(v) = tag else {
                            unreachable!()
                        };

                        arr.push(v as u8);
                    }

                    Ok(NbtTag::ByteArray(arr))
                },
                3 => {
                    let mut arr = Vec::with_capacity(self.list.len());
                    for tag in self.list {
                        let NbtTag::Int(v) = tag else {
                            unreachable!()
                        };

                        arr.push(v);
                    }

                    Ok(NbtTag::IntArray(arr))
                },
                4 => {
                    let mut arr = Vec::with_capacity(self.list.len());
                    for tag in self.list {
                        let NbtTag::Long(v) = tag else {
                            unreachable!()
                        };

                        arr.push(v);
                    }

                    Ok(NbtTag::LongArray(arr))
                },
                13.. => unreachable!(),
                id => Ok(NbtTag::List { id, tags: self.list }),
            }
        } else {
            Ok(NbtTag::List { id: 0, tags: vec![] })
        }
    }
}

pub struct RawNbtMapSerializer<'a> {
    ser: &'a mut RawNbtSerializer,
    map: HashMap<String, NbtTag>,
}

impl<'a> SerializeMap for RawNbtMapSerializer<'a> {
    type Ok = NbtTag;
    type Error = NbtSerializationError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(&mut self, key: &K, value: &V) -> Result<(), Self::Error> {
        let NbtTag::String(key) = key.serialize(&mut *self.ser)? else {
            return Err(NbtSerializationError::InvalidMapKey);
        };

        self.map.insert(key.to_string(), value.serialize(&mut *self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(self.map.into_iter().collect()))
    }
}

impl<'a> SerializeStruct for RawNbtMapSerializer<'a> {
    type Ok = NbtTag;
    type Error = NbtSerializationError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        self.map.insert(key.to_string(), value.serialize(&mut *self.ser)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(self.map.into_iter().collect()))
    }
}

impl<'a> SerializeStructVariant for RawNbtMapSerializer<'a> {
    type Ok = NbtTag;
    type Error = NbtSerializationError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        <Self as SerializeStruct>::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeStruct>::end(self)
    }
}