use std::io::{self, Write};

use super::{Header, Record};

#[derive(Debug)]
pub struct Writer<W> {
    inner: W,
}

impl<W> Writer<W>
where
    W: Write,
{
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn write_header(&mut self, header: &Header) -> io::Result<()> {
        write!(self.inner, "{}", header)
    }

    pub fn write_record(&mut self, record: &Record) -> io::Result<()> {
        write!(
            self.inner,
            "{chrom}\t{pos}\t{id}\t{ref}\t{alt}\t{qual}\t{filter}\t{info}",
            chrom = record.chromosome(),
            pos = record.position(),
            id = record.id(),
            r#ref = record.reference_bases(),
            alt = record.alternate_bases(),
            qual = record.quality_score(),
            filter = record.filter_status(),
            info = record.info(),
        )?;

        if let Some(format) = record.format() {
            write!(self.inner, "\t{}", format)?;

            for field in record.genotypes() {
                write!(self.inner, "\t{}", field)?;
            }
        }

        writeln!(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::record::{Format, Genotype};

    use super::*;

    #[test]
    fn test_write_header() -> io::Result<()> {
        let mut writer = Writer::new(Vec::new());

        let header = Header::default();
        writer.write_header(&header)?;

        let expected = b"##fileformat=VCFv4.3
#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO
";

        assert_eq!(writer.get_ref().as_slice(), &expected[..]);

        Ok(())
    }

    #[test]
    fn test_write_record() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::new(Vec::new());

        let record = Record::builder()
            .set_chromosome("sq0".parse()?)
            .set_position(1)
            .set_reference_bases("A".parse()?)
            .build()?;

        writer.write_record(&record)?;

        let expected = b"sq0\t1\t.\tA\t.\t.\t.\t.\n";

        assert_eq!(writer.get_ref(), expected);

        Ok(())
    }

    #[test]
    fn test_write_record_with_format() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::new(Vec::new());

        let format: Format = "GT:GQ".parse()?;

        let record = Record::builder()
            .set_chromosome("sq0".parse()?)
            .set_position(1)
            .set_reference_bases("A".parse()?)
            .set_format(format.clone())
            .add_genotype(Genotype::from_str_format("0|0:13", &format)?)
            .build()?;

        writer.write_record(&record)?;

        let expected = b"sq0\t1\t.\tA\t.\t.\t.\t.\tGT:GQ\t0|0:13\n";

        assert_eq!(writer.get_ref(), expected);

        Ok(())
    }
}
