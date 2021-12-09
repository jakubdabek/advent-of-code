#![allow(unused_imports)]

use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct Data<'a> {
    input: [&'a str; 10],
    output: [&'a str; 4],
}

fn collect_arr<I: IntoIterator, const N: usize>(it: I) -> [I::Item; N] {
    let mut it = it.into_iter();
    let arr = [(); N].map(|_| it.next().unwrap());
    assert!(it.next().is_none());
    arr
}

impl<'a> TryFrom<&'a str> for Data<'a> {
    type Error = anyhow::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let (input, output) = line.split_once('|').context("split |")?;

        Ok(Data {
            input: collect_arr(input.trim().split_ascii_whitespace()),
            output: collect_arr(output.trim().split_ascii_whitespace()),
        })
    }
}

#[aoc_generator(day8)]
pub fn generate<'input>(s: &'input str) -> Vec<Data<'input>> {
    try_from_lines(s).expect("couldn't parse input")
}

#[aoc(day8, part1)]
pub fn day8_part1(data: &[Data<'_>]) -> i32 {
    data.iter()
        .map(|d| {
            d.output
                .iter()
                .filter(|display| matches!(display.len(), 2..=4 | 7))
                .count()
        })
        .sum::<usize>() as _
}

const _DIGITS: &str = "
1:     4:     7:      8:
 ....   ....   aaaa    aaaa
.    c b    c .    c  b    c
.    c b    c .    c  b    c
 ....   dddd   ....    dddd
.    f .    f .    f  e    f
.    f .    f .    f  e    f
 ....   ....   ....    gggg

0:      2:      3:      5:      6:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa    aaaa
b    c  .    c  .    c  b    .  b    .  b    c
b    c  .    c  .    c  b    .  b    .  b    c
 ....    dddd    dddd    dddd    dddd    dddd
e    f  e    .  .    f  .    f  e    f  .    f
e    f  e    .  .    f  .    f  e    f  .    f
 gggg    gggg    gggg    gggg    gggg    gggg";

const F: bool = false;
const T: bool = true;

//   a  b  c  d  e  f  g
const LOOKUP: [[bool; 7]; 10] = [
    [T, T, T, F, T, T, T], // 0
    [F, F, T, F, F, T, F], // 1
    [T, F, T, T, T, F, T], // 2
    [T, F, T, T, F, T, T], // 3
    [F, T, T, T, F, T, F], // 4
    [T, T, F, T, F, T, T], // 5
    [T, T, F, T, T, T, T], // 6
    [T, F, T, F, F, T, F], // 7
    [T, T, T, T, T, T, T], // 8
    [T, T, T, T, F, T, T], // 9
];

#[aoc(day8, part2)]
pub fn day8_part2(data: &[Data<'_>]) -> i32 {
    let mut rest = Vec::with_capacity(6);
    let mut sum = 0;

    for data in data {
        let (mut d1, mut d4, mut d7, mut d8) = ("", "", "", "");
        rest.clear();
        for display in data.input {
            match display.len() {
                2 => d1 = display,
                4 => d4 = display,
                3 => d7 = display,
                7 => d8 = display,
                _ => rest.push(display),
            }
        }
        debug_assert_ne!(d1, "");
        debug_assert_ne!(d4, "");
        debug_assert_ne!(d7, "");
        debug_assert_ne!(d8, "");

        let a = d7.bytes().find(|ch| !d1.as_bytes().contains(ch)).unwrap();
        let g = (b'a'..=b'g')
            .filter(|&ch| ch != a)
            .find(|ch| rest.iter().all(|disp| disp.as_bytes().contains(ch)))
            .unwrap();
        let bd: [u8; 2] = collect_arr(d4.bytes().filter(|ch| !d1.as_bytes().contains(ch)));
        let d = bd
            .iter()
            .copied()
            .find(|ch| {
                rest.iter()
                    .filter(|disp| !disp.as_bytes().contains(ch))
                    .count()
                    == 1
            })
            .unwrap();
        debug_assert!(bd.contains(&d));
        let b = bd.iter().copied().find(|&ch| ch != d).unwrap();

        let (mut c, mut e, mut f) = (0, 0, 0);
        for candidate in (b'a'..=b'g').filter(|ch| ![a, b, d, g].contains(ch)) {
            let missing_count = rest
                .iter()
                .filter(|disp| !disp.as_bytes().contains(&candidate))
                .count();
            match missing_count {
                1 => f = candidate,
                2 => c = candidate,
                3 => e = candidate,
                _ => unreachable!(),
            }
        }
        debug_assert_ne!(c, 0);
        debug_assert_ne!(e, 0);
        debug_assert_ne!(f, 0);

        let segment_letters = [a, b, c, d, e, f, g];

        let mut number = 0;
        for output in data.output.iter().copied() {
            let mut segments = [false; 7];
            for ch in output.bytes() {
                segments[segment_letters.iter().position(|&chh| ch == chh).unwrap()] = true;
            }

            let value = LOOKUP.iter().position(|s| s == &segments).unwrap();
            number = number * 10 + value as i32;
        }

        sum += number;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |\
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |\
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |\
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |\
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |\
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |\
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |\
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |\
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |\
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |\
fgae cfgab fg bagce";

    fn get_example_data() -> Vec<Data<'static>> {
        vec![
            Data {
                input: [
                    "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                    "fabcd", "edb",
                ],
                output: ["fdgacbe", "cefdb", "cefbgd", "gcbe"],
            },
            Data {
                input: [
                    "edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde",
                    "gfcbed", "gfec",
                ],
                output: ["fcgedb", "cgb", "dgebacf", "gc"],
            },
            Data {
                input: [
                    "fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb",
                    "cdgabef",
                ],
                output: ["cg", "cg", "fdcagb", "cbg"],
            },
            Data {
                input: [
                    "fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca",
                    "fcdbega",
                ],
                output: ["efabcd", "cedba", "gadfec", "cb"],
            },
            Data {
                input: [
                    "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab",
                    "fcbdga",
                ],
                output: ["gecf", "egdcabf", "bgf", "bfgea"],
            },
            Data {
                input: [
                    "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                    "bafgc", "acf",
                ],
                output: ["gebdcfa", "ecba", "ca", "fadegcb"],
            },
            Data {
                input: [
                    "dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb",
                    "gdcebf", "gf",
                ],
                output: ["cefg", "dcbef", "fcge", "gbcadfe"],
            },
            Data {
                input: [
                    "bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg",
                    "gebcd",
                ],
                output: ["ed", "bcgafe", "cdgba", "cbgef"],
            },
            Data {
                input: [
                    "egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb",
                    "bfceg",
                ],
                output: ["gbdfcae", "bgc", "cg", "cgb"],
            },
            Data {
                input: [
                    "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                    "fegbdc",
                ],
                output: ["fgae", "cfgab", "fg", "bagce"],
            },
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day8_part1() {
        assert_eq!(super::day8_part1(&get_example_data()), 26);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(super::day8_part2(&get_example_data()), 61229);
    }
}
