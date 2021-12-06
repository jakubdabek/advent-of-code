// cargo-aoc limitation
#![allow(clippy::needless_lifetimes)]

extern crate aoc_utils;

use aoc_utils::libs::aoc_runner;
use aoc_utils::libs::aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

aoc_lib! { year = 2021 }
