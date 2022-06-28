use std::io::{Write};

// TODO: Remove Sized marker trait and refactor for streaming?
pub trait BedSerialize: Sized {
    fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String>;
}