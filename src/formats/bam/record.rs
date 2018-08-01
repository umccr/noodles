use formats::bam::{Cigar, Data, Quality, Sequence};

#[derive(Clone, Copy, Debug)]
pub struct Flag(u16);

impl Flag {
    pub fn new(flag: u16) -> Flag {
        Flag(flag)
    }

    pub fn inner(&self) -> u16 {
        self.0
    }

    pub fn is_paired(&self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn is_proper_pair(&self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn is_unmapped(&self) -> bool {
        self.0 & 0x04 != 0
    }

    pub fn is_mate_unmapped(&self) -> bool {
        self.0 & 0x08 != 0
    }

    pub fn is_reverse(&self) -> bool {
        self.0 & 0x10 != 0
    }

    pub fn is_mate_reverse(&self) -> bool {
        self.0 & 0x20 != 0
    }

    pub fn is_read_1(&self) -> bool {
        self.0 & 0x40 != 0
    }

    pub fn is_read_2(&self) -> bool {
        self.0 & 0x80 != 0
    }

    pub fn is_secondary(&self) -> bool {
        self.0 & 0x0100 != 0
    }

    pub fn is_qc_fail(&self) -> bool {
        self.0 & 0x0200 != 0
    }

    pub fn is_dup(&self) -> bool {
        self.0 & 0x0400 != 0
    }

    pub fn is_supplementary(&self) -> bool {
        self.0 & 0x0800 != 0
    }
}

#[derive(Debug)]
pub struct Record {
    ref_id: i32,
    pos: i32,
    mapq: u8,
    bin: u16,
    flag: Flag,
    next_ref_id: i32,
    next_ref_pos: i32,
    tlen: i32,
    read_name: String,
    cigar: Cigar,
    sequence: Sequence,
    quality: Quality,
    data: Data,
}

impl Record {
    pub fn new(
        ref_id: i32,
        pos: i32,
        mapq: u8,
        bin: u16,
        flag: Flag,
        next_ref_id: i32,
        next_ref_pos: i32,
        tlen: i32,
        read_name: String,
        cigar: Cigar,
        sequence: Sequence,
        quality: Quality,
        data: Data,
    ) -> Record {
        Record {
            ref_id,
            pos,
            mapq,
            bin,
            flag,
            next_ref_id,
            next_ref_pos,
            tlen,
            read_name,
            cigar,
            sequence,
            quality,
            data,
        }
    }

    /// Returns the size of the alignment record sans block size.
    pub fn len(&self) -> usize {
        use std::mem::size_of;

        size_of::<i32>() // ref_id
            + size_of::<i32>() // pos
            + size_of::<u8>() // l_read_name
            + size_of::<u8>() // mapq
            + size_of::<u16>() // bin
            + size_of::<u16>() // n_cigar_op
            + size_of::<u16>() // flag
            + size_of::<i32>() // l_seq
            + size_of::<i32>() // next_ref_id
            + size_of::<i32>() // next_ref_pos
            + size_of::<i32>() // tlen
            + self.l_read_name() as usize // read_name
            + size_of::<u32>() * self.cigar.len() // cigar
            + self.sequence.len() // seq
            + self.quality.len() // qual
            + self.data.len() // auxiliary data
    }

    pub fn block_size(&self) -> i32 {
        self.len() as i32
    }

    pub fn ref_id(&self) -> i32 {
        self.ref_id
    }

    pub fn pos(&self) -> i32 {
        self.pos
    }

    pub fn l_read_name(&self) -> u8 {
        let len = self.read_name.len() as u8;
        len + 1
    }

    pub fn mapq(&self) -> u8 {
        self.mapq
    }

    pub fn bin(&self) -> u16 {
        self.bin
    }

    pub fn n_cigar_op(&self) -> u16 {
        self.cigar().len() as u16
    }

    pub fn flag(&self) -> Flag {
        self.flag
    }

    pub fn l_seq(&self) -> i32 {
        self.sequence.n_chars() as i32
    }

    pub fn next_ref_id(&self) -> i32 {
        self.next_ref_id
    }

    pub fn next_ref_pos(&self) -> i32 {
        self.next_ref_pos
    }

    pub fn tlen(&self) -> i32 {
        self.tlen
    }

    pub fn read_name(&self) -> &str {
        &self.read_name
    }

    pub fn cigar(&self) -> &Cigar {
        &self.cigar
    }

    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }

    pub fn quality(&self) -> &Quality{
        &self.quality
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}
