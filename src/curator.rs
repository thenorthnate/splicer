
use std::collections::HashMap;

pub struct Curator<K, V> {
    // Binary search vector - lookup the
    ordered_exons: Vec<ExonExtract>,
    // Lookup connections graph
    // Splicing Graph
    annotated_exons: HashMap<K, V>,
}

struct ExonExtract {
    name: String,
    start: u32,
    end: u32,
}
