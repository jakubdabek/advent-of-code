use std::convert::TryFrom;
use std::str::FromStr;

pub mod libs {
    pub use anyhow;
    pub use itertools;
    pub use lexical;
    pub use once_cell;
    pub use regex;

    pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

    pub use aoc_runner;
    pub use aoc_runner_derive;
}

pub mod structs;

pub fn parse_lines<T: FromStr<Err = E>, E>(s: impl AsRef<str>) -> Result<Vec<T>, E> {
    s.as_ref().lines().map(str::parse).collect()
}

pub fn try_from_lines<'a, T: TryFrom<&'a str, Error = E>, E>(s: &'a str) -> Result<Vec<T>, E> {
    s.lines().map(T::try_from).collect()
}

pub fn lexical_parse_lines<T: lexical::FromLexical>(s: impl AsRef<str>) -> lexical::Result<Vec<T>> {
    s.as_ref().lines().map(lexical::parse).collect()
}

pub fn for_neighbours(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
    mut f: impl FnMut((usize, usize)),
) {
    if x + 1 < width {
        f((x + 1, y));
    }
    if y + 1 < height {
        f((x, y + 1));
    }
    if x > 0 {
        f((x - 1, y));
    }
    if y > 0 {
        f((x, y - 1));
    }
}
