use crate::de::RecordState::ExpectingChrom;
use crate::{error, Reader, Record};
use error::{Error, Result};
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any, Deserialize, Deserializer};
use std::io;
use std::iter::Peekable;

pub fn from_bytes<'a, T>(records: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut reader = Reader::new(records);
    let records = reader.records();
    let mut deserializer = Record3Deserializer::new(records);
    T::deserialize(&mut deserializer)
}

/// Keeps track of the state associated with the next record that is parsed.
/// We need to keep track of the state, as records are returned whole from the iterator.
enum RecordState {
    ExpectingChrom,
    ExpectingChromStart(Record<3>),
    ExpectingChromEnd(Record<3>),
}

/// The Record Deserializer. This struct implements Deserializer and associated Traits.
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

    /// We can use forward_to_deserialize_any for many types, as bed Records are self-describing.
    forward_to_deserialize_any! {
          bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
          bytes byte_buf option unit unit_struct newtype_struct seq tuple
          tuple_struct map enum identifier ignored_any
    }
}

/// SeqAccess will be needed to sequences, and to deserialize a struct.
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
