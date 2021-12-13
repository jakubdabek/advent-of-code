#![allow(unused_imports)]

use std::convert::TryFrom;
use std::detect::__is_feature_detected::xsavec;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    dots: Vec<(u16, u16)>,
    folds: Vec<(u16, u8)>,
}

#[aoc_generator(day13)]
pub fn generate(s: &str) -> Data {
    let (dots, folds) = s.split_once("\n\n").unwrap();
    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let [x, y] = [x, y].map(|coord| lexical::parse(coord).unwrap());
            (x, y)
        })
        .collect();

    let folds = folds
        .lines()
        .map(|line| {
            let (_, line) = line.rsplit_once(' ').unwrap();
            let (axis, coord) = line.split_once('=').unwrap();
            (lexical::parse(coord).unwrap(), axis.as_bytes()[0])
        })
        .collect();

    Data { dots, folds }
}

fn fold(dots: &mut Vec<(u16, u16)>, folds: impl IntoIterator<Item = (u16, u8)>) {
    for (fold, axis) in folds {
        for (x, y) in dots.iter_mut() {
            let coord = if axis == b'x' { x } else { y };
            if *coord > fold {
                let diff = *coord - fold;
                *coord = fold - diff;
            }
        }
    }

    dots.sort_unstable_by_key(|&(x, y)| (y, x));
    dots.dedup();
}

#[aoc(day13, part1)]
pub fn day13_part1(data: &Data) -> i32 {
    let mut data = data.clone();
    fold(&mut data.dots, data.folds[..1].iter().copied());
    data.dots.len() as _
}

#[aoc(day13, part2)]
pub fn day13_part2(data: &Data) -> String {
    let mut data = data.clone();
    fold(&mut data.dots, data.folds);

    let mut line = 0;
    let mut column = 0;
    for (x, y) in data.dots {
        if y > line {
            println!();
            assert_eq!(line + 1, y);
            line = y;
            column = 0;
        }

        print!("{}\u{2588}", " ".repeat((x - column) as _));
        column = x + 1;
    }
    println!();

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    fn get_example_data() -> Data {
        Data {
            dots: vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ],
            folds: vec![(7, b'y'), (5, b'x')],
        }
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day13_part1() {
        assert_eq!(super::day13_part1(&get_example_data()), 17);
    }

    #[test]
    fn day13_part2() {
        assert_eq!(super::day13_part2(&get_example_data()), "O".to_owned());
    }
}
