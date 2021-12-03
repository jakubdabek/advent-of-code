use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::lexical_parse_lines;
use aoc_utils::libs::*;

#[aoc_generator(day1)]
pub fn generate(s: &str) -> Vec<i32> {
    lexical_parse_lines(s).expect("couldn't parse input")
}

#[aoc(day1, part1)]
pub fn day1_part1(values: &[i32]) -> i32 {
    values.windows(2).filter(|s| s[0] < s[1]).count() as _
}

#[aoc(day1, part2)]
pub fn day1_part2(values: &[i32]) -> i32 {
    values.windows(4).filter(|s| s[0] < s[3]).count() as _
}

#[cfg(test)]
mod tests {
    const EXAMPLE_DATA: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn day1_part1() {
        assert_eq!(super::day1_part1(EXAMPLE_DATA), 7);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(super::day1_part2(EXAMPLE_DATA), 5);
    }
}
