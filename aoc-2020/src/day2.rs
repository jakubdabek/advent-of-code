use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use std::convert::{TryFrom, TryInto};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Line {
    range: RangeInclusive<u8>,
    letter: u8,
    password: Vec<u8>,
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s = s.as_bytes().iter();
        let bottom_bound = parse_bound(&mut s, b'-')?;
        let upper_bound = parse_bound(&mut s, b' ')?;
        let letter = *s.next().ok_or(())?;
        s.next().ok_or(())?;
        s.next().ok_or(())?; // b": "
        let password = s.as_slice().to_owned();

        fn parse_bound<'e>(
            iter: &mut impl Iterator<Item = &'e u8>,
            stop_char: u8,
        ) -> Result<u8, ()> {
            let mut value = 0;
            for b in iter {
                match b {
                    v @ b'0'..=b'9' => {
                        value = value * 10 + (v - b'0');
                    }
                    &c if c == stop_char => break,
                    _ => return Err(()),
                }
            }
            Ok(value)
        }

        Ok(Line {
            range: bottom_bound..=upper_bound,
            letter,
            password,
        })
    }
}

#[aoc_generator(day2)]
pub fn generate(s: &str) -> Vec<Line> {
    try_from_lines(s).expect("couldn't parse input")
}

#[aoc(day2, part1)]
pub fn day2_part1(values: &[Line]) -> i32 {
    values
        .iter()
        .filter(|line| {
            let count = line.password.iter().filter(|&&c| c == line.letter).count();
            line.range
                .contains(&count.try_into().expect("letter count too large"))
        })
        .count()
        .try_into()
        .expect("valid password count too large")
}
