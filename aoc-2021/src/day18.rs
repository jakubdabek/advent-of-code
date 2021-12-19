#![allow(unused_imports)]

use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fmt::{self, Display, Write};

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::{debug_do, try_from_lines};
use aoc_utils::grids::{print_digit, print_grid};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    number: Vec<u8>,
}

const BRANCH: u8 = u8::MAX;
const HALF_BRANCH: u8 = BRANCH - 1;

impl TryFrom<&'_ str> for Data {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let line = line.as_bytes();
        let mut index = 0;
        let mut number = Vec::new();
        while let Some(ch) = line.get(index) {
            match ch {
                b'[' => {
                    number.push(BRANCH);
                    index += 1;
                }
                b']' => index += 1,
                b',' => index += 1,
                ch if ch.is_ascii_digit() => {
                    let (value, len) = lexical::parse_partial(&line[index..])?;
                    number.push(value);
                    index += len;
                }
                _ => unreachable!(),
            }
        }
        Ok(Data { number })
    }
}

#[aoc_generator(day18)]
pub fn generate(s: &str) -> Vec<Data> {
    try_from_lines(s).expect("couldn't parse input")
}

fn print_snail(num: &[u8]) {
    let mut stack = Vec::new();

    for &v in num.iter() {
        // debug_do!{
        //     println!();
        //     print_digit(&v);
        //     print!(" ");
        //     print_grid(&[&stack], print_digit);
        // }
        if v == BRANCH {
            print!("[");
            stack.push(v);
        } else {
            let last = stack.last().copied().unwrap();
            if last == BRANCH {
                print!("{},", v);
                stack.push(v);
            } else {
                if last == HALF_BRANCH {
                    print!("{}]", v);
                    stack.pop();
                } else {
                    print!("{}", v);
                }
                while stack
                    .last()
                    .map(|&last| !matches!(last, BRANCH))
                    .unwrap_or(false)
                {
                    print!("]");
                    let pop = stack.pop().unwrap();
                    if pop == HALF_BRANCH {
                        continue;
                    }
                    let _branch = stack.pop();
                    assert_eq!(_branch, Some(BRANCH));
                }
                if let Some(last_branch) = stack.last_mut() {
                    assert!(matches!(*last_branch, BRANCH | HALF_BRANCH));
                    *last_branch = HALF_BRANCH;
                }
                print!(",");
            }
        }
    }
    // while let Some(last) = stack.pop() {
    //     assert!(matches!(last, BRANCH | HALF_BRANCH));
    //     print!("]");
    // }
    assert!(stack.is_empty());
}

fn reduce_snail(num: &mut Vec<u8>) {
    debug_do! {
        println!("Reduce number:");
        print_snail(num);
        println!();
    }
    let mut stack = Vec::new();
    let mut to_split = Vec::with_capacity(2);
    let mut done_something;
    loop {
        stack.clear();
        done_something = false;
        let mut level = 0;

        for (i, &v) in num.iter().enumerate() {
            // debug_do!{
            //     print_digit(&v);
            //     print!(" ");
            //     print_grid(&[&stack], print_digit);
            // }
            if v == BRANCH {
                stack.push(v);
                level += 1;
            } else {
                let last = stack.last().copied().unwrap();
                if last == BRANCH {
                    stack.push(v);
                } else {
                    if last == HALF_BRANCH {
                        stack.pop();
                        level -= 1;
                    }
                    // debug_do!(println!("{}: {}", level, v));
                    if level > 4 {
                        done_something = true;
                        let left = num[i - 1];
                        debug_do! {
                            println!("Before explode: [{}, {}]", left, v);
                            print_snail(num);
                            println!();
                        }
                        if let Some(prev) = num[..i - 1].iter_mut().rposition(|v| *v != BRANCH) {
                            num[prev] += left;
                            if num[prev] >= 10 {
                                to_split.push(prev);
                            }
                        }
                        if let Some(succ) = num[i + 1..].iter_mut().position(|v| *v != BRANCH) {
                            let succ = i + 1 + succ;
                            num[succ] += v;
                            if num[succ] >= 10 {
                                to_split.push(succ - 2);
                            }
                        }
                        drop(num.splice(i - 2..=i, [0]));
                        debug_do! {
                            println!("After explode:");
                            print_snail(num);
                            println!();
                        }
                        break;
                    }

                    while stack
                        .last()
                        .map(|&last| !matches!(last, BRANCH))
                        .unwrap_or(false)
                    {
                        level -= 1;
                        let pop = stack.pop().unwrap();
                        if pop == HALF_BRANCH {
                            continue;
                        }
                        let _branch = stack.pop();
                        assert_eq!(_branch, Some(BRANCH));
                    }
                    if let Some(last_branch) = stack.last_mut() {
                        assert!(matches!(*last_branch, BRANCH | HALF_BRANCH));
                        *last_branch = HALF_BRANCH;
                    }
                }
            }
        }

        if !to_split.is_empty() {
            println!("splitting: {:?}", to_split);
            for (i, mut to_split) in to_split.drain(..).enumerate() {
                if i == 1 {
                    to_split += 2;
                }
                let value = num[to_split];
                drop(num.splice(
                    to_split..=to_split,
                    [BRANCH, value / 2, value / 2 + value % 2],
                ));
                debug_do! {
                    println!("After split:");
                    print_snail(num);
                    println!();
                }
            }
        } else if !done_something {
            break;
        }
    }
}

fn sum_snail(data: &[Data]) -> Data {
    let sum = data.iter().fold(Vec::new(), |mut acc, num| {
        if acc.is_empty() {
            acc.extend(num.number.iter().copied());
            return acc;
        }
        acc.insert(0, BRANCH);
        acc.extend(num.number.iter().copied());
        reduce_snail(&mut acc);
        acc
    });

    Data{number:sum}
}

fn magnitude_snail(data: &Data) -> i32 {
    todo!()
}

#[aoc(day18, part1)]
pub fn day18_part1(data: &[Data]) -> i32 {
    let sum = sum_snail(data);

    debug_do! {
        print_snail(&sum.number);
        println!();
    }

    magnitude_snail(&sum)
}

#[aoc(day18, part2)]
pub fn day18_part2(data: &[Data]) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: &[&str] = &["[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]"];

    const SUM_EXAMPLES: &[&str] = &[
        "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]",
        "[1,1]\n[2,2]\n[3,3]\n[4,4]",
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]",
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ];

    const SUM_EXPECTED: &[&str] = &[
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    ];

    const SUM_EXPANDED_EXAMPLE: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    const SUM_EXPANDED_EXPECTED: &[&str] = &[
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
        "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
        "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
        "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
        "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
        "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
        "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    ];

    #[test]
    fn explode() {
        let mut d = Data::try_from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
        reduce_snail(&mut d.number);
        let mut expected = Data::try_from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(d, expected);
    }

    #[test]
    fn sum() {
        for (input, expected) in SUM_EXAMPLES.iter().zip_eq(SUM_EXPECTED) {
            assert_eq!(sum_snail(&super::generate(input)), super::generate(expected)[0]);
        }

        let expanded_example = super::generate(SUM_EXPANDED_EXAMPLE);
        for i in 1..expanded_example.len() {
            assert_eq!(sum_snail(&expanded_example[..i]), super::generate(SUM_EXPANDED_EXPECTED[i - 1])[0]);
        }
    }

    #[test]
    fn day18_part1() {
        for (input, data) in EXAMPLE_INPUTS.iter().zip_eq([100]) {
            assert_eq!(super::day18_part1(&super::generate(input)), data);
        }
    }

    #[test]
    fn day18_part2() {
        for (input, &data) in EXAMPLE_INPUTS.iter().zip_eq([todo!()]) {
            assert_eq!(super::day18_part2(&super::generate(input)), data);
        }
    }
}
