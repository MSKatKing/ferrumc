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
    Compound(Vec<(String, NbtTag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    const END_ID: u8 = 0;
    const BYTE_ID: u8 = 1;
    const SHORT_ID: u8 = 2;
    const INT_ID: u8 = 3;
    const LONG_ID: u8 = 4;
    const FLOAT_ID: u8 = 5;
    const DOUBLE_ID: u8 = 6;
    const BYTE_ARRAY_ID: u8 = 7;
    const STRING_ID: u8 = 8;
    const LIST_ID: u8 = 9;
    const COMPOUND_ID: u8 = 10;
    const INT_ARRAY_ID: u8 = 11;
    const LONG_ARRAY_ID: u8 = 12;
}