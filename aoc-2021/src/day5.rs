#![allow(unused_imports)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Range, RangeInclusive};

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::libs::itertools::Itertools;
use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct Data {
    from: Point,
    to: Point,
}

impl Data {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Data {
            from: Point { x: x1, y: y1 },
            to: Point { x: x2, y: y2 },
        }
    }
}

impl TryFrom<&'_ str> for Data {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (x1, y1, x2, y2) = line
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|s| s.parse().ok())
            .collect_tuple()
            .context("wrong number of values")?;

        Ok(Data::new(x1, y1, x2, y2))
    }
}

#[aoc_generator(day5)]
pub fn generate(s: &str) -> Vec<Data> {
    try_from_lines(s).expect("couldn't parse input")
}

fn range_incl(a: i32, b: i32) -> RangeInclusive<i32> {
    (a.min(b)..=b.max(a))
}

fn fill_axis(
    counts: &mut HashMap<(i32, i32), usize>,
    coords: impl FnOnce() -> ((i32, i32), (i32, i32)),
    entry: impl Fn(i32, i32) -> (i32, i32),
) {
    let ((a1, b1), (a2, b2)) = coords();
    assert_eq!(a1, a2);

    for b in range_incl(b1, b2) {
        *counts.entry(entry(a1, b)).or_insert(0) += 1;
    }
}

fn iter_diagonal(data: &Data, f: impl FnMut((i32, i32))) {
    let &Data {
        from: Point { x: x1, y: y1 },
        to: Point { x: x2, y: y2 },
    } = data;
    match (x1 < x2, y1 < y2) {
        (true, true) => (x1..=x2).zip(y1..=y2).for_each(f),
        (true, false) => (x1..=x2).zip((y2..=y1).rev()).for_each(f),
        (false, true) => (x2..=x1).rev().zip(y1..=y2).for_each(f),
        (false, false) => (x2..=x1).rev().zip((y2..=y1).rev()).for_each(f),
    }
}

fn fill_counts(data: &Data, counts: &mut HashMap<(i32, i32), usize>, allow_diagonal: bool) {
    if data.from.x == data.to.x {
        fill_axis(
            counts,
            || ((data.from.x, data.from.y), (data.to.x, data.to.y)),
            |a, b| (a, b),
        );
    } else if data.from.y == data.to.y {
        fill_axis(
            counts,
            || ((data.from.y, data.from.x), (data.to.y, data.to.x)),
            |a, b| (b, a),
        );
    } else if allow_diagonal {
        iter_diagonal(data, |(x, y)| {
            *counts.entry((x, y)).or_insert(0) += 1;
        });
    }
}

fn _print_counts(counts: &HashMap<(i32, i32), usize>) {
    let (x_max, y_max) = counts.keys().fold((0, 0), |(x_max, y_max), &(x, y)| {
        (x_max.max(x), y_max.max(y))
    });
    for y in 0..=y_max {
        for x in 0..=x_max {
            print!("{} ", counts.get(&(x, y)).copied().unwrap_or(0));
        }
        println!()
    }
}

#[aoc(day5, part1)]
pub fn day5_part1(data: &[Data]) -> i32 {
    let mut counts = HashMap::new();

    for d in data.iter() {
        fill_counts(d, &mut counts, false);
    }

    // _print_counts(&counts);
    counts.values().filter(|&&c| c > 1).count() as _
}

#[aoc(day5, part2)]
pub fn day5_part2(data: &[Data]) -> i32 {
    let mut counts = HashMap::new();

    for d in data.iter() {
        fill_counts(d, &mut counts, true);
    }

    // _print_counts(&counts);
    counts.values().filter(|&&c| c > 1).count() as _
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    fn get_example_data() -> Vec<Data> {
        vec![
            Data::new(0, 9, 5, 9),
            Data::new(8, 0, 0, 8),
            Data::new(9, 4, 3, 4),
            Data::new(2, 2, 2, 1),
            Data::new(7, 0, 7, 4),
            Data::new(6, 4, 2, 0),
            Data::new(0, 9, 2, 9),
            Data::new(3, 4, 1, 4),
            Data::new(0, 0, 8, 8),
            Data::new(5, 5, 8, 2),
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day5_part1() {
        assert_eq!(super::day5_part1(&get_example_data()), 5);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(super::day5_part2(&get_example_data()), 12);
    }
}
