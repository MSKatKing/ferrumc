pub mod tag;

#[derive(Debug, Copy, Clone)]
pub enum NbtSerializationOptions {
    LittleEndian,
    Network,
}