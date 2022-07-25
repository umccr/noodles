use crate::de::RecordState::ExpectingChrom;
use crate::{error, Reader, Record};
use error::{Error, Result};
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any, Deserialize, Deserializer};
use std::io;
use std::iter::Peekable;

/// A user would likely use a function such as the following to deserialize a set of records.
pub fn from_bytes<'a, T>(records: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut reader = Reader::new(records);
    let records = reader.records();
    let mut deserializer = Record3Deserializer::new(records);
    T::deserialize(&mut deserializer)
}

/// This leeps track of the state associated with the next record that is parsed.
/// We need to keep track of the state as records are returned whole from the iterator, whereas
/// the deserializer progresses one field at a time.
enum RecordState {
    ExpectingChrom,
    ExpectingChromStart(Record<3>),
    ExpectingChromEnd(Record<3>),
}

/// The Record Deserializer. This struct implements Deserializer and associated Traits. It stores
/// an iterator over the records that are given to it.
pub struct Record3Deserializer<I>
where
    I: Iterator<Item = io::Result<Record<3>>>,
{
    records: Peekable<I>,
    state: RecordState,
}

impl<I> Record3Deserializer<I>
where
    I: Iterator<Item = io::Result<Record<3>>>,
{
    fn new(records: I) -> Self {
        Self {
            records: records.peekable(),
            state: ExpectingChrom,
        }
    }
}

impl<'de, 'a, I> Deserializer<'de> for &'a mut Record3Deserializer<I>
where
    I: Iterator<Item = io::Result<Record<3>>>,
{
    type Error = Error;

    /// This function could handle deserializing an individual record, matching the deserializer's
    /// RecordState, and progressing it to the next state.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    /// We will likely need to handle deserializing a struct.
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    /// And a sequence, in a similar way.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    /// We can probably use forward_to_deserialize_any for many types, as bed Records are self-describing.
    forward_to_deserialize_any! {
          bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
          bytes byte_buf option unit unit_struct newtype_struct tuple
          tuple_struct map enum identifier ignored_any
    }
}

/// SeqAccess will be needed to deserialize sequences and structs.
impl<'de, I> SeqAccess<'de> for Record3Deserializer<I>
where
    I: Iterator<Item = io::Result<Record<3>>>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        todo!()
    }
}
