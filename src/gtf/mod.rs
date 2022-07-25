use std::collections::HashMap;

/// An entry in a GTF or GFF-Version 2 file
pub struct Entry {
    /// seqname - name of the chromosome or scaffold; chromosome names can be given with or
    /// without the 'chr' prefix. Important note: the seqname must be one used within Ensembl,
    /// i.e. a standard chromosome name or an Ensembl identifier such as a scaffold ID,
    /// without any additional content such as species or assembly. See the example GFF output below.
    sequence_name: String,
    /// source - name of the program that generated this feature, or the data source (database or project name)
    source: String,
    /// feature - feature type name, e.g. Gene, Variation, Similarity
    feature: String,
    /// start - Start position* of the feature, with sequence numbering starting at 1.
    start: u32,
    /// end - End position* of the feature, with sequence numbering starting at 1.
    end: u32,
    /// score - A floating point value.
    score: f32,
    /// strand - defined as + (forward) or - (reverse).
    strand: String,
    /// frame - One of '0', '1' or '2'. '0' indicates that the first base of the feature is
    /// the first base of a codon, '1' that the second base is the first base of a codon, and so on..
    frame: u32,
    /// attribute - A semicolon-separated list of tag-value pairs, providing additional information about each feature.
    attribute: HashMap<String, String>,
}
