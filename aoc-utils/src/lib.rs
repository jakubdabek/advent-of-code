use std::convert::TryFrom;
use std::str::FromStr;

#[rustfmt::skip]
pub mod libs {
    pub use anyhow;
    pub use itertools;
    pub use lexical;
    pub use once_cell;
    pub use regex;
    pub use bitvec;
    pub use hex;

    pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

    pub use aoc_runner;
    pub use aoc_runner_derive;
}

#[macro_export]
macro_rules! debug_println {
    ($($tt:tt)*) => { #[cfg(debug_assertions)] { println!($($tt)*); } };
}

pub mod parse;
pub mod structs;
pub mod grids;

pub fn parse_lines<T: FromStr<Err = E>, E>(s: impl AsRef<str>) -> Result<Vec<T>, E> {
    s.as_ref().lines().map(str::parse).collect()
}

pub fn try_from_lines<'a, T: TryFrom<&'a str, Error = E>, E>(s: &'a str) -> Result<Vec<T>, E> {
    s.lines().map(T::try_from).collect()
}

pub fn lexical_parse_lines<T: lexical::FromLexical>(s: impl AsRef<str>) -> lexical::Result<Vec<T>> {
    s.as_ref().lines().map(lexical::parse).collect()
}

pub trait DefaultExt {
    fn make_default(&self) -> Self;
}

impl<T: Default> DefaultExt for T {
    fn make_default(&self) -> Self {
        T::default()
    }
}
