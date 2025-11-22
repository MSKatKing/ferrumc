#[macro_export]
macro_rules! nbt_compound {
    ($($name:expr => $tag:expr),*$(,)?) => {
        $crate::tag::NbtTag::Compound(vec![
            $(
                ($name.to_string(), $tag),
            )*
        ])
    };
}

#[macro_export]
macro_rules! nbt_byte {
    ($byte:expr) => {
        $crate::tag::NbtTag::Byte($byte)
    };
}

#[macro_export]
macro_rules! nbt_short {
    ($short:expr) => {
        $crate::tag::NbtTag::Short($short)
    };
}

#[macro_export]
macro_rules! nbt_int {
    ($int:expr) => {
        $crate::tag::NbtTag::Int($int)
    }
}

#[macro_export]
macro_rules! nbt_long {
    ($long:expr) => {
        $crate::tag::NbtTag::Long($long)
    }
}

#[macro_export]
macro_rules! nbt_string {
    ($string:expr) => {
        $crate::tag::NbtTag::String($string.into())
    }
}

#[macro_export]
macro_rules! nbt_byte_array {
    ($($byte:expr),*$(,)?) => {
        $crate::tag::NbtTag::ByteArray(vec![$($byte,)*])
    };
    ($byte:expr; $count:expr) => {
        $crate::tag::NbtTag::ByteArray(vec![$byte; $count])
    }
}

#[macro_export]
macro_rules! nbt_int_array {
    ($($int:expr),*$(,)?) => {
        $crate::tag::NbtTag::IntArray(vec![$($int,)*])
    };
    ($int:expr; $count:expr) => {
        $crate::tag::NbtTag::IntArray(vec![$int; $count])
    }
}

#[macro_export]
macro_rules! nbt_long_array {
    ($($long:expr),*$(,)?) => {
        $crate::tag::NbtTag::LongArray(vec![$($long,)*])
    };
    ($long:expr; $count:expr) => {
        $crate::tag::NbtTag::LongArray(vec![$long; $count])
    }
}

#[macro_export]
macro_rules! nbt_list {
    ($($item:expr),*$(,)?) => {
        $crate::tag::NbtTag::List(vec![$($item,)*])
    };
}

#[cfg(test)]
mod tests {
    use crate::NbtTag;

    #[test]
    fn test_nbt_macros() {
        let tag = NbtTag::Compound(vec![
            ("a".to_string(), NbtTag::Byte(0)),
            ("b".to_string(), NbtTag::IntArray(vec![5; 10])),
            ("c".to_string(), NbtTag::Compound(vec![
                ("a".to_string(), NbtTag::Byte(0)),
                ("b".to_string(), NbtTag::IntArray(vec![1; 10])),
                ("c".to_string(), NbtTag::Compound(vec![]))
            ])),
        ]);

        let macro_tag = nbt_compound!(
            "a" => nbt_byte!(0),
            "b" => nbt_int_array!(5; 10),
            "c" => nbt_compound!(
                "a" => nbt_byte!(0),
                "b" => nbt_int_array!(1; 10),
                "c" => nbt_compound!(),
            ),
        );

        assert_eq!(tag, macro_tag);
    }
}