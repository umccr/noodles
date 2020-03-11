pub mod resolve;

use noodles_sam as sam;

use crate::{Feature, Flags, Tag};

#[derive(Clone, Debug, Default)]
pub struct Record {
    pub bam_bit_flags: i32,
    pub cram_bit_flags: i32,
    pub reference_id: i32,
    pub read_length: i32,
    pub alignment_start: i32,
    pub read_group: i32,
    pub read_name: Vec<u8>,
    pub next_mate_bit_flags: i32,
    pub next_fragment_reference_sequence_id: i32,
    pub next_mate_alignment_start: i32,
    pub template_size: i32,
    pub distance_to_next_fragment: i32,
    pub tags: Vec<Tag>,
    pub bases: Vec<u8>,
    pub features: Vec<Feature>,
    pub mapping_quality: i32,
    pub quality_scores: Vec<u8>,
}

impl Record {
    pub fn bam_bit_flags(&self) -> sam::Flags {
        // `bam_bit_flags` can safely be casted to a u16 because it is the same range as specified
        // in the SAM specification, i.e., [0, 2^16 - 1].
        sam::Flags::from(self.bam_bit_flags as u16)
    }

    pub fn cram_bit_flags(&self) -> Flags {
        // `cram_bit_flags` can safely be casted to to a u8 because CRAM currently only has 4 bit
        // flags, i.e., the largest value is 2^4 - 1.
        Flags::from(self.cram_bit_flags as u8)
    }

    pub fn read_length(&self) -> i32 {
        self.read_length
    }

    pub fn alignment_start(&self) -> i32 {
        self.alignment_start
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn features(&self) -> &[Feature] {
        &self.features
    }

    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }
}
