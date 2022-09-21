use noodles_bed::Record;
use noodles_core::Position;

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

    // // For the BED file format representation of a bed::Record, we need to implement our own Deserializer.
    // let record = "sq0\t7\t13\nsq0\t20\t34\n";
    // let record: Record<3> = noodles_bed::record_from_str(record).unwrap();
    // println!("{:#?}", record);

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

    // TODO: remove and/or fix
    // println!("{:#?}", noodles_bed::record_to_string(record).unwrap());

    // Failing serde-transcode example:
    // From json representation to noodles-bed representation
    let input = r#"{"chrom":"sq0","start":8,"end":13}"#;
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let mut serializer = noodles_bed::RecordSerializer::new();
    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

    // // Failing serde-transcode example:
    // // From json representation to noodles-bed representation
    // let input = r#"sq0\t7\t13\n"#;
    // let mut deserializer = noodles_bed::RecordDeserializer::from(input);
    // let mut serializer = serde_json::Serializer::new(std::io::stdout());
    // serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

    // From noodles-bed representation to json representation
}

// #[cfg(test)]
// mod bioserde_tests {
//     use super::*;

//     #[test]
//     fn test_json_bed3_to_noodles_bed3() {
//         let input = r#"{"chrom":"sq0","start":8,"end":13}"#;
//         let result = json_bed3_to_noodles_bed3(input);

//         let expected = "sq0\t7\t13";
//         assert_eq!(&result, expected);
//     }

//     #[test]
//     fn test_noodles_bed3_to_json_bed3() {
//         let input = "sq0\t7\t13";
//         let result = noodles_bed3_to_json_bed3(input);

//         let expected = r#"{"chrom":"sq0","start":8,"end":13,"name":null,"score":null,"strand":null,"thick_start":8,"thick_end":13,"color":null,"blocks":[]}"#;
//         assert_eq!(&result, expected);
//     }

//     #[test]
//     fn test_json_bed3_to_noodles_bed3_with_enum_usage() {
//         let input = r#"{"chrom":"sq0","start":8,"end":13}"#;
//         let result = convert_to_format(input, SupportedFormat::JsonBed3, SupportedFormat::Record3);

//         let expected = "sq0\t7\t13";
//         assert_eq!(&result, expected);
//     }

//     #[test]
//     fn test_noodles_bed3_to_json_bed3_with_enum_usage() {
//         let input = "sq0\t7\t13";
//         let result = convert_to_format(input, SupportedFormat::Record3, SupportedFormat::JsonBed3);

//         let expected = r#"{"chrom":"sq0","start":8,"end":13,"name":null,"score":null,"strand":null,"thick_start":8,"thick_end":13,"color":null,"blocks":[]}"#;
//         assert_eq!(&result, expected);
//     }

//     #[test]
//     fn test_json_bed4_to_noodles_bed4_with_enum_usage() {
//         let input = r#"{"chrom":"sq0","start":8,"end":13,"name":"ndls1"}"#;
//         let result = convert_to_format(input, SupportedFormat::JsonBed4, SupportedFormat::Record4);

//         let expected = "sq0\t7\t13\tndls1";
//         assert_eq!(&result, expected);
//     }

//     #[test]
//     fn test_noodles_bed4_to_json_bed4_with_enum_usage() {
//         let input = "sq0\t7\t13\tndls1";
//         let result = convert_to_format(input, SupportedFormat::Record4, SupportedFormat::JsonBed4);

//         let expected = r#"{"chrom":"sq0","start":8,"end":13,"name":"ndls1","score":null,"strand":null,"thick_start":8,"thick_end":13,"color":null,"blocks":[]}"#;
//         assert_eq!(&result, expected);
//     }
// }
