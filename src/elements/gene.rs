//! # Gene
//!
//! Definition of a gene

use crate::elements::sequence::Sequence;


#[derive(Debug)]
pub struct Gene {
    id: String,
    version: u32,
    name: String,
    source: String,
    gene_biotype: String,
    sequence: Sequence,
}
