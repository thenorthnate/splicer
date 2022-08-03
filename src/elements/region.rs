//! # Region
//!
//! A start and end data structure

#[derive(Debug)]
pub struct Region {
    pub end: u32,
    pub start: u32,
}

impl Region {
    pub fn new(start: u32, end: u32) -> Region {
        Region{
            end: end,
            start: start,
        }
    }
}
