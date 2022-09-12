use crate::bioserde::{json_bed_to_noodles_bed, noodles_bed_to_json_bed};
use noodles_bed::Record;
use noodles_core::Position;

mod bioserde {
    use noodles_bed::Record;

    // TODO: check if this signature still makes sense
    // pub fn convert_serde_format(format_one, format_two)

    /// A function that receives a string in the noodles-bedrepresentation,
    /// deserialize to a Record<3>, and reserialize to a json-bed representation
    /// TODO: deserialize to a Record<N>
    pub fn noodles_bed_to_json_bed(input: &str) -> String {
        let record: Record<3> = noodles_bed::record_from_str(input).unwrap();

        // TODO: start treating errors
        serde_json::to_string(&record).unwrap()
    }

    /// A function that receives a string in the json-bed representation,
    /// deserialize to a Record<3>, and reserialize to a noodles-bed representation
    /// TODO: deserialize to a Record<N>
    pub fn json_bed_to_noodles_bed(input: &str) -> String {
        let record: Record<3> = serde_json::from_str(input).unwrap();

        // TODO: start treating errors
        noodles_bed::record_to_string(record).unwrap()
    }

    #[cfg(test)]
    mod serde_tests {
        use super::*;

        #[test]
        fn test_json_bed_to_noodles_bed() {
            let input = r#"{"chrom":"sq0","start":8,"end":13}"#;
            let result = json_bed_to_noodles_bed(input);

            let expected = "sq0\t7\t13";
            assert_eq!(&result, expected);
        }

        #[test]
        fn test_noodles_bed_to_json_bed() {
            let input = "sq0\t7\t13";
            let result = noodles_bed_to_json_bed(input);

            let expected = r#"{"chrom":"sq0","start":8,"end":13,"name":null,"score":null,"strand":null,"thick_start":8,"thick_end":13,"color":null,"blocks":[]}"#;
            assert_eq!(&result, expected);
        }
    }
}

/// Demonstration of deserialization.
fn main() {
    // This is a JSON representation of a bed::Record, so we can use serde_json.
    let j = r#"{"chrom":"sq0","start":8,"end":13}"#;
    let record: Record<3> = serde_json::from_str(j).unwrap();
    println!("{:#?}", record);

    // Testing Vec of Json
    let inputs = r#"[{"chrom":"sq0","start":8,"end":13},{"chrom":"sq1","start":14,"end":18}]"#;
    let records: Vec<Record<3>> = serde_json::from_str(inputs).unwrap();
    println!("\n Testing Vec of json: \n{:#?}", records);

    // For the BED file format representation of a bed::Record, we need to implement our own Deserializer.
    let record = "sq0\t7\t13";
    // let record: Record<3> = noodles_bed::record_from_str(record).unwrap();
    let record: Record<3> = noodles_bed::record_from_str(record).unwrap();
    println!("{:#?}", record);

    // TODO: check this.

    // For the BED file format representation of a bed::Record, we need to implement our own Deserializer.
    let record = "sq0\t7\t13\nsq0\t20\t34\n";
    let vec_record: Vec<Record<3>> = noodles_bed::vec_record_from_str(record).unwrap();
    println!("{:#?}", vec_record);

    // Serialization is similar, if we want the JSON representation, we can use the serde_json Serializer.
    let record = Record::<3>::builder()
        .set_reference_sequence_name("sq0")
        .set_start_position(Position::new(8).unwrap())
        .set_end_position(Position::new(13).unwrap())
        .build()
        .unwrap();
    println!("{:#?}", serde_json::to_string(&record).unwrap());

    // We need to implement our own Serialization for the BED file format representation.
    let record = Record::<3>::builder()
        .set_reference_sequence_name("sq0")
        .set_start_position(Position::try_from(8).unwrap())
        .set_end_position(Position::try_from(13).unwrap())
        .build()
        .unwrap();

    println!("{:#?}", noodles_bed::record_to_string(record).unwrap());

    // // Failing serde-transcode example:
    // // From json representation to noodles-bed representation
    // let input = r#"{"chrom":"sq0","start":8,"end":13}"#;
    // let mut deserializer = serde_json::Deserializer::from_str(input);
    // let mut serializer = noodles_bed::RecordSerializer::new();
    // serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

    // // Failing serde-transcode example:
    // // From json representation to noodles-bed representation
    // let input = r#"sq0\t7\t13\n"#;
    // let mut deserializer = noodles_bed::RecordDeserializer::from(input);
    // let mut serializer = serde_json::Serializer::new(std::io::stdout());
    // serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

    // From noodles-bed representation to json-bed representation
    let input = r#"{"chrom":"sq0","start":8,"end":13}"#;
    let result = json_bed_to_noodles_bed(input);
    println!(
        "json_bed_to_noodles_bed: input: {:?}, result: {:?}",
        input, result
    );

    // From json-bed representation to noodles-bed representation
    let input = "sq0\t7\t13";
    let result = noodles_bed_to_json_bed(input);
    println!(
        "noodles_bed_to_json_bed: input: {:?}, result: {:?}",
        input, result
    );
}
