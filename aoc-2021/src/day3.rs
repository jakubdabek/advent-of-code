use std::cmp::Ordering;
use std::convert::TryFrom;

use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub struct Data<'a> {
    val: &'a str,
}

impl<'a> TryFrom<&'a str> for Data<'a> {
    type Error = anyhow::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        Ok(Data { val: line })
    }
}

#[aoc_generator(day3)]
pub fn generate<'input>(s: &'input str) -> Vec<Data<'input>> {
    try_from_lines(s).expect("couldn't parse input")
}

#[aoc(day3, part1)]
pub fn day3_part1(values: &[Data<'_>]) -> u32 {
    let mut counts = vec![0_u16; values[0].val.len()];
    for d in values {
        counts
            .iter_mut()
            .zip(d.val.as_bytes())
            .for_each(|(c, &d)| *c += (d == b'1') as u16);
    }

    let common = counts
        .iter()
        .fold(0, |acc, &c| acc * 2 + (c * 2 > values.len() as u16) as u32);

    let uncommon = !common & ((1_u32 << counts.len()) - 1);

    common * uncommon
}

#[aoc(day3, part2)]
pub fn day3_part2(values: &[Data<'_>]) -> u32 {
    let mut values = values.to_owned();
    values.sort_unstable();

    fn find_rating<'a>(mut gas_range: &[Data<'a>], rev: bool) -> Data<'a> {
        let mut index = 0;

        while gas_range.len() > 1 {
            let zeroes = gas_range.partition_point(|d| d.val.as_bytes()[index] == b'0');

            let ord = (zeroes * 2).cmp(&gas_range.len());
            gas_range = match (rev, ord) {
                // more common digit or 1s if equal
                (false, Ordering::Less | Ordering::Equal) => &gas_range[zeroes..],
                (false, Ordering::Greater) => &gas_range[..zeroes],
                // less common digit or 0s if equal
                (true, Ordering::Less | Ordering::Equal) => &gas_range[..zeroes],
                (true, Ordering::Greater) => &gas_range[zeroes..],
            };

            index += 1;
        }

        gas_range[0]
    }

    let oxygen = find_rating(&values, false);
    let co2 = find_rating(&values, true);

    let oxygen_value = oxygen
        .val
        .bytes()
        .fold(0, |acc, b| acc * 2 + (b == b'1') as u32);

    let co2_value = co2
        .val
        .bytes()
        .fold(0, |acc, b| acc * 2 + (b == b'1') as u32);

    oxygen_value * co2_value
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    fn get_example_data() -> Vec<Data<'static>> {
        vec![
            Data { val: "00100" },
            Data { val: "11110" },
            Data { val: "10110" },
            Data { val: "10111" },
            Data { val: "10101" },
            Data { val: "01111" },
            Data { val: "00111" },
            Data { val: "11100" },
            Data { val: "10000" },
            Data { val: "11001" },
            Data { val: "00010" },
            Data { val: "01010" },
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day3_part1() {
        assert_eq!(super::day3_part1(&get_example_data()), 198);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(super::day3_part2(&get_example_data()), 230);
    }
}
