use log::{debug, info};
use std::fs;
use std::error::Error;

use crate::segment::{Segment, SegmentType};

pub fn read_gtf(path: &str) -> Result<Vec<Segment>, Box<dyn Error>> {
    info!("about to read {}", path);
    let raw_data = fs::read_to_string(path)?;
    info!("finished reading {} into memory, starting parsing", path);
    let lines_str = raw_data.lines();
    let mut segments = Vec::new();
    let mut i = 0;
    for line in lines_str {
        if i < 20 {
            println!("{}", line);
        }
        if let Some(segment) = parse_gtf_entry(line) {
            segments.push(segment);
        }
        i += 1;
    }
    info!("finished parsing {}, found {} segments", path, segments.len());
    Ok(segments)
}

fn parse_gtf_entry(line: &str) -> Option<Segment> {
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
        // let attribute_str = elements[8].replace("\"", "");
        // let attributes: Vec<&str> = attribute_str.split(";").collect();
        // for attr in attributes {

        // }
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

// ["1", "ensembl_havana", "exon", "1471765", "1472089", ".", "+", ".", "gene_id \"ENSG00000160072\"; gene_version \"20\"; transcript_id \"ENST00000673477\"; transcript_version \"1\"; exon_number \"1\"; gene_name \"ATAD3B\"; gene_source \"ensembl_havana\"; gene_biotype \"protein_coding\"; transcript_name \"ATAD3B-206\"; transcript_source \"ensembl_havana\"; transcript_biotype \"protein_coding\"; tag \"CCDS\"; ccds_id \"CCDS30\"; exon_id \"ENSE00003889014\"; exon_version \"1\"; tag \"basic\";"]
