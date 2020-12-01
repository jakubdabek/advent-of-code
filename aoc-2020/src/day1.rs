use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::parse_lines;
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn generate(s: &str) -> Vec<i32> {
    parse_lines(s).expect("couldn't parse input")
}

#[aoc(day1, part1, online)]
pub fn day1_part1_online(values: &[i32]) -> i32 {
    let mut seen = HashSet::new();

    for v in values.iter() {
        let inverted = 2020 - v;
        if seen.contains(&inverted) {
            return v * inverted;
        } else {
            seen.insert(v);
        }
    }

    panic!("couldn't find value")
}

#[aoc(day1, part1, premade)]
pub fn day1_part1_premade(values: &[i32]) -> i32 {
    let seen: HashSet<_> = values.iter().copied().collect();

    for v in values.iter() {
        let inverted = 2020 - v;
        if seen.contains(&inverted) {
            return v * inverted;
        }
    }

    panic!("couldn't find value")
}

#[aoc(day1, part2)]
pub fn day1_part2(values: &[i32]) -> i32 {
    let seen: HashSet<_> = values.iter().copied().collect();

    for v1 in values.iter() {
        for v2 in values.iter() {
            if v1 == v2 {
                continue;
            }

            let inverted = 2020 - v1 - v2;
            if seen.contains(&inverted) {
                return v1 * v2 * inverted;
            }
        }
    }

    panic!("couldn't find the value")
}
