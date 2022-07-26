//! # GTF
//!
//! Annotion file functions and definitions

use log::{
    debug,
    error,
    info,
};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};

use crate::curator::Curator;
use crate::elements::segment::{Segment, SegmentType};


pub fn read_gtf(path: &str) -> Result<Vec<Segment>, Box<dyn Error>> {
    info!("reading and parsing {}", path);
    let file_ptr = fs::File::open(path)?;
    let mut segments = Vec::new();
    let mut i = 0;
    let mut exon_count = 0;
    for line_result in io::BufReader::new(file_ptr).lines() {
        let line = match line_result {
            Ok(v) => v,
            Err(e) => {
                error!("failed to read a line due to: {}", e);
                continue;
            }
        };
        if i < 10 {
            println!("{}", line);
        }
        if let Some(segment) = parse_gtf_entry(&line, i) {
            if segment.segment_type == SegmentType::Exon {
                exon_count += 1;
            }
            segments.push(segment);
        }
        i += 1;
    }
    info!("finished parsing {}, found {} segments, {} exons", path, segments.len(), exon_count);
    Ok(segments)
}

fn parse_gtf_entry(line: &str, line_number: u32) -> Option<Segment> {
    if line.starts_with("#") {
        // This is a comment line!
        debug!("GTF Header: {}", line);
        return None;
    }

    let elements: Vec<&str> = line.split("\t").collect();
    if elements.len() < 5 {
        return None;
    }
    let index = elements[0].parse::<u32>().ok();
    let segment_type = match elements[2] {
        "CDS" => SegmentType::CDS,
        "exon" => SegmentType::Exon,
        "gene" => SegmentType::Gene,
        "start_codon" => SegmentType::StartCodon,
        "transcript" => SegmentType::Transcript,
        _ => SegmentType::Unknown,
    };
    let start = elements[3].parse::<u32>().ok();
    let end = elements[4].parse::<u32>().ok();
    if elements.len() > 8 {
        // parse attributes!
        let attr_map = parse_attributes(elements[8]);
        if line_number < 20 {
            println!("Attrs: {:?}", attr_map);
        }
    }
    if let Some(start_val) = start {
        if let Some(end_val) = end {
            let mut new_segment = Segment::new(start_val, end_val, segment_type);
            new_segment.index = index;
            return Some(new_segment);
        }
    }
    None
}

fn parse_attributes(line: &str) -> HashMap<String, String> {
    let attribute_str = line.replace("\"", "");
    let attributes: Vec<&str> = attribute_str.split(";").collect();
    let mut attr_map = HashMap::new();
    for attr in attributes {
        let cleaned_attr = attr.strip_prefix(" ");
        if let Some(cleaned) = cleaned_attr {
            let attr_parts: Vec<&str> = cleaned.split(" ").collect();
            if attr_parts.len() == 2 {
                attr_map.insert(attr_parts[0].replace(" ", "").to_owned(), attr_parts[1].replace(" ", "").to_owned());
            }
        }
    }
    attr_map
}

// ["1", "ensembl_havana", "exon", "1471765", "1472089", ".", "+", ".", "gene_id \"ENSG00000160072\"; gene_version \"20\"; transcript_id \"ENST00000673477\"; transcript_version \"1\"; exon_number \"1\"; gene_name \"ATAD3B\"; gene_source \"ensembl_havana\"; gene_biotype \"protein_coding\"; transcript_name \"ATAD3B-206\"; transcript_source \"ensembl_havana\"; transcript_biotype \"protein_coding\"; tag \"CCDS\"; ccds_id \"CCDS30\"; exon_id \"ENSE00003889014\"; exon_version \"1\"; tag \"basic\";"]
