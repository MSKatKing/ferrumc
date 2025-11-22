use crate::de::NbtDeserializationError;
use crate::NbtTag;
use serde::de::{DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserializer;

pub struct RawNbtDeserializer {
    tag: NbtTag,
}

impl RawNbtDeserializer {
    pub fn new(tag: NbtTag) -> RawNbtDeserializer {
        RawNbtDeserializer { tag }
    }
}

impl<'a> Deserializer<'a> for RawNbtDeserializer {
    type Error = NbtDeserializationError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>,
    {
        match self.tag {
            NbtTag::Byte(v) => visitor.visit_i8(v),
            NbtTag::Short(v) => visitor.visit_i16(v),
            NbtTag::Int(v) => visitor.visit_i32(v),
            NbtTag::Long(v) => visitor.visit_i64(v),
            NbtTag::Float(v) => visitor.visit_f32(v),
            NbtTag::Double(v) => visitor.visit_f64(v),
            NbtTag::ByteArray(v) => visitor.visit_byte_buf(v),
            NbtTag::String(v) => visitor.visit_string(v),
            NbtTag::List { tags, .. } => visitor.visit_seq(RawNbtSeqDeserializer { tags: tags.into_iter() }),
            NbtTag::Compound(tags) => visitor.visit_map(RawNbtMapDeserializer {
                entries: tags.into_iter(),
                value: None
            }),
            NbtTag::IntArray(arr) => visitor.visit_seq(RawNbtSeqDeserializer {
                tags: arr.iter()
                    .map(|v| NbtTag::Int(*v))
            }),
            NbtTag::LongArray(arr) => visitor.visit_seq(RawNbtSeqDeserializer {
                tags: arr.iter()
                    .map(|v| NbtTag::Long(*v))
            }),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Byte(v) => visitor.visit_bool(v > 0),
            _ => Err(NbtDeserializationError::type_mismatch("Byte", &self.tag)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Byte(v) => visitor.visit_i8(v),
            _ => Err(NbtDeserializationError::type_mismatch("Byte", &self.tag)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Short(v) => visitor.visit_i16(v),
            _ => Err(NbtDeserializationError::type_mismatch("Short", &self.tag)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Int(v) => visitor.visit_i32(v),
            _ => Err(NbtDeserializationError::type_mismatch("Int", &self.tag)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Long(v) => visitor.visit_i64(v),
            _ => Err(NbtDeserializationError::type_mismatch("Long", &self.tag)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Byte(v) => visitor.visit_u8(v as _),
            _ => Err(NbtDeserializationError::type_mismatch("Byte", &self.tag)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Short(v) => visitor.visit_u16(v as _),
            _ => Err(NbtDeserializationError::type_mismatch("Short", &self.tag)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Int(v) => visitor.visit_u32(v as _),
            _ => Err(NbtDeserializationError::type_mismatch("Int", &self.tag)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Long(v) => visitor.visit_u32(v as _),
            _ => Err(NbtDeserializationError::type_mismatch("Long", &self.tag)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Float(v) => visitor.visit_f32(v),
            _ => Err(NbtDeserializationError::type_mismatch("Float", &self.tag)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Double(v) => visitor.visit_f64(v),
            _ => Err(NbtDeserializationError::type_mismatch("Double", &self.tag)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Byte(v) => visitor.visit_char(v as u8 as _),
            _ => Err(NbtDeserializationError::type_mismatch("Byte", &self.tag)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::String(v) => visitor.visit_str(v.as_str()),
            _ => Err(NbtDeserializationError::type_mismatch("String", &self.tag)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::String(v) => visitor.visit_string(v),
            _ => Err(NbtDeserializationError::type_mismatch("String", &self.tag)),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::ByteArray(v) => visitor.visit_bytes(v.as_slice()),
            _ => Err(NbtDeserializationError::type_mismatch("ByteArray", &self.tag)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::ByteArray(v) => visitor.visit_byte_buf(v),
            _ => Err(NbtDeserializationError::type_mismatch("ByteArray", &self.tag)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        Err(NbtDeserializationError::TypeNotSupported)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Compound(v) => if v.is_empty() { visitor.visit_unit() } else { Err(NbtDeserializationError::NonEmptyUnitStruct) },
            _ => Err(NbtDeserializationError::type_mismatch("Compound", &self.tag)),
        }
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::ByteArray(arr) => visitor.visit_seq(RawNbtSeqDeserializer {
                tags: arr.into_iter()
                    .map(|v| NbtTag::Byte(v as _))
            }),
            NbtTag::IntArray(arr) => visitor.visit_seq(RawNbtSeqDeserializer {
                tags: arr.into_iter()
                    .map(|v| NbtTag::Int(v))
            }),
            NbtTag::LongArray(arr) => visitor.visit_seq(RawNbtSeqDeserializer {
                tags: arr.into_iter()
                    .map(|v| NbtTag::Long(v))
            }),
            NbtTag::List { tags, .. } => visitor.visit_seq(RawNbtSeqDeserializer { tags: tags.into_iter() }),
            _ => Err(NbtDeserializationError::type_mismatch("List", &self.tag)),
        }
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        Err(NbtDeserializationError::TypeNotSupported)
    }

    fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        Err(NbtDeserializationError::TypeNotSupported)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Compound(v) => visitor.visit_map(RawNbtMapDeserializer { entries: v.into_iter(), value: None }),
            _ => Err(NbtDeserializationError::type_mismatch("Compound", &self.tag)),
        }
    }

    fn deserialize_struct<V>(self, _: &'static str, _: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.tag {
            NbtTag::Compound(v) => visitor.visit_map(RawNbtMapDeserializer { entries: v.into_iter(), value: None }),
            _ => Err(NbtDeserializationError::type_mismatch("Compound", &self.tag)),
        }
    }

    fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        visitor.visit_enum(RawNbtEnumDeserializer { value: self.tag })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        self.deserialize_any(visitor)
    }
}

pub struct RawNbtSeqDeserializer<T: Iterator<Item=NbtTag>> {
    tags: T,
}

impl<'a, Iter: Iterator<Item=NbtTag>> SeqAccess<'a> for RawNbtSeqDeserializer<Iter> {
    type Error = NbtDeserializationError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'a>,
    {
        if let Some(v) = self.tags.next() {
            seed.deserialize(RawNbtDeserializer::new(v)).map(Some)
        } else {
            Ok(None)
        }
    }
}

pub struct RawNbtMapDeserializer {
    entries: std::collections::btree_map::IntoIter<String, NbtTag>,
    value: Option<NbtTag>,
}

impl<'a> MapAccess<'a> for RawNbtMapDeserializer {
    type Error = NbtDeserializationError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'a>,
    {
        match self.entries.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.as_str().into_deserializer()).map(Some)
            },
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'a>,
    {
        seed.deserialize(RawNbtDeserializer::new(self.value.take().unwrap()))
    }
}

struct RawNbtEnumDeserializer {
    value: NbtTag,
}

impl<'a> EnumAccess<'a> for RawNbtEnumDeserializer {
    type Error = NbtDeserializationError;
    type Variant = RawNbtEnumVariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'a>,
    {
        let v = seed.deserialize("".into_deserializer())?;
        Ok((v, RawNbtEnumVariantDeserializer { value: self.value }))
    }
}

struct RawNbtEnumVariantDeserializer {
    value: NbtTag,
}

impl<'a> VariantAccess<'a> for RawNbtEnumVariantDeserializer {
    type Error = NbtDeserializationError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'a>,
    {
        seed.deserialize(RawNbtDeserializer { tag: self.value })
    }

    fn tuple_variant<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        Err(NbtDeserializationError::TypeNotSupported)
    }

    fn struct_variant<V>(self, _: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'a>
    {
        match self.value {
            NbtTag::Compound(v) => visitor.visit_map(RawNbtMapDeserializer { entries: v.into_iter(), value: None }),
            _ => Err(NbtDeserializationError::type_mismatch("Compound", &self.value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::de::raw::custom::RawNbtDeserializer;
    use crate::{nbt_byte, nbt_compound, nbt_int_array};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo {
        a: i8,
        b: Vec<i32>,
        c: Bar,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Bar {
        a: i8,
        b: Vec<i32>,
        c: HashMap<String, i32>,
    }

    #[test]
    fn test_deserialize() {
        let tag = nbt_compound!(
            "a" => nbt_byte!(0),
            "b" => nbt_int_array!(5; 10),
            "c" => nbt_compound!(
                "a" => nbt_byte!(0),
                "b" => nbt_int_array!(1; 10),
                "c" => nbt_compound!()
            )
        );

        let test = Foo {
            a: 0,
            b: vec![5; 10],
            c: Bar {
                a: 0,
                b: vec![1; 10],
                c: HashMap::new(),
            }
        };

        let a = Foo::deserialize(RawNbtDeserializer { tag });

        assert!(a.is_ok());

        let a = a.unwrap();

        assert_eq!(a, test)
    }
}