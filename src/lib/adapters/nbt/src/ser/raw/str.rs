use std::fmt::{Display, Formatter};
use crate::raw::RawNbt;

impl Display for RawNbt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, (name, tag)) in self.root.iter().enumerate() {
            write!(f, "\"{}\":{tag}{}", name.replace("\"", "\\\""), if i < self.root.len() - 1 { "," } else { "" })?;
        }
        write!(f, "}}")
    }
}