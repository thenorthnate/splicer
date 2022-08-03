//! # Curator

use std::collections::HashMap;

use crate::elements::gene::Gene;
use crate::elements::segment::Segment;
use crate::elements::transcript::Transcript;

pub struct Curator {
    source: String,
    genes: Vec<Gene>,
    segments: Vec<Segment>,
    transcripts: Vec<Transcript>,

    // // Binary search vector - lookup the
    // ordered_exons: Vec<ExonExtract>,
    // // Lookup connections graph
    // // Splicing Graph
    // annotated_exons: HashMap<K, V>,
}
