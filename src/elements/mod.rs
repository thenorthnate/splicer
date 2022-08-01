//! # Elements
//!
//! Various components in a genetic sequence

pub mod gene;
pub mod segment;
pub mod sequence;
pub mod transcript;

#[derive(Debug, PartialEq)]
pub enum Strand {
    Forward,
    Reverse,
}
