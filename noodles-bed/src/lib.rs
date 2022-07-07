#![allow(missing_docs)]

//! **noodles-bed** handles the reading and writing of the BED (Browser Extensible Data) format.

mod reader;
pub mod record;
mod writer;

pub use self::{reader::Reader, record::Record, writer::Writer}; 

// SerDe
mod de;
mod error;
mod ser;

//pub use de::{from_str, Deserializer};
pub use de::{from_reader, from_str};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};