#![warn(missing_docs)]

//! **noodles-bed** handles the reading and writing of the BED (Browser Extensible Data) format.

mod reader;
pub mod record;
pub mod serialize;
pub mod ser;
mod writer;

pub use self::{reader::Reader, record::Record, writer::Writer};
