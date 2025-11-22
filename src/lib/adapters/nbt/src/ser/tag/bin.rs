use crate::NbtTag;
use crate::ser::NbtSerializationOptions;

impl NbtTag {
    pub fn to_binary(&self, opts: NbtSerializationOptions) -> Vec<u8> {
        match self {
            NbtTag::Byte(v) => v.to_be_bytes().to_vec(),
            NbtTag::Short(v) => v.to_be_bytes().to_vec(),
            NbtTag::Int(v) => v.to_be_bytes().to_vec(),
            NbtTag::Long(v) => v.to_be_bytes().to_vec(),
            NbtTag::Float(v) => v.to_be_bytes().to_vec(),
            NbtTag::Double(v) => v.to_be_bytes().to_vec(),
            NbtTag::ByteArray(arr) => {
                let mut buf = Vec::with_capacity(arr.len() + 4);
                buf.extend_from_slice(&(arr.len() as u32).to_be_bytes());
                buf.extend_from_slice(&arr);
                buf
            },
            NbtTag::String(str) => {
                let mut buf = Vec::with_capacity(str.len() + 2);
                buf.extend_from_slice(&(str.len() as u16).to_be_bytes());
                buf.extend_from_slice(str.as_bytes());
                buf
            },
            NbtTag::List { id, tags } => {
                let mut buf = Vec::with_capacity(5);
                buf.push(*id);
                buf.extend_from_slice(&(tags.len() as u32).to_be_bytes());
                buf.extend_from_slice(tags.iter().map(|tag| tag.to_binary(opts)).flatten().collect::<Vec<_>>().as_slice());
                buf
            },
            NbtTag::Compound(tags) => {
                let mut buf = Vec::new();
                for (name, tag) in tags {
                    buf.push(tag.get_id());
                    buf.extend_from_slice(&(name.len() as u16).to_be_bytes());
                    buf.extend_from_slice(name.as_bytes());
                    buf.extend_from_slice(&tag.to_binary(opts));
                }
                buf.push(0);
                buf
            },
            NbtTag::IntArray(arr) => {
                let mut buf = Vec::with_capacity(arr.len() * size_of::<i32>() + 4);
                buf.extend_from_slice(&(arr.len() as u32).to_be_bytes());
                buf.extend_from_slice(&arr.into_iter().map(|&i| i.to_be_bytes()).flatten().collect::<Vec<_>>());
                buf
            },
            NbtTag::LongArray(arr) => {
                let mut buf = Vec::with_capacity(arr.len() * size_of::<i64>() + 4);
                buf.extend_from_slice(&(arr.len() as u32).to_be_bytes());
                buf.extend_from_slice(&arr.into_iter().map(|&i| i.to_be_bytes()).flatten().collect::<Vec<_>>());
                buf
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NbtTag;
    use crate::ser::NbtSerializationOptions;

    const DATA: [u8; 117] = [
        0x01, 0x00, 0x01, 0x61, 0x00, 0x0B, 0x00, 0x01, 0x62, 0x00, 0x00, 0x00,
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


    #[test]
    fn test_bin_serialization() {
        let tag = NbtTag::Compound(vec![
            ("a".to_string(), NbtTag::Byte(0)),
            ("b".to_string(), NbtTag::IntArray(vec![5; 10])),
            ("c".to_string(), NbtTag::Compound(vec![
                ("a".to_string(), NbtTag::Byte(0)),
                ("b".to_string(), NbtTag::IntArray(vec![1; 10])),
                ("c".to_string(), NbtTag::Compound(vec![]))
            ])),
        ]);

        assert_eq!(tag.to_binary(NbtSerializationOptions::Network), DATA)
    }
}