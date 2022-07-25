use noodles_bed::{from_bytes, Record};
use noodles_bed::record::Builder;
use noodles_core::Position;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Record3 {
    chrom: String,
    start: Position,
    end: Position
}

/// Demonstration of deserialization.
fn main() {
    // This is a JSON representation of a bed::Record, so we can use serde_json.
    let j = r#"{"chrom":"sq0","start":8,"end":13}"#;
    let record: Record3 = serde_json::from_str(j).unwrap();
    println!("{:#?}", record);

    // For the BED file format representation of a bed::Record, we need to implement our own Deserializer.
    // let record = b"sq0\t7\t13\nsq0\t20\t34\n";
    // let record: Record<3> = from_bytes(record).unwrap();
    // println!("{:#?}", record);

    // Serialization is similar, if we want the JSON representation, that's already implemented.
    let record = Record3 {
        chrom: "sq0".to_string(),
        start: Position::new(8).unwrap(),
        end: Position::new(13).unwrap()
    };
    println!("{:#?}", serde_json::to_string(&record).unwrap());

    // // We need to implement our own Serialization for the BED file format representation.
    // let record = Record::builder()
    //   .set_reference_sequence_name("sq0")
    //   .set_start_position(Position::try_from(7).unwrap())
    //   .set_end_position(Position::try_from(13).unwrap()).build().unwrap();
    // let record: Record<3> = from_bytes(record).unwrap();
    // println!("{:#?}", record);
}
