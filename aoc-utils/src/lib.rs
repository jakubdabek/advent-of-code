use std::str::FromStr;

pub fn parse_lines<T: FromStr<Err = E>, E>(s: impl AsRef<str>) -> Result<Vec<T>, E> {
    s.as_ref().lines().map(str::parse).collect()
}
