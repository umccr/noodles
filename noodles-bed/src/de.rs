use std::io::BufRead;

use crate::{Record, reader::Reader};

pub fn from_reader<R>(reader: &mut Reader<R>) -> Vec<Record<3>>
where
    R: BufRead
{
    reader
        .records::<3>()
        // fails silently, bad design
        // TODO: define correct error handling for both of these maps
        //     should we fail the whole operation if a single record fails?
        .filter_map(|record| record.ok())
        .collect()
}


#[cfg(test)]
mod tests {
    use noodles_core::Position;

    use super::*;

    #[test]
    fn test_bed_deserialization() {
        let data = b"sq0\t8\t13\n";
        let mut reader = Reader::new(&data[..]);

        let record1 = Record::<3>::builder()
            .set_reference_sequence_name("sq0")
            .set_start_position(Position::try_from(9).expect("Failed to create position"))
            .set_end_position(Position::try_from(13).expect("Failed to create position"))
            .build()
            .expect("Failed to build bed record");

        let expected = vec![record1];

        assert_eq!(from_reader(&mut reader), expected)
    }

}
