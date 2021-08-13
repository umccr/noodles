use std::convert::TryFrom;

use noodles_bgzf as bgzf;
use noodles_csi::{
    index::reference_sequence::{bin::Chunk, Metadata},
    BinningIndex, BinningIndexReferenceSequence,
};
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

use crate::{
    index::{reference_sequence::Bin, Header, ReferenceSequence, ReferenceSequenceNames},
    Index,
};

const NUL: u8 = b'\x00';

/// An async tabix writer.
pub struct Writer<W> {
    inner: bgzf::AsyncWriter<W>,
}

impl<W> Writer<W>
where
    W: AsyncWrite + Unpin,
{
    /// Creates an async tabix writer.
    pub fn new(inner: W) -> Self {
        Self {
            inner: bgzf::AsyncWriter::new(inner),
        }
    }

    /// Returns the underlying writer.
    pub fn into_inner(self) -> W {
        self.inner.into_inner()
    }

    /// Shuts down the output stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> io::Result<()> {
    /// use noodles_tabix as tabix;
    /// let mut writer = tabix::AsyncWriter::new(Vec::new());
    /// writer.shutdown().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn shutdown(&mut self) -> io::Result<()> {
        self.inner.shutdown().await
    }

    /// Writes a tabix index.
    pub async fn write_index(&mut self, index: &Index) -> io::Result<()> {
        write_magic(&mut self.inner).await?;

        let n_ref = i32::try_from(index.reference_sequences().len())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        self.inner.write_i32_le(n_ref).await?;

        write_header(&mut self.inner, index.header()).await?;

        write_reference_sequence_names(&mut self.inner, index.reference_sequence_names()).await?;
        write_reference_sequences(&mut self.inner, index.reference_sequences()).await?;

        if let Some(n_no_coor) = index.unplaced_unmapped_record_count() {
            self.inner.write_u64_le(n_no_coor).await?;
        }

        Ok(())
    }
}

async fn write_magic<W>(writer: &mut W) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    writer.write_all(crate::MAGIC_NUMBER).await
}

async fn write_header<W>(writer: &mut W, header: &Header) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let format = i32::from(header.format());
    writer.write_i32_le(format).await?;

    let col_seq = i32::try_from(header.reference_sequence_name_index())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    writer.write_i32_le(col_seq).await?;

    let col_beg = i32::try_from(header.start_position_index())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    writer.write_i32_le(col_beg).await?;

    let col_end = header.end_position_index().map_or(Ok(0), |i| {
        i32::try_from(i).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    })?;
    writer.write_i32_le(col_end).await?;

    let meta = i32::from(header.line_comment_prefix());
    writer.write_i32_le(meta).await?;

    let skip = i32::try_from(header.line_skip_count())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    writer.write_i32_le(skip).await?;

    Ok(())
}

async fn write_reference_sequence_names<W>(
    writer: &mut W,
    reference_sequence_names: &ReferenceSequenceNames,
) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    // Add 1 for each trailing NUL.
    let len: usize = reference_sequence_names.iter().map(|n| n.len() + 1).sum();
    let l_nm = i32::try_from(len).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    writer.write_i32_le(l_nm).await?;

    for reference_sequence_name in reference_sequence_names {
        writer.write_all(reference_sequence_name.as_bytes()).await?;
        writer.write_u8(NUL).await?;
    }

    Ok(())
}

async fn write_reference_sequences<W>(
    writer: &mut W,
    reference_sequences: &[ReferenceSequence],
) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    for reference_sequence in reference_sequences {
        write_reference_sequence(writer, reference_sequence).await?;
    }

    Ok(())
}

async fn write_reference_sequence<W>(
    writer: &mut W,
    reference_sequence: &ReferenceSequence,
) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    write_bins(
        writer,
        reference_sequence.bins(),
        reference_sequence.metadata(),
    )
    .await?;

    write_intervals(writer, reference_sequence.intervals()).await?;

    Ok(())
}

async fn write_bins<W>(writer: &mut W, bins: &[Bin], metadata: Option<&Metadata>) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let n_bin = i32::try_from(bins.len())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
        .and_then(|n| {
            if metadata.is_some() {
                n.checked_add(1)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "n_bin overflow"))
            } else {
                Ok(n)
            }
        })?;

    writer.write_i32_le(n_bin).await?;

    for bin in bins {
        write_bin(writer, bin).await?;
    }

    if let Some(m) = metadata {
        write_metadata(writer, m).await?;
    }

    Ok(())
}

async fn write_bin<W>(writer: &mut W, bin: &Bin) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    writer.write_u32_le(bin.id()).await?;
    write_chunks(writer, bin.chunks()).await?;
    Ok(())
}

async fn write_chunks<W>(writer: &mut W, chunks: &[Chunk]) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let n_chunk =
        i32::try_from(chunks.len()).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    writer.write_i32_le(n_chunk).await?;

    for chunk in chunks {
        write_chunk(writer, chunk).await?;
    }

    Ok(())
}

async fn write_chunk<W>(writer: &mut W, chunk: &Chunk) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let cnk_beg = u64::from(chunk.start());
    writer.write_u64_le(cnk_beg).await?;

    let cnk_end = u64::from(chunk.end());
    writer.write_u64_le(cnk_end).await?;

    Ok(())
}

async fn write_intervals<W>(writer: &mut W, intervals: &[bgzf::VirtualPosition]) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    let n_intv = i32::try_from(intervals.len())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    writer.write_i32_le(n_intv).await?;

    for &interval in intervals {
        let ioff = u64::from(interval);
        writer.write_u64_le(ioff).await?;
    }

    Ok(())
}

async fn write_metadata<W>(writer: &mut W, metadata: &Metadata) -> io::Result<()>
where
    W: AsyncWrite + Unpin,
{
    use crate::index::reference_sequence::bin::{METADATA_CHUNK_COUNT, METADATA_ID};

    let bin_id = METADATA_ID;
    writer.write_u32_le(bin_id).await?;

    let n_chunk = METADATA_CHUNK_COUNT;
    writer.write_u32_le(n_chunk).await?;

    let ref_beg = u64::from(metadata.start_position());
    writer.write_u64_le(ref_beg).await?;

    let ref_end = u64::from(metadata.end_position());
    writer.write_u64_le(ref_end).await?;

    let n_mapped = metadata.mapped_record_count();
    writer.write_u64_le(n_mapped).await?;

    let n_unmapped = metadata.unmapped_record_count();
    writer.write_u64_le(n_unmapped).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_magic() -> io::Result<()> {
        let mut buf = Vec::new();
        write_magic(&mut buf).await?;
        assert_eq!(buf, b"TBI\x01");
        Ok(())
    }

    #[tokio::test]
    async fn test_write_header() -> io::Result<()> {
        let header = crate::index::header::Builder::gff().build();

        let mut buf = Vec::new();
        write_header(&mut buf, &header).await?;

        let expected = [
            0x00, 0x00, 0x00, 0x00, // format = Generic(GFF)
            0x01, 0x00, 0x00, 0x00, // col_seq = 1
            0x04, 0x00, 0x00, 0x00, // col_beg = 4
            0x05, 0x00, 0x00, 0x00, // col_end = 5
            0x23, 0x00, 0x00, 0x00, // meta = '#'
            0x00, 0x00, 0x00, 0x00, // skip = 0
        ];

        assert_eq!(buf, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_reference_sequence_names() -> io::Result<()> {
        let reference_sequence_names = vec![String::from("sq0"), String::from("sq1")]
            .into_iter()
            .collect();

        let mut buf = Vec::new();
        write_reference_sequence_names(&mut buf, &reference_sequence_names).await?;

        let expected = [
            0x08, 0x00, 0x00, 0x00, // l_nm = 8
            0x73, 0x71, 0x30, 0x00, // names[0] = b"sq0\x00"
            0x73, 0x71, 0x31, 0x00, // names[1] = b"sq1\x00"
        ];

        assert_eq!(buf, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_metadata() -> io::Result<()> {
        let metadata = Metadata::new(
            bgzf::VirtualPosition::from(610),
            bgzf::VirtualPosition::from(1597),
            55,
            0,
        );

        let mut buf = Vec::new();
        write_metadata(&mut buf, &metadata).await?;

        let expected = [
            0x4a, 0x92, 0x00, 0x00, // bin = 37450
            0x02, 0x00, 0x00, 0x00, // n_chunks = 2
            0x62, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ref_beg = 610
            0x3d, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ref_end = 1597
            0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // n_mapped = 55
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // n_unmapped = 0
        ];

        assert_eq!(buf, expected);

        Ok(())
    }
}