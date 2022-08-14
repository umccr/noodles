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

pub use de::{from_bytes, Record3Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, to_string, Record3Serializer};
