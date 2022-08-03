//! # Elements
//!
//! Various components in a genetic sequence

pub mod gene;
pub mod region;
pub mod segment;
pub mod transcript;

#[derive(Debug, PartialEq)]
pub enum Strand {
    Forward,
    Reverse,
}
