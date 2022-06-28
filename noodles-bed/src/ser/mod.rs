//! BED data structure serialization.

use crate::serialize::BedSerialize;
use std::io::{Cursor, Write};
use std::str;

use serde::Serializer;
use crate::Writer;

/// Serialize BED into a plain String
pub fn to_string<T: BedSerialize>(model: &T) -> Result<String, String> {
  let buf = Cursor::new(Vec::new());
  let cursor = serialize_with_writer(model, buf)?;
  let data = str::from_utf8(cursor.get_ref()).expect("Found invalid UTF-8");
  Ok(data.into())
}

pub fn serialize_with_writer<W: Write, T: BedSerialize>(
  model: &T,
  writer: W,
) -> Result<W, String> {
  let mut serializer = Serializer::new_from_writer(writer);
  match BedSerialize::serialize(model, &mut serializer) {
    Ok(()) => Ok(serializer.into_inner()),
    Err(msg) => Err(msg),
  }
}

impl<'de, W: Write> dyn Serializer {
  pub fn new(writer: Writer<W>) -> Self {
    Serializer {
      writer,
    }
  }

  pub fn new_from_writer(writer: W) -> Self {
    todo!()
  }

  pub fn write(&mut self) -> Writer<W>
  {
    todo!()
  }
}