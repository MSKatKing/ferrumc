use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
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
        id: u8,
        tags: Vec<NbtTag>,
    },
    Compound(BTreeMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    pub const BYTE_ID: u8 = 1;
    pub const SHORT_ID: u8 = 2;
    pub const INT_ID: u8 = 3;
    pub const LONG_ID: u8 = 4;
    pub const FLOAT_ID: u8 = 5;
    pub const DOUBLE_ID: u8 = 6;
    pub const BYTE_ARRAY_ID: u8 = 7;
    pub const STRING_ID: u8 = 8;
    pub const LIST_ID: u8 = 9;
    pub const COMPOUND_ID: u8 = 10;
    pub const INT_ARRAY_ID: u8 = 11;
    pub const LONG_ARRAY_ID: u8 = 12;

    pub fn get_id(&self) -> u8 {
        match self {
            &NbtTag::Byte(_) => NbtTag::BYTE_ID,
            &NbtTag::Short(_) => NbtTag::SHORT_ID,
            &NbtTag::Int(_) => NbtTag::INT_ID,
            &NbtTag::Long(_) => NbtTag::LONG_ID,
            &NbtTag::Float(_) => NbtTag::FLOAT_ID,
            &NbtTag::Double(_) => NbtTag::DOUBLE_ID,
            &NbtTag::ByteArray(_) => NbtTag::BYTE_ARRAY_ID,
            &NbtTag::String(_) => NbtTag::STRING_ID,
            &NbtTag::List { .. } => NbtTag::LIST_ID,
            &NbtTag::Compound(_) => NbtTag::COMPOUND_ID,
            &NbtTag::IntArray(_) => NbtTag::INT_ARRAY_ID,
            &NbtTag::LongArray(_) => NbtTag::LONG_ARRAY_ID,
        }
    }
}