use std::cmp::Ordering;
use std::convert::TryFrom;

use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::libs::*;
use aoc_utils::libs::itertools::Itertools;
use aoc_utils::try_from_lines;

const LINE_SIZE: usize = 12;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub struct Data {
    val: [u8; LINE_SIZE],
}

impl TryFrom<&'_ str> for Data {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        // let val = lexical::parse_with_options::<_, _, {lexical::NumberFormatBuilder::from_radix(2)}>(line, &Default::default())?;
        let mut val = [b'0'; LINE_SIZE];
        val[LINE_SIZE - line.len()..].copy_from_slice(line.as_bytes());
        Ok(Data { val })
    }
}

#[aoc_generator(day3)]
pub fn generate(s: &str) -> Vec<Data> {
    try_from_lines(s).expect("couldn't parse input")
    // s.lines().next().unwrap().len(),
}

#[aoc(day3, part1)]
pub fn day3_part1(values: &[Data]) -> u32 {
    let mut counts = [0_u16; LINE_SIZE];
    for d in values {
        counts
            .iter_mut()
            .zip(d.val)
            .for_each(|(c, d)| *c += (d == b'1') as u16);
    }

    let mut common = 0_u32;
    let mut exp = 1;
    for &bit in counts.iter().rev() {
        common += exp * (bit > values.len() as u16 / 2) as u32;
        exp *= 2;
    }

    let leading_zeroes = counts.iter().take_while(|&&x| x == 0).count();
    let uncommon = !(common | (!0_u32 << (LINE_SIZE - leading_zeroes)));

    common * uncommon
}

#[aoc(day3, part2)]
pub fn day3_part2(values: &[Data]) -> u32 {
    let mut values = values.to_owned();
    values.sort_unstable();

    let leading_zeroes = values
        .last()
        .unwrap()
        .val
        .iter()
        .take_while(|&&x| x == b'0')
        .count();

    fn find_rating(mut gas_range: &[Data], leading_zeroes: usize, rev: bool) -> Data {
        let mut index = leading_zeroes;

        while gas_range.len() > 1 {
            let zeroes = gas_range.partition_point(|d| d.val[index] == b'0');

            println!("{} {}", index, zeroes);
            println!(
                "{:#?}",
                gas_range
                    .iter()
                    .map(|d| std::str::from_utf8(&d.val).unwrap())
                    .collect_vec()
            );

            let ord = {
                let ord = zeroes.cmp(&(gas_range.len() / 2));
                if rev {
                    ord.reverse()
                } else {
                    ord
                }
            };
            gas_range = match ord {
                Ordering::Less => &gas_range[zeroes..],
                Ordering::Equal if rev => &gas_range[..zeroes],
                Ordering::Equal => &gas_range[zeroes..],
                Ordering::Greater => &gas_range[..zeroes],
            };

            index += 1;
        }

        gas_range[0]
    }

    let oxygen = find_rating(&values, leading_zeroes, false);
    let co2 = find_rating(&values, leading_zeroes, true);

    let mut oxygen_value = 0_u32;
    let mut exp = 1;
    for &bit in oxygen.val.iter().rev() {
        oxygen_value += exp * (bit == b'1') as u32;
        exp *= 2;
    }
    println!("oxygen: {}", oxygen_value);

    let mut co2_value = 0_u32;
    let mut exp = 1;
    for &bit in co2.val.iter().rev() {
        co2_value += exp * (bit == b'1') as u32;
        exp *= 2;
    }
    println!("co2: {}", co2_value);

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

    fn get_example_data() -> Vec<Data> {
        vec![
            Data {
                val: *b"000000000100",
            },
            Data {
                val: *b"000000011110",
            },
            Data {
                val: *b"000000010110",
            },
            Data {
                val: *b"000000010111",
            },
            Data {
                val: *b"000000010101",
            },
            Data {
                val: *b"000000001111",
            },
            Data {
                val: *b"000000000111",
            },
            Data {
                val: *b"000000011100",
            },
            Data {
                val: *b"000000010000",
            },
            Data {
                val: *b"000000011001",
            },
            Data {
                val: *b"000000000010",
            },
            Data {
                val: *b"000000001010",
            },
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
