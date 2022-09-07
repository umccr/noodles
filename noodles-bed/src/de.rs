use crate::error;
use crate::record::{BedN, SerdeRecordWrapper};
use error::{Error, Result};
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::{de, forward_to_deserialize_any, ser, Deserialize};

pub struct RecordDeserializer<'de> {
    input: &'de str,
}

fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = RecordDeserializer::from(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(ser::Error::custom("trailing characters"))
    }
}

pub fn record_from_str<T>(s: &str) -> Result<T>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let srw: SerdeRecordWrapper<T> = from_str(s)?;
    Ok(srw.0)
}

pub fn vec_record_from_str<T>(s: &str) -> Result<Vec<T>>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let srw_vec: Vec<SerdeRecordWrapper<T>> = from_str(s)?;
    Ok(srw_vec.into_iter().map(|wrap| wrap.0).collect())
}

pub fn from_bytes<'a, T>(_records: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    todo!()
}

impl<'de> From<&'de str> for RecordDeserializer<'de> {
    fn from(input: &'de str) -> Self {
        RecordDeserializer { input }
    }
}

impl<'de> RecordDeserializer<'de> {
    fn parse_string(&mut self) -> &'de str {
        match self.input.find('\n') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + 1..];
                s
            }
            None => {
                let s = self.input;
                self.input = "";
                s
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut RecordDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_string())
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
          bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
          bytes byte_buf option unit unit_struct tuple struct
          tuple_struct map enum identifier ignored_any
    }
}

impl<'de> SeqAccess<'de> for RecordDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.input.is_empty() {
            Ok(None)
        } else {
            seed.deserialize(&mut *self).map(Some)
        }
    }
}

#[cfg(test)]
mod serde_tests {
    use crate::{
        record::{Color, Name, Score, Strand},
        Record,
    };

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

    #[test]
    fn test_from_string_single_auxiliar_bed_record_4_wrapper() {
        let input = "sq0\t7\t13\tndls1";
        let result: Record<4> = record_from_str(input).unwrap();

        let expected = Record::<4>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_name("ndls1".parse::<Name>().unwrap())
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_5_wrapper() {
        let input = "sq0\t7\t13\t.\t21";
        let result: Record<5> = record_from_str(input).unwrap();

        let expected = Record::<5>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_score(Score::try_from(21).unwrap())
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_6_wrapper() {
        let input = "sq0\t7\t13\t.\t0\t+";
        let result: Record<6> = record_from_str(input).unwrap();

        let expected = Record::<6>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_strand(Strand::Forward)
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_7_wrapper() {
        let input = "sq0\t7\t13\t.\t0\t.\t7";
        let result: Record<7> = record_from_str(input).unwrap();

        let start = noodles_core::Position::try_from(8).unwrap();

        let expected = Record::<7>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_thick_start(start)
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_8_wrapper() {
        let input = "sq0\t7\t13\t.\t0\t.\t7\t13";
        let result: Record<8> = record_from_str(input).unwrap();

        let start = noodles_core::Position::try_from(8).unwrap();
        let end = noodles_core::Position::try_from(13).unwrap();

        let expected = Record::<8>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(end)
            .set_thick_start(start)
            .set_thick_end(end)
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_9_wrapper() {
        let input = "sq0\t7\t13\t.\t0\t.\t7\t13\t255,0,0";
        let result: Record<9> = record_from_str(input).unwrap();

        let start = noodles_core::Position::try_from(8).unwrap();
        let end = noodles_core::Position::try_from(13).unwrap();

        let expected = Record::<9>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(end)
            .set_thick_start(start)
            .set_thick_end(end)
            .set_color(Color::RED)
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_12_wrapper() {
        let input = "sq0\t7\t13\t.\t0\t.\t7\t13\t0\t1\t2\t0";
        let result: Record<12> = record_from_str(input).unwrap();

        let start = noodles_core::Position::try_from(8).unwrap();
        let end = noodles_core::Position::try_from(13).unwrap();

        let expected = Record::<12>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(end)
            .set_thick_start(start)
            .set_thick_end(end)
            .set_blocks(vec![(0, 2)])
            .build()
            .unwrap();

        assert_eq!(result, expected);
    }
}
