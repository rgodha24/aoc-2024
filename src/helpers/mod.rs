mod direction;
mod grid;
mod point;

use std::{fmt::Debug, str::FromStr};

pub use direction::*;
pub use grid::*;
use num::Num;
pub use point::*;

/// parses a string with no newlines by splitting whitespace, then parsing each output of that as
/// a number of the type N passed into the function
pub fn line_to_nums<N>(line: &str) -> impl Iterator<Item = N> + '_
where
    N: Num + FromStr<Err: Debug>,
{
    line.split_whitespace().map(|s| s.parse::<N>().unwrap())
}
