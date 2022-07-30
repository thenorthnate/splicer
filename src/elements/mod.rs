//! # Elements
//!
//! Various components in a genetic sequence

#[derive(Debug)]
pub struct Gene {
    id: String,
    version: u32,
    name: String,
    source: String,
    gene_biotype: String,
    start: u32,
    end: u32,
}

#[derive(Debug)]
pub struct Transcript {

}

#[derive(Debug, PartialEq)]
pub enum Strand {
    Forward,
    Reverse,
}

#[derive(Debug, PartialEq)]
pub enum SegmentType {
    CDS,
    Exon,
    ExonFivePrime,
    ExonThreePrime,
    Gene,
    Intron,
    StartCodon,
    Transcript,
    Unknown,
}

#[derive(Debug)]
pub struct Segment {
    pub index: Option<u32>,
    pub id: Option<String>,
    pub segment_type: SegmentType,
    /// start - Start position* of the feature, with sequence numbering starting at 1.
    pub start: u32,
    /// end - End position* of the feature, with sequence numbering starting at 1.
    pub end: u32,
    pub version: Option<u32>,
    pub score: Option<f32>,
}

impl Segment {
    pub fn new(start: u32, end: u32, segment_type: SegmentType) -> Segment {
        Segment{
            index: None,
            id: None,
            segment_type: segment_type,
            start: start,
            end: end,
            version: None,
            score: None,
        }
    }
}
