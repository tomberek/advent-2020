#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use scan_fmt::{scan_fmt, scan_fmt_some};
use std::collections::{HashMap,hash_map::Entry};
use nohash_hasher::{IntSet,IntMap};
use itertools::Itertools;

use array2d::Array2D;


#[aoc_generator(day15)]
fn input_generator(inp: &str) -> Vec<usize> {
    let nums = inp
        .lines()
        .map(|line| line.split(","))
        .flatten()
        .map(|p| p.parse().unwrap())
        .collect();
    return nums;
}

#[aoc(day15, part1)]
fn part1(inp: &Vec<usize>) -> usize {
    return run(2020, inp);
}
#[aoc(day15, part2)]
fn part2(inp: &Vec<usize>) -> usize {
    return run(ITER, inp);
}

const ITER: usize = 30_000_000;
const LIMIT : usize = ITER / 10;
pub fn run(iter: usize, inp: &[usize]) -> usize {
    let mut hmap: HashMap<u32,u32> = HashMap::default();
    let mut map: Vec<u32> = vec![0; LIMIT];
    let mut state = 0 as usize;
    for i in 0..inp.len() {
        let res = inp[i];
        map[state] = i as u32;
        state = res as usize;
    }
    for i in inp.len()..iter {
        if state >= LIMIT {
            match hmap.entry(state as u32) {
                Entry::Occupied(mut v) => state = i -v.insert(i as u32) as usize,
                Entry::Vacant(v) => { v.insert(i as u32); state = 0},
            }
        } else {
            let v = map[state];
            let res = if v == 0 { 0 } else { i - v as usize };
            map[state] = i as u32;
            state = res;
        }
    }
    return state;
}
