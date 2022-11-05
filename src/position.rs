use std::ops::Range;
#[derive(Debug, Clone, PartialEq)]
pub struct Position(pub String, pub Range<usize>);