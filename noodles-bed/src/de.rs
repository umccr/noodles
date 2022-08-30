// use crate::de::RecordState::ExpectingChrom;
use crate::record::{AuxiliarBedRecordWrapper, BedN};
use crate::{error, Reader, Record};
use error::{Error, Result};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any, Deserialize, Deserializer};
use std::io;
use std::iter::Peekable;

pub fn from_bytes<'a, T>(records: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    todo!()
}

// // I will use this auxiliar later to emulate whats going on in the boilerplate on test
// pub fn record_from_string<T>(record: T) -> Result<String>
// where
//     T: BedN<3> + std::str::FromStr + std::fmt::Display,
//     <T as std::str::FromStr>::Err: std::fmt::Display,
// {
//     let abrw = AuxiliarBedRecordWrapper { record };
//     from_str(&abrw)
// }

pub fn record_from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let abrw: AuxiliarBedRecordWrapper<T> = from_str(s)?;
    Ok(abrw.0)
}

pub fn vec_record_from_str<'a, T>(s: &'a str) -> Result<Vec<T>>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let abrw_vec: Vec<AuxiliarBedRecordWrapper<T>> = from_str(s)?;
    Ok(abrw_vec.into_iter().map(|wrap| wrap.0).collect())
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = RecordDeserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        panic!()
    }
}

pub struct RecordDeserializer<'de> {
    input: &'de str,
}

impl<'de> RecordDeserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        RecordDeserializer { input }
    }

    // It seems I am implementing things i wasn't expecting to implement.
    fn next_field(&mut self) -> Result<&'de str> {
        // todo!()
        Ok(self.input)
        // match self.it.next() {
        //     Some(field) => {
        //         self.field += 1;
        //         Ok(field)
        //     }
        //     None => Err(DeserializeError {
        //         field: None,
        //         kind: DEK::UnexpectedEndOfRow,
        //     }),
        // }
    }

    fn parse_string(&mut self) -> &'de str {
        match self.input.find('\n') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                s
            }
            None => {
                let s = &self.input[..];
                self.input = "";
                s
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut RecordDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    /// We will likely need to handle deserializing a sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    /// We also need to deserialize a struct in a similar way, as the BED
    /// format doesn't have a concept of a struct. It just processes records
    /// line by line similar to CSV, where fields are separated by some separator.
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("map");
        // visitor.visit_map(self)
        visitor.visit_seq(self)
    }

    // fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    // where
    //     V: Visitor<'de>,
    // {
    //     dbg!("string");
    //     visitor.visit_str(self.input)
    // }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        dbg!("string");
        self.next_field().and_then(|f| visitor.visit_str(f.into()))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        dbg!("identifier");
        // self.deserialize_string(visitor)
        // self.deserialize_str(visitor)
        visitor.visit_str("record")
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // dbg!(self.input);
        // dbg!("deserialize_ignored_any");
        // drop(self.input);
        self.parse_string();
        // panic!(); // uncomment to stop loop
        visitor.visit_unit()
    }

    // /// We can probably use forward_to_deserialize_any for many types, as bed Records are self-describing.
    // forward_to_deserialize_any! {
    //       bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    //       bytes byte_buf option unit unit_struct newtype_struct tuple
    //       tuple_struct map enum identifier ignored_any
    // }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
          bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
          bytes byte_buf option unit unit_struct tuple
          tuple_struct enum
    }
}

/// SeqAccess will be needed to deserialize sequences and structs.
impl<'de> SeqAccess<'de> for RecordDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.input == "" {
            Ok(None)
        } else {
            seed.deserialize(&mut *self).map(Some)
        }
    }

    // fn next_element<T>(&mut self) -> Result<Option<T>>
    // where
    //     T: Deserialize<'de>,
    // {
    //     self.next_element_seed(std::marker::PhantomData)
    // }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<'de, 'a> MapAccess<'de> for RecordDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // // Check if there are no more entries.
        // if self.de.peek_char()? == '}' {
        //     return Ok(None);
        // }
        // // Comma is required before every entry except the first.
        // if !self.first && self.de.next_char()? != ',' {
        //     return Err(Error::ExpectedMapComma);
        // }
        // self.first = false;
        // // Deserialize a map key.
        // seed.deserialize(&mut *self.de).map(Some)
        seed.deserialize(&mut *self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // It doesn't make a difference whether the colon is parsed at the end
        // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // case the code is a bit simpler having it here.
        // if self.de.next_char()? != ':' {
        //     return Err(Error::ExpectedMapColon);
        // }
        // Deserialize a map value.
        seed.deserialize(&mut *self)
    }
}

// /// A user would likely use a function such as the following to deserialize a set of records.
// pub fn from_bytes<'a, T>(records: &'a [u8]) -> Result<T>
// where
//     T: Deserialize<'a>,
// {
//     let mut reader = Reader::new(records);
//     let records = reader.records();
//     let mut deserializer = Record3Deserializer::new(records);
//     T::deserialize(&mut deserializer)
// }

// // I believe this function shouldn't be user facing
// fn from_string<'a, T>(value: String) -> Result<T>
// where
//     T: Deserialize<'a>,
// {
//     // TODO: How to generalize Bed<N>
//     let mut serializer = Record3Deserializer {
//         input: String::new(),
//     };
//     value.deserialize(&mut serializer)?;
//     Ok(serializer.output)
// }

// pub fn record_from_string<T>(record: String) -> Result<T>
// where
//     T: BedN<3> + std::str::FromStr + std::fmt::Display,
//     <T as std::str::FromStr>::Err: std::fmt::Display,
// {
//     let abrw = AuxiliarBedRecordWrapper { record };
//     from_string(&abrw)
// }

// /// This leeps track of the state associated with the next record that is parsed.
// /// We need to keep track of the state as records are returned whole from the iterator, whereas
// /// the deserializer progresses one field at a time.
// enum RecordState {
//     ExpectingChrom,
//     ExpectingChromStart(Record<3>),
//     ExpectingChromEnd(Record<3>),
// }

// /// The Record Deserializer. This struct implements Deserializer and associated Traits. It stores
// /// an iterator over the records that are given to it.
// pub struct Record3Deserializer<I>
// where
//     I: Iterator<Item = io::Result<Record<3>>>,
// {
//     records: Peekable<I>,
//     state: RecordState,
// }

// impl<I> Record3Deserializer<I>
// where
//     I: Iterator<Item = io::Result<Record<3>>>,
// {
//     fn new(records: I) -> Self {
//         Self {
//             records: records.peekable(),
//             state: ExpectingChrom,
//         }
//     }
// }

// impl<'de, 'a, I> Deserializer<'de> for &'a mut Record3Deserializer<I>
// where
//     I: Iterator<Item = io::Result<Record<3>>>,
// {
//     type Error = Error;

//     /// This function could handle deserializing an individual record, matching the deserializer's
//     /// RecordState, and progressing it to the next state.
//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//         // match self.state {
//         //     ExpectingChrom => deserialize_string,
//         //     ExpectingChromStart(Record<3>) => ,
//         //     ExpectingChromEnd(Record<3>) => ,
//         // }
//     }

//     // fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
//     //     self.next_field().and_then(|f| visitor.visit_str(f.into()))
//     // }

//     /// We will likely need to handle deserializing a sequence.
//     fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_seq(self)
//     }

//     /// We also need to deserialize a struct in a similar way, as the BED
//     /// format doesn't have a concept of a struct. It just processes records
//     /// line by line similar to CSV, where fields are separated by some separator.
//     fn deserialize_struct<V>(
//         self,
//         name: &'static str,
//         fields: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_seq(self)
//     }

//     /// We can probably use forward_to_deserialize_any for many types, as bed Records are self-describing.
//     // forward_to_deserialize_any! {
//     //       bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str
//     //       bytes byte_buf option unit unit_struct newtype_struct tuple
//     //       tuple_struct map enum identifier ignored_any
//     // }

//     forward_to_deserialize_any! {
//           bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
//           bytes byte_buf option unit unit_struct newtype_struct tuple
//           tuple_struct map enum identifier ignored_any
//     }
// }

// /// SeqAccess will be needed to deserialize sequences and structs.
// impl<'de, I> SeqAccess<'de> for Record3Deserializer<I>
// where
//     I: Iterator<Item = io::Result<Record<3>>>,
// {
//     type Error = Error;

//     fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
//     where
//         T: DeserializeSeed<'de>,
//     {
//         todo!()
//     }
// }

#[cfg(test)]
mod serde_tests {
    use crate::{record::Name, Record};

    use super::*;

    #[test]
    fn test_from_string_single_auxiliar_bed_record_wrapper() {
        let input = "sq0\t7\t13\n";
        let result: Record<3> = record_from_str(input).unwrap();

        let expected = Record::<3>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_string_multiple_auxiliar_bed_record_wrapper() {
        let input = "sq0\t7\t13\nsq1\t13\t18\n";
        let result: Vec<Record<3>> = vec_record_from_str(input).unwrap();

        let record1 = Record::<3>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .build()
            .unwrap();
        let record2 = Record::<3>::builder()
            .set_reference_sequence_name("sq1")
            .set_start_position(noodles_core::Position::try_from(14).unwrap())
            .set_end_position(noodles_core::Position::try_from(18).unwrap())
            .build()
            .unwrap();
        let expected = vec![record1, record2];

        assert_eq!(result, expected);
    }
}
