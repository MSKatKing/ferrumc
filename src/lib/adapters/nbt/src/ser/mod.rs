pub mod tag;
pub mod raw;

#[derive(Debug, Copy, Clone)]
pub enum NbtSerializationOptions {
    LittleEndian,
    Network,
}