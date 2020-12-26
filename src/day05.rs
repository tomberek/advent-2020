#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::*;

#[aoc_generator(day05)]
fn input_generator(inp: &str) -> Vec<usize> {
    let m = inp
        .lines()
        .map(|line| {
            let l = line
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1");

            return usize::from_str_radix(l.as_str(), 2).unwrap();
        })
        .collect();
    return m;
}

#[aoc(day05, part1)]
fn part1(ps: &Vec<usize>) -> usize {
    let res = ps
        .iter()
        .map(|bits| return 8 * (bits >> 3) + (bits & 0b111))
        .max()
        .unwrap();
    return res;
}

#[aoc(day05, part2)]
fn part2(ps: &Vec<usize>) -> usize {
    let mut ps = ps.clone();
    ps.sort();
    let res = ps
        .iter()
        .tuple_windows()
        .filter(|(&a,&b)|b-a!=1)
        .max()
        .map(|(&a,&b)|a+1)
        .unwrap();
    return res;
}
