#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::aoc;

// #[aoc_generator(day06)]
// fn input_generator(inp: &str) -> &str {
//     inp
// }
use itertools::{sorted, Itertools};
use rayon::prelude::*;

#[aoc(day06, part1)]
fn part1(inp: &str) -> usize {
    let m = inp
        .split("\n\n")
        .map(|group| {
            let mut g = group.chars().filter(|&c| c != '\n').collect::<Vec<char>>();
            g.sort_unstable();
            g.dedup();
            g.iter().count()
        })
        .sum();
    return m;
}

#[aoc(day06, part2)]
fn part2(inp: &str) -> usize {
    return inp
        .split("\n\n")
        .par_bridge()
        .map(|group| {
            let numpeople = group.split("\n").count();
            let mut g = group.chars().filter(|&c| c != '\n').collect::<Vec<char>>();
            g.sort_unstable();
            let mut gc = g.clone();
            gc.dedup();
            gc.iter()
                .map(|&c| return (g.iter().filter(|&&ch| ch == c).count() == numpeople) as usize)
                .sum::<usize>()
        })
        .sum();
}
