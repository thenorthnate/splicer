//! # Gene
//!
//! Definition of a gene

use crate::elements::region::Region;
use crate::elements::Strand;


#[derive(Debug)]
pub struct Gene {
    /// protein_coding
    biotype: String,
    /// ENSG00000160072
    id: String,
    version: u32,
    /// ATAD3B
    name: String,
    score: Option<f32>,
    strand: Option<Strand>,
    gene_biotype: String,
    sequence: Region,
}

impl Gene {
    /// 1       ensembl_havana  gene    1471765 1497848 .       +       .
    /// gene_id "ENSG00000160072"; gene_version "20"; gene_name "ATAD3B"; gene_source "ensembl_havana"; gene_biotype "protein_coding";
    pub fn new(source: String, start: u32, end: u32, attributes: &str) -> Gene {

    }

    fn parse_attributes(attributes: &str)
}
