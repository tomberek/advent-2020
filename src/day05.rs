#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use aoc_runner_derive::{aoc, aoc_generator};

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
    let mut last = 0;
    let res = ps
        .iter()
        .map(|bits| {
            println!("{} {}", last, bits);
            if bits - last > 1 {
                let bits = 562;
                let id = 8 * (bits >> 3) + (bits & 0b111);
                println!("{} {} {}", last, bits, id);
            }
            last = *bits;
            return bits;
        })
        .max()
        .unwrap();
    return *res;
}
