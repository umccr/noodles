//! GFF directives.

pub mod sequence_region;

pub use self::sequence_region::SequenceRegion;

use std::{error, fmt, str::FromStr};

pub(crate) const PREFIX: &str = "##";

/// A GFF directive.
///
/// This is also called a pragma or metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Directive {
    /// The GFF version (`gff-version`).
    GffVersion(String),
    /// A reference to a sequence segment (`sequence-region`).
    SequenceRegion(SequenceRegion),
    /// The ontology used for the feature types (`feature-ontology`).
    FeatureOntology(String),
    /// The ontology used for the attributes (`attribute-ontology`).
    AttributeOntology(String),
    /// The ontology used for the sources (`source-ontology`).
    SourceOntology(String),
    /// The species the annotations apply to (`species`).
    Species(String),
    /// The genome build used for the start and end positions (`genome-build`).
    GenomeBuild(String),
    /// A marker indicating that all forward references to feature IDs have been resolved (`#`).
    ForwardReferencesAreResolved,
    /// A marker indicating the end of the records list and start of a bundled reference sequences
    /// (`FASTA`).
    StartOfFasta,
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GffVersion(version) => write!(f, "{}gff-version {}", PREFIX, version),
            Self::SequenceRegion(sequence_region) => write!(f, "{}", sequence_region),
            Self::FeatureOntology(uri) => write!(f, "{}feature-ontology {}", PREFIX, uri),
            Self::AttributeOntology(uri) => write!(f, "{}attribute-ontology {}", PREFIX, uri),
            Self::SourceOntology(uri) => write!(f, "{}source-ontology {}", PREFIX, uri),
            Self::Species(uri) => write!(f, "{}species {}", PREFIX, uri),
            Self::GenomeBuild(args) => write!(f, "{}genome-build {}", PREFIX, args),
            Self::ForwardReferencesAreResolved => write!(f, "{}#", PREFIX),
            Self::StartOfFasta => write!(f, "{}FASTA", PREFIX),
        }
    }
}

/// An error returned when a raw GFF directive fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The directive prefix (`##`) is missing.
    MissingPrefix,
    /// The directive name is missing.
    MissingName,
    /// The directive name is invalid.
    InvalidName(String),
    /// The directive value is missing.
    MissingValue,
    /// A sequence region is invalid.
    InvalidSequenceRegion(sequence_region::ParseError),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPrefix => f.write_str("directive prefix is missing"),
            Self::MissingName => f.write_str("directive name is missing"),
            Self::InvalidName(s) => write!(f, "invalid directive name: {}", s),
            Self::MissingValue => f.write_str("directive value is missing"),
            Self::InvalidSequenceRegion(e) => write!(f, "{}", e),
        }
    }
}

impl FromStr for Directive {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(PREFIX) {
            return Err(ParseError::MissingPrefix);
        }

        let mut components = s[PREFIX.len()..].splitn(2, |c: char| c.is_ascii_whitespace());

        let name = components.next().ok_or_else(|| ParseError::MissingName)?;

        match name {
            "gff-version" => components
                .next()
                .map(|s| Self::GffVersion(s.into()))
                .ok_or_else(|| ParseError::MissingValue),
            "sequence-region" => components
                .next()
                .ok_or_else(|| ParseError::MissingValue)
                .and_then(|s| s.parse().map_err(ParseError::InvalidSequenceRegion))
                .map(Self::SequenceRegion),
            "feature-ontology" => components
                .next()
                .map(|s| Self::FeatureOntology(s.into()))
                .ok_or_else(|| ParseError::MissingValue),
            "attribute-ontology" => components
                .next()
                .map(|s| Self::AttributeOntology(s.into()))
                .ok_or_else(|| ParseError::MissingValue),
            "source-ontology" => components
                .next()
                .map(|s| Self::SourceOntology(s.into()))
                .ok_or_else(|| ParseError::MissingValue),
            "species" => components
                .next()
                .map(|s| Self::Species(s.into()))
                .ok_or_else(|| ParseError::MissingValue),
            "genome-build" => components
                .next()
                .map(|s| Self::GenomeBuild(s.into()))
                .ok_or_else(|| ParseError::MissingValue),
            "#" => Ok(Self::ForwardReferencesAreResolved),
            "FASTA" => Ok(Self::StartOfFasta),
            _ => Err(ParseError::InvalidName(name.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(
            Directive::GffVersion(String::from("3")).to_string(),
            "##gff-version 3"
        );

        let directive = Directive::SequenceRegion(SequenceRegion::new(String::from("sq0"), 8, 13));
        assert_eq!(directive.to_string(), "##sequence-region sq0 8 13");

        assert_eq!(
            Directive::FeatureOntology(String::from("https://example.com/fo.obo")).to_string(),
            "##feature-ontology https://example.com/fo.obo"
        );

        assert_eq!(
            Directive::AttributeOntology(String::from("https://example.com/ao.obo")).to_string(),
            "##attribute-ontology https://example.com/ao.obo"
        );

        assert_eq!(
            Directive::SourceOntology(String::from("https://example.com/so.obo")).to_string(),
            "##source-ontology https://example.com/so.obo"
        );

        assert_eq!(
            Directive::Species(String::from("https://example.com/species?id=1")).to_string(),
            "##species https://example.com/species?id=1"
        );

        assert_eq!(
            Directive::GenomeBuild(String::from("NCBI GRCh38.p13")).to_string(),
            "##genome-build NCBI GRCh38.p13"
        );

        assert_eq!(Directive::ForwardReferencesAreResolved.to_string(), "###");
        assert_eq!(Directive::StartOfFasta.to_string(), "##FASTA");
    }
}
