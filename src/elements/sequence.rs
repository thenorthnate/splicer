//! # Sequence
//!
//! A start and end data structure

#[derive(Debug)]
pub struct Sequence {
    pub end: u32,
    pub start: u32,
}

impl Sequence {
    pub fn new(start: u32, end: u32) -> Sequence {
        Sequence{
            end: end,
            start: start,
        }
    }
}
