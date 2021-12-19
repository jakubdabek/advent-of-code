#![allow(unused_imports)]

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::hint::unreachable_unchecked;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;
use itertools::Either;
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::{try_from_lines, debug_do};

pub type Data = Vec<u8>;

#[aoc_generator(day16)]
pub fn generate<'input>(s: &'input str) -> Data {
    hex::decode(s.trim().as_bytes()).unwrap()
}

type BS = BitSlice<u8, Msb0>;
type ResInt = u64;

fn parse_packet<'a>(data: &'a BS, version_sum: &mut ResInt) -> (&'a BS, ResInt) {
    debug_do!(println!("* * parse_packet({:?})", data));
    let version = data[0..3].load_be::<u8>();
    let type_id = data[3..6].load_be::<u8>();
    let data = &data[6..];

    *version_sum += version as ResInt;

    match type_id {
        4 => parse_literal(data),
        0 => parse_operator(data, version_sum, 0, |acc, val| acc + val),
        1 => parse_operator(data, version_sum, 1, |acc, val| acc * val),
        2 => parse_operator(data, version_sum, ResInt::MAX, |acc, val| acc.min(val)),
        3 => parse_operator(data, version_sum, 0, |acc, val| acc.max(val)),
        5 | 6 | 7 => {
            let (data, value) = parse_operator(
                data,
                version_sum,
                Either::Left(None::<ResInt>),
                |acc, val| match acc {
                    Either::Left(Some(acc)) => Either::Right(acc.cmp(&val)),
                    Either::Left(None) => Either::Left(Some(val)),
                    Either::Right(_) => unreachable!(),
                },
            );
            let ord = value.unwrap_right();
            let value = match type_id {
                5 => (ord == Ordering::Greater) as _,
                6 => (ord == Ordering::Less) as _,
                7 => (ord == Ordering::Equal) as _,
                _ => unsafe { unreachable_unchecked() },
            };

            (data, value)
        }
        _ => unreachable!(),
    }
}

fn parse_literal(data: &BS) -> (&BS, ResInt) {
    debug_do!(println!("* * * parse_literal({:?})", data));
    let (value, chunks_processed) = data
        .chunks_exact(5)
        .enumerate()
        .try_fold(0, |acc, (i, chunk)| {
            let num = (acc << 4) + chunk[1..5].load_be::<u8>() as ResInt;
            debug_do!(println!("* * * parse_literal(..): {} {:?}", num, &chunk[1..5]));
            if chunk[0] {
                Ok(num)
            } else {
                Err((num, i + 1))
            }
        })
        .unwrap_err();

    debug_do!(println!("* * * parse_literal(..) = {}", value));
    (&data[chunks_processed * 5..], value)
}

fn parse_operator<'a, T>(
    data: &'a BS,
    version_sum: &mut ResInt,
    mut init: T,
    mut fold: impl FnMut(T, ResInt) -> T,
) -> (&'a BS, T) {
    debug_do!(println!("* * * parse_operator({:?})", data));

    let (length_type_id, data) = data.split_first().unwrap();
    match *length_type_id {
        false => {
            let (length, data) = data.split_at(15);
            debug_do!(println!("* * * * {:?}", length));
            let length = length.load_be::<u16>() as usize;
            debug_do!(println!("* * * * {:?}", length));

            let (mut inner_data, data) = data.split_at(length);
            while !inner_data.is_empty() {
                let (new_data, new_value) = parse_packet(inner_data, version_sum);
                inner_data = new_data;
                init = fold(init, new_value);
            }

            (data, init)
        }
        true => {
            let (count, data) = data.split_at(11);
            let count = count.load_be::<u16>();

            let mut data = data;
            for _ in 0..count {
                let (new_data, new_value) = parse_packet(data, version_sum);
                data = new_data;
                init = fold(init, new_value);
            }

            (data, init)
        }
    }
}

#[aoc(day16, part1)]
pub fn day16_part1(data: &Data) -> ResInt {
    debug_do!(println!("# part1({})", hex::encode(data)));
    let bits = data.view_bits::<Msb0>();
    let mut version_sum = 0;
    let bits = parse_packet(bits, &mut version_sum).0;
    debug_assert!(bits.not_any());
    version_sum
}

#[aoc(day16, part2)]
pub fn day16_part2(data: &Data) -> ResInt {
    debug_do!(println!("# part2({})", hex::encode(data)));
    let bits = data.view_bits::<Msb0>();
    let mut version_sum = 0;
    let (bits, value) = parse_packet(bits, &mut version_sum);
    debug_assert!(bits.not_any());
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUTS: &[&str] = &[
        "D2FE28",
        "38006F45291200",
        "EE00D40C823060",
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];

    const VERSION_SUMS: &[ResInt] = &[
        6,
        1 + 0b110 + 0b010,
        7 + 0b010 + 0b100 + 0b001,
        4 + 1 + 5 + 6,
        12,
        23,
        31,
    ];

    const EXAMPLE_INPUTS2: &[&str] = &[
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
    ];

    const PACKET_VALUES: &[ResInt] = &[3, 54, 7, 9, 1, 0, 0, 1];

    #[test]
    fn day16_part1() {
        for (input, &data) in EXAMPLE_INPUTS.iter().zip_eq(VERSION_SUMS) {
            assert_eq!(super::day16_part1(&super::generate(input)), data);
        }
    }

    #[test]
    fn day16_part2() {
        for (input, &data) in EXAMPLE_INPUTS2.iter().zip_eq(PACKET_VALUES) {
            assert_eq!(super::day16_part2(&super::generate(input)), data);
        }
    }
}
