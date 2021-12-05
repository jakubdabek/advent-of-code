#![allow(unused_imports)]

use std::cmp::Ordering;
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

trait DispatchIterator {
    fn run<I: IntoIterator<Item = i32>>(self, i: I);
}

fn dispatch_range(from: i32, to: i32, dispatch: impl DispatchIterator) {
    match from.cmp(&to) {
        Ordering::Less => dispatch.run(from..=to),
        Ordering::Equal => dispatch.run(std::iter::repeat(from)),
        Ordering::Greater => dispatch.run((to..=from).rev()),
    }
}

fn iter_lines(data: &Data, allow_diagonal: bool, f: impl FnMut((i32, i32))) {
    let &Data {
        from: Point { x: x1, y: y1 },
        to: Point { x: x2, y: y2 },
    } = data;

    struct DispatchXs<F>(i32, i32, F);
    impl<F: FnMut((i32, i32))> DispatchIterator for DispatchXs<F> {
        fn run<I: IntoIterator<Item = i32>>(self, i: I) {
            dispatch_range(self.0, self.1, DispatchYs(i, self.2))
        }
    }

    struct DispatchYs<I, F>(I, F);
    impl<I2: IntoIterator<Item = i32>, F: FnMut((i32, i32))> DispatchIterator for DispatchYs<I2, F> {
        fn run<I: IntoIterator<Item = i32>>(self, i: I) {
            self.0.into_iter().zip(i).for_each(self.1)
        }
    }

    if allow_diagonal || (x1 == x2 || y1 == y2) {
        dispatch_range(x1, x2, DispatchXs(y1, y2, f));
    }
}

fn fill_counts(data: &Data, counts: &mut HashMap<(i32, i32), usize>, allow_diagonal: bool) -> i32 {
    let mut overlapping = 0;
    iter_lines(data, allow_diagonal, |(x, y)| {
        let entry = counts.entry((x, y)).or_insert(0);
        *entry += 1;
        if *entry == 2 {
            overlapping += 1;
        }
    });

    overlapping
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
    let mut overlapping = 0;

    for d in data.iter() {
        overlapping += fill_counts(d, &mut counts, false);
    }

    // _print_counts(&counts);
    overlapping
}

#[aoc(day5, part2)]
pub fn day5_part2(data: &[Data]) -> i32 {
    let mut counts = HashMap::new();
    let mut overlapping = 0;

    for d in data.iter() {
        overlapping += fill_counts(d, &mut counts, true);
    }

    // _print_counts(&counts);
    overlapping
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
