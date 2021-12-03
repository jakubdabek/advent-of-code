use std::convert::TryFrom;
use std::str::FromStr;

pub mod libs {
    pub use anyhow;
    pub use lexical;
    pub use itertools;
    pub use regex;
    pub use once_cell;

    pub fn foo() {}

    pub use aoc_runner;
    pub use aoc_runner_derive;
}

pub fn parse_lines<T: FromStr<Err=E>, E>(s: impl AsRef<str>) -> Result<Vec<T>, E> {
    s.as_ref().lines().map(str::parse).collect()
}

pub fn try_from_lines<'a, T: TryFrom<&'a str, Error=E>, E>(s: &'a str) -> Result<Vec<T>, E> {
    s.lines().map(T::try_from).collect()
}

pub fn lexical_parse_lines<T: lexical::FromLexical>(s: impl AsRef<str>) -> lexical::Result<Vec<T>> {
    s.as_ref().lines().map(lexical::parse).collect()
}
