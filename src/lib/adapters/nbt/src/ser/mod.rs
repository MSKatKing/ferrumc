pub mod tag;
mod raw;

#[derive(Debug, Copy, Clone)]
pub enum NbtSerializationOptions {
    LittleEndian,
    Network,
}