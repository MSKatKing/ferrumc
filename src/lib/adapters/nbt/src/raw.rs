use std::collections::HashMap;
use crate::NbtTag;

pub struct RawNbt {
    root: HashMap<String, NbtTag>,
}

impl RawNbt {
    pub fn new() -> RawNbt {
        Self { root: HashMap::new() }
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