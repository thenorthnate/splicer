//! # Segment
//!
//! Contiguous region of nucleotides

use std::collections::HashMap;


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
    pub count: u32,
    /// end - End position* of the feature, with sequence numbering starting at 1.
    pub end: u32,
    /// frame - One of '0', '1' or '2'. '0' indicates that the first base of the feature is
    /// the first base of a codon, '1' that the second base is the first base of a codon, and so on..
    pub frame: Option<u8>,
    pub id: Option<String>,
    pub index: Option<u32>,
    pub next: HashMap<u32, u32>,
    /// score - A floating point value.
    pub score: Option<f32>,
    pub segment_type: SegmentType,
    /// start - Start position* of the feature, with sequence numbering starting at 1.
    pub start: u32,
    pub version: Option<u32>,
}

impl Segment {
    pub fn new(start: u32, end: u32, segment_type: SegmentType) -> Segment {
        Segment{
            count: 0,
            end: end,
            frame: None,
            id: None,
            index: None,
            next: HashMap::new(),
            score: None,
            segment_type: segment_type,
            start: start,
            version: None,
        }
    }
}


// /// An entry in a GTF or GFF-Version 2 file: https://uswest.ensembl.org/info/website/upload/gff.html
// #[derive(Debug)]
// pub struct Entry {
//     /// The number that the gene is in the file
//     gene_number: u32,
//     /// seqname - name of the chromosome or scaffold; chromosome names can be given with or
//     /// without the 'chr' prefix. Important note: the seqname must be one used within Ensembl,
//     /// i.e. a standard chromosome name or an Ensembl identifier such as a scaffold ID,
//     /// without any additional content such as species or assembly. See the example GFF output below.
//     sequence_name: String,
//     /// source - name of the program that generated this feature, or the data source (database or project name)
//     source: String,
//     /// feature - feature type name, e.g. Gene, Variation, Similarity
//     feature: String,
//     /// start - Start position* of the feature, with sequence numbering starting at 1.
//     start: u32,
//     /// end - End position* of the feature, with sequence numbering starting at 1.
//     end: u32,
//     /// score - A floating point value.
//     score: f32,
//     /// strand - defined as + (forward) or - (reverse).
//     strand: String,
//     /// frame - One of '0', '1' or '2'. '0' indicates that the first base of the feature is
//     /// the first base of a codon, '1' that the second base is the first base of a codon, and so on..
//     frame: u32,
//     /// attribute - A semicolon-separated list of tag-value pairs, providing additional information about each feature.
//     attribute: HashMap<String, String>,
// }
