use std::collections::HashMap;
use serde::Serialize;
use crate::NbtTag;
use crate::ser::raw::{NbtSerializationError, RawNbtSerializer};

pub struct RawNbt {
    pub(crate) root: HashMap<String, NbtTag>,
}

impl RawNbt {
    pub fn new() -> RawNbt {
        Self { root: HashMap::new() }
    }

    pub fn from_data<T: Serialize>(data: &T) -> Result<Self, NbtSerializationError> {
        if let NbtTag::Compound(tags) = data.serialize(&mut RawNbtSerializer) {
            Ok(Self { root: tags.into_iter().collect() })
        } else {
            Err(NbtSerializationError::TypeNotSupported)
        }
    }

    pub fn from_tag(tag: NbtTag) -> Option<RawNbt> {
        let NbtTag::Compound(tags) = tag else {
            return None;
        };

        let mut root: HashMap<String, NbtTag> = HashMap::new();

        for (name, tag) in tags {
            root.insert(name, tag);
        }

        Some(Self { root })
    }
}