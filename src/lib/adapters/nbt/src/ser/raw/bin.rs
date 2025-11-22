use crate::NbtTag;
use crate::raw::RawNbt;
use crate::ser::NbtSerializationOptions;

impl RawNbt {
    pub fn to_binary(&self, opts: NbtSerializationOptions) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(NbtTag::COMPOUND_ID);
        match opts {
            NbtSerializationOptions::Network => {},
            _ => buf.extend_from_slice(&[0, 0]),
        }

        for (name, tag) in self.root.iter() {
            buf.push(tag.get_id());
            buf.extend_from_slice(&(name.len() as u16).to_be_bytes());
            buf.extend_from_slice(name.as_bytes());
            buf.extend_from_slice(&tag.to_binary(opts));
        }

        buf.push(0);

        buf
    }
}