use std::fmt::{Display, Formatter};
use crate::tag::NbtTag;

/// Returns the NbtTag as SNBT
impl Display for NbtTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn fmt_arr<T: Display>(arr: &Vec<T>, f: &mut Formatter<'_>) -> std::fmt::Result {
            for (i, item) in arr.iter().enumerate() {
                write!(f, "{}{}", item, if i < arr.len() - 1 { "," } else { "" })?;
            }
            write!(f, "]")
        }

        match self {
            NbtTag::Byte(i) => write!(f, "{}", i),
            NbtTag::Short(i) => write!(f, "{}", i),
            NbtTag::Int(i) => write!(f, "{}", i),
            NbtTag::Long(i) => write!(f, "{}", i),
            NbtTag::Float(i) => write!(f, "{}", i),
            NbtTag::Double(i) => write!(f, "{}", i),
            NbtTag::String(i) => write!(f, "\"{}\"", i.replace("\"", "\\\"")),
            NbtTag::ByteArray(arr) => {
                write!(f, "[B;")?;
                fmt_arr(arr, f)
            },
            NbtTag::IntArray(arr) => {
                write!(f, "[I;")?;
                fmt_arr(arr, f)
            },
            NbtTag::LongArray(arr) => {
                write!(f, "[L;")?;
                fmt_arr(arr, f)
            },
            NbtTag::List { tags, .. } => {
                write!(f, "[")?;
                fmt_arr(tags, f)
            },
            NbtTag::Compound(tags) => {
                write!(f, "{{")?;
                for (i, (name, tag)) in tags.iter().enumerate() {
                    write!(f, "\"{}\":{}{}", name, tag, if i < tags.len() - 1 { "," } else { "" })?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{nbt_byte, nbt_compound, nbt_int_array};

    #[test]
    fn test_snbt_serialization() {
        let tag = nbt_compound!(
            "a" => nbt_byte!(0),
            "b" => nbt_int_array!(5; 10),
            "c" => nbt_compound!(
                "a" => nbt_byte!(0),
                "b" => nbt_int_array!(1; 10),
                "c" => nbt_compound!()
            )
        );

        assert_eq!(tag.to_string(), "{\"a\":0,\"b\":[I;5,5,5,5,5,5,5,5,5,5],\"c\":{\"a\":0,\"b\":[I;1,1,1,1,1,1,1,1,1,1],\"c\":{}}}")
    }
}