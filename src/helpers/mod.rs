mod direction;
mod grid;
mod point;

use num::Num;
use std::{fmt::Debug, str::FromStr};

pub use super::tiles;
pub use direction::*;
pub use grid::*;
pub use point::*;

/// parses a string with no newlines by splitting whitespace, then parsing each output of that as
/// a number of the type N passed into the function
pub fn line_to_nums<N>(line: &str) -> impl Iterator<Item = N> + '_
where
    N: Num + FromStr<Err: Debug>,
{
    line.split_whitespace().map(|s| s.parse::<N>().unwrap())
}

pub fn digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

#[macro_export]
macro_rules! tiles {
    ($($char:expr => $name:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        enum Tile {
            #[default]
            $($name,)*
        }

        impl From<char> for Tile {
            fn from(value: char) -> Self {
                match value {
                    $($char => Tile::$name,)*
                    c => panic!("unknown character '{c}' trying to be parsed as a tile"),
                }
            }
        }
        impl Into<char> for Tile {
            fn into(self) -> char {
                match self {
                    $(Tile::$name => $char,)*
                }
            }
        }

        impl std::fmt::Display for Tile {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let c: char = (*self).into();
                write!(f, "{}", c)
            }
        }
    };
}
