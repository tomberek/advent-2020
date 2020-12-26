#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use scan_fmt::{scan_fmt, scan_fmt_some};
use std::collections::HashMap;
use std::collections::HashSet;

use array2d::Array2D;

use itertools::Itertools;
use nohash_hasher::{IntMap, IntSet};
use unroll::unroll_for_loops;

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

#[unroll_for_loops]
#[aoc(day15, part2)]
fn part2(inp: &Vec<usize>) -> usize {
    // https://stackoverflow.com/questions/53691012/is-there-any-way-to-allocate-a-standard-rust-array-directly-on-the-heap-skippin
    // maybe ^^^ ?

    const ITER : usize = 30_000_000;
    let mut map = {
        let mut map: Vec<u32> = Vec::with_capacity(ITER);
        unsafe { map.set_len(ITER); };
        let slice = map.into_boxed_slice();
        // for i in &mut slice[..] {
        //     *i = 0;
        // }
        let raw_slice = Box::into_raw(slice);
        unsafe {
                Box::from_raw(raw_slice as *mut [u32; ITER])
        }
    };

    // let mut map_pre: Vec<u32> = vec![0; ITER];
    // let mut map = map_pre.into_boxed_slice();
    //let mut map: [u32;ITER] = [0; ITER];
    let mut state = 0 as usize;
    for i in 0..inp.len() {
        let res = inp[i];
        map[state] = i as u32;
        state = res as usize;
    }
    unsafe {
    for i in inp.len()..ITER {
        let &v = map.get_unchecked(state);
        let res = if v == 0 { 0 } else { i.unchecked_sub(v as usize) };
        *map.get_unchecked_mut(state) = i as u32;
        state = res;
    }
    }
    return state;
}

pub fn run(iter: usize, inp: &[usize]) -> usize {
    let mut map: Vec<u32> = vec![0; iter];
    let mut state = 0 as usize;
    for i in 0..inp.len() {
        let res = inp[i];
        map[state] = i as u32;
        state = res as usize;
    }
    for i in inp.len()..iter {
        let v = map[state];
        let res = if v == 0 { 0 } else { i - v as usize };
        map[state] = i as u32;
        state = res;
    }
    return state;
}
