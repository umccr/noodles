use serde::{ser, Serialize};

use crate::{
    error::{Error, Result},
    record::{AuxiliarBedRecordWrapper, BedN},
    Record,
};

pub struct Record3Serializer {
    output: String,
}

// I believe this function shouldn't be user facing
fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    // TODO: How to generalize Bed<N>
    let mut serializer = Record3Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

// And that we should restrict the types that have access to the to_string() inner function
pub fn vec_record_to_string<T>(vec: Vec<T>) -> Result<String>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let input: Vec<AuxiliarBedRecordWrapper<T>> = vec
        .into_iter()
        .map(|record| AuxiliarBedRecordWrapper(record))
        .collect();

    to_string(&input)
}

pub fn record_to_string<T>(record: T) -> Result<String>
where
    T: BedN<3> + std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let abrw = AuxiliarBedRecordWrapper(record);
    to_string(&abrw)
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let string = to_string(value)?;
    Ok(string.as_bytes().to_vec())
}

impl<'a> ser::Serializer for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    // TODO: maybe use the `itoa` crate for performance.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += v;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!();
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // TODO: check all unorthodox serialization decisions
    //     (maybe unexpected data serializations should return an error)
    fn serialize_unit(self) -> Result<()> {
        self.output += "null";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
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
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    // TODO: `blocks` field uses this, make a test with it
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":[";
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self);
        self.output += "\n";
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('(') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += ")";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output += "\t";
        }
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output += "\t";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "\n";
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Record3Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.is_empty() && !self.output.ends_with('\n') {
            self.output += "\t";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "\t";
        Ok(())
    }
}
#[cfg(test)]
mod serde_tests {
    use crate::{record::Name, Record};

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

    // TODO
    // #[test]
    // fn test_to_bytes_single_record() { }

    // #[test]
    // fn test_to_bytes_vec_record() { }
}
