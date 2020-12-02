use std::convert::TryFrom;
use std::str::FromStr;

pub fn parse_lines<T: FromStr<Err = E>, E>(s: impl AsRef<str>) -> Result<Vec<T>, E> {
    s.as_ref().lines().map(str::parse).collect()
}

pub fn try_from_lines<'a, T: TryFrom<&'a str, Error = E>, E>(s: &'a str) -> Result<Vec<T>, E> {
    s.lines().map(T::try_from).collect()
}
