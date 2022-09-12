use serde::{ser, Serialize};

use crate::{
    error::{Error, Result},
    record::{BedN, SerdeRecordWrapper},
};

pub struct RecordSerializer {
    output: String,
}

impl RecordSerializer {
    pub fn new() -> Self {
        RecordSerializer {
            output: String::new(),
        }
    }
}

fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = RecordSerializer::new();

    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

pub fn vec_record_to_string<T>(vec: Vec<T>) -> Result<String>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let input: Vec<SerdeRecordWrapper<T>> = vec
        .into_iter()
        .map(|record| SerdeRecordWrapper(record))
        .collect();

    to_string(&input)
}

pub fn record_to_string<T>(record: T) -> Result<String>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let srw = SerdeRecordWrapper(record);
    to_string(&srw)
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let string = to_string(value)?;
    Ok(string.as_bytes().to_vec())
}

impl<'a> ser::Serializer for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        unimplemented!()
    }

    // TODO: maybe use the `itoa` crate for performance.
    fn serialize_i64(self, _v: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut RecordSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
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
    fn test_to_string_single_auxiliar_bed_record_wrapper() {
        let record = Record::<3>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_multiple_auxiliar_bed_record_wrapper() {
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

        let input = vec![record1, record2];

        let result = vec_record_to_string(input).unwrap();
        let expected = "sq0\t7\t13\nsq1\t13\t18\n";
        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_4_wrapper() {
        let record = Record::<4>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_name("ndls1".parse::<Name>().unwrap())
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\tndls1";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_5_wrapper() {
        let record = Record::<5>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_score(Score::try_from(21).unwrap())
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\t.\t21";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_6_wrapper() {
        let record = Record::<6>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(noodles_core::Position::try_from(8).unwrap())
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_strand(Strand::Forward)
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\t.\t0\t+";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_7_wrapper() {
        let start = noodles_core::Position::try_from(8).unwrap();

        let record = Record::<7>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(noodles_core::Position::try_from(13).unwrap())
            .set_thick_start(start)
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\t.\t0\t.\t7";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_8_wrapper() {
        let start = noodles_core::Position::try_from(8).unwrap();
        let end = noodles_core::Position::try_from(13).unwrap();

        let record = Record::<8>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(end)
            .set_thick_start(start)
            .set_thick_end(end)
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\t.\t0\t.\t7\t13";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_9_wrapper() {
        let start = noodles_core::Position::try_from(8).unwrap();
        let end = noodles_core::Position::try_from(13).unwrap();

        let record = Record::<9>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(end)
            .set_thick_start(start)
            .set_thick_end(end)
            .set_color(Color::RED)
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\t.\t0\t.\t7\t13\t255,0,0";

        assert_eq!(&result, expected);
    }

    #[test]
    fn test_to_string_single_auxiliar_bed_record_12_wrapper() {
        let start = noodles_core::Position::try_from(8).unwrap();
        let end = noodles_core::Position::try_from(13).unwrap();

        let record = Record::<12>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(start)
            .set_end_position(end)
            .set_thick_start(start)
            .set_thick_end(end)
            .set_blocks(vec![(0, 2)])
            .build()
            .unwrap();

        let result = record_to_string(record).unwrap();
        let expected = "sq0\t7\t13\t.\t0\t.\t7\t13\t0\t1\t2\t0";

        assert_eq!(&result, expected);
    }
}
