#![allow(dead_code)]

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day03)]
fn input_generator(inp: &str) -> Vec<Vec<bool>> {
    let nums = inp
        .lines()
        .map(|line| return line.chars().map(|c| c == '#').collect())
        .collect();
    return nums;
}

fn traverse(ps: &Vec<Vec<bool>>, rule: fn(usize, &Vec<bool>) -> bool) -> usize {
    ps.iter()
        .enumerate()
        .map(|(i, v)| {
            if rule(i, v) {
                return 1;
            }
            return 0;
        })
        .sum()
}
#[aoc(day03, part1)]
fn part1(ps: &Vec<Vec<bool>>) -> usize {
    let mut mult = 1;
    let p1 = |i: usize, v: &Vec<bool>| v[3 * i % v.len()];
    mult *= traverse(ps, p1);
    return mult;
}

#[aoc(day03, part2)]
fn part2(ps: &Vec<Vec<bool>>) -> usize {
    let mut mult = 1;
    let p1 = |i: usize, v: &Vec<bool>| v[i % v.len()];
    let p2 = |i: usize, v: &Vec<bool>| v[3 * i % v.len()];
    let p3 = |i: usize, v: &Vec<bool>| v[5 * i % v.len()];
    let p4 = |i: usize, v: &Vec<bool>| v[7 * i % v.len()];
    let p5 = |i: usize, v: &Vec<bool>| i % 2 == 0 && v[i / 2 % v.len()];
    mult *= traverse(ps, p1);
    mult *= traverse(ps, p2);
    mult *= traverse(ps, p3);
    mult *= traverse(ps, p4);
    mult *= traverse(ps, p5);
    return mult;
}
