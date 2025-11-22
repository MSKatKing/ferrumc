use crate::ser::raw::{NbtSerializationError, RawNbtSerializer};
use crate::NbtTag;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct RawNbt {
    pub(crate) root: BTreeMap<String, NbtTag>,
}

impl RawNbt {
    pub fn new() -> RawNbt {
        Self { root: BTreeMap::new() }
    }

    pub fn from_data<T: Serialize>(data: &T) -> Result<Self, NbtSerializationError> {
        if let NbtTag::Compound(tags) = data.serialize(&mut RawNbtSerializer)? {
            Ok(Self { root: tags.into_iter().collect() })
        } else {
            Err(NbtSerializationError::TypeNotSupported)
        }
    }

    pub fn from_tag(tag: NbtTag) -> Option<RawNbt> {
        let NbtTag::Compound(tags) = tag else {
            return None;
        };

        let mut root = BTreeMap::new();

        for (name, tag) in tags {
            root.insert(name, tag);
        }

        Some(Self { root })
    }

    pub fn from_tag_unchecked(tag: NbtTag) -> RawNbt {
        Self::from_tag(tag).expect("tag should be NbtTag::Compound")
    }
}