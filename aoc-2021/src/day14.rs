#![allow(unused_imports)]

use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    polymer: [[u64; 26]; 26],
    last: u8,
    rules: Vec<((u8, u8), u8)>,
}

#[aoc_generator(day14)]
pub fn generate(s: &str) -> Data {
    let (polymer, rules) = s.split_once("\n\n").unwrap();

    let (polymer, last) = {
        let mut polymer_table = [[0; 26]; 26];
        for p in polymer.as_bytes().windows(2) {
            polymer_table[(p[0] - b'A') as usize][(p[1] - b'A') as usize] += 1;
        }
        let last = *polymer.as_bytes().last().unwrap() - b'A';
        (polymer_table, last)
    };

    let rules = rules
        .lines()
        .map(|line| {
            let (pair, insert) = line.split_once(" -> ").unwrap();
            let pair = pair.as_bytes();
            let [p1, p2, i] = [pair[0], pair[1], insert.as_bytes()[0]].map(|b| b - b'A');
            ((p1, p2), i)
        })
        .collect();

    Data {
        polymer,
        last,
        rules,
    }
}

fn print_freqs(data: &Data) {
    for (i, p) in data.polymer.iter().enumerate() {
        println!("{}: {:?}", (i as u8 + b'A') as char, p);
    }
    println!();
}

fn polymerize(data: &Data, steps: usize) -> u64 {
    let mut data = data.clone();

    for _step in 0..steps {
        #[allow(clippy::clone_on_copy)]
        let mut new_polymer = data.polymer.clone();
        for &((p1, p2), insert) in data.rules.iter() {
            let [p1, p2, insert] = [p1, p2, insert].map(|l| l as usize);
            new_polymer[p1][p2] -= data.polymer[p1][p2];
            new_polymer[p1][insert] += data.polymer[p1][p2];
            new_polymer[insert][p2] += data.polymer[p1][p2]
        }
        data.polymer = new_polymer;

        #[cfg(debug_assertions)]
        {
            println!("After step {}", _step + 1);
            print_freqs(&data);
        }
    }

    #[cfg(debug_assertions)]
    {
        print_freqs(&data);

        let mut freq = data
            .polymer
            .iter()
            .enumerate()
            .map(|(i, follow)| {
                let follow = follow.iter().sum::<u64>();
                if i == data.last as usize {
                    ((i as u8 + b'A') as char, follow + 1)
                } else {
                    ((i as u8 + b'A') as char, follow)
                }
            })
            .collect_vec();

        freq.sort_by_key(|&(_, f)| f);
        println!("{:?}", freq);
    }

    let (min, max) = data
        .polymer
        .iter()
        .enumerate()
        .map(|(i, follow)| {
            let follow = follow.iter().sum::<u64>();
            if i == data.last as usize {
                follow + 1
            } else {
                follow
            }
        })
        .filter(|&f| f != 0)
        .minmax()
        .into_option()
        .unwrap();

    (max - min) as _
}

#[aoc(day14, part1)]
pub fn day14_part1(data: &Data) -> u64 {
    polymerize(data, 10)
}

#[aoc(day14, part2)]
pub fn day14_part2(data: &Data) -> u64 {
    polymerize(data, 40)
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    fn get_example_data() -> Data {
        Data {
            polymer: {
                let mut polymer = Default::default();

                if false {
                    polymer
                } else {
                    polymer[(b'N' - b'A') as usize][(b'N' - b'A') as usize] = 1;
                    polymer[(b'N' - b'A') as usize][(b'C' - b'A') as usize] = 1;
                    polymer[(b'C' - b'A') as usize][(b'B' - b'A') as usize] = 1;

                    polymer
                }
            },
            last: b'B' - b'A',
            rules: vec![
                ((b'C' - b'A', b'H' - b'A'), b'B' - b'A'),
                ((b'H' - b'A', b'H' - b'A'), b'N' - b'A'),
                ((b'C' - b'A', b'B' - b'A'), b'H' - b'A'),
                ((b'N' - b'A', b'H' - b'A'), b'C' - b'A'),
                ((b'H' - b'A', b'B' - b'A'), b'C' - b'A'),
                ((b'H' - b'A', b'C' - b'A'), b'B' - b'A'),
                ((b'H' - b'A', b'N' - b'A'), b'C' - b'A'),
                ((b'N' - b'A', b'N' - b'A'), b'C' - b'A'),
                ((b'B' - b'A', b'H' - b'A'), b'H' - b'A'),
                ((b'N' - b'A', b'C' - b'A'), b'B' - b'A'),
                ((b'N' - b'A', b'B' - b'A'), b'B' - b'A'),
                ((b'B' - b'A', b'N' - b'A'), b'B' - b'A'),
                ((b'B' - b'A', b'B' - b'A'), b'N' - b'A'),
                ((b'B' - b'A', b'C' - b'A'), b'B' - b'A'),
                ((b'C' - b'A', b'C' - b'A'), b'N' - b'A'),
                ((b'C' - b'A', b'N' - b'A'), b'C' - b'A'),
            ],
        }
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day14_part1() {
        assert_eq!(super::day14_part1(&get_example_data()), 1588);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(super::day14_part2(&get_example_data()), 2188189693529);
    }
}
