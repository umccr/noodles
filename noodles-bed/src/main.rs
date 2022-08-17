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

    // For the BED file format representation of a bed::Record, we need to implement our own Deserializer.
    // let record = b"sq0\t7\t13\nsq0\t20\t34\n";
    // let record: Record<3> = from_bytes(record).unwrap();
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

    // // maybe it should be possible to NOT borrow
    // println!("{:#?}", to_bytes(record).unwrap());

    // println!("{:#?}", noodles_bed::record_to_string(record).unwrap());
    // println!("{:#?}", to_bytes(&record).unwrap());
}
