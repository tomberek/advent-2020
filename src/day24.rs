#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::collections::VecDeque;
use std::default::Default;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use scan_fmt::{scan_fmt, scan_fmt_some, scanln_fmt};
use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};

use faster::*;
use itertools::Itertools;
use ndarray::prelude::*;
use nohash_hasher::{IntMap, IntSet};
use packed_simd::{u32x4, Simd};
use rayon::prelude::*;
use std::cell::Cell;

type C = i8;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Coord {
    x: C,
    y: C,
    //z: C,
}
impl Hash for Coord {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i8(self.x);
        state.write_i8(self.y);
    }
}
#[inline]
fn pairing(a: C, b: C) -> usize {
    let k1 = pairing_boost(a) as usize;
    let k2 = pairing_boost(b) as usize;
    unsafe {
    let k3 = k1.unchecked_add(k2);
    return (k3.unchecked_mul(k3 + 1) >> 1) + k2;
    }
}
#[inline]
fn pairing_boost(a: C) -> C {
    2 * a.abs() + ((a.signum() - 1) >> 1)
}

#[aoc_generator(day24)]
fn input_generator(inp: &str) -> Vec<Coord> {
    let nums = inp
        .lines()
        .map(|line| {
            let l: Vec<char> = line.chars().collect();
            let mut coord = Coord { 
                x: 0, 
                y: 0, 
                //z: 0 
            };
            let mut index = 0;
            while index < l.len() {
                match (l.get(index), l.get(index + 1)) {
                    (Some('e'), _) => {
                        coord.x += 1;
                        coord.y -= 1;
                    }
                    (Some('w'), _) => {
                        coord.x -= 1;
                        coord.y += 1;
                    }
                    (Some('n'), Some('w')) => {
                        coord.y += 1;
                        //coord.z -= 1;
                        index += 1;
                    }
                    (Some('n'), Some('e')) => {
                        coord.x += 1;
                        //coord.z -= 1;
                        index += 1;
                    }
                    (Some('s'), Some('w')) => {
                        coord.x -= 1;
                        //coord.z += 1;
                        index += 1;
                    }
                    (Some('s'), Some('e')) => {
                        coord.y -= 1;
                        //coord.z += 1;
                        index += 1;
                    }
                    _ => panic!("unknown"),
                }
                index += 1;
            }
            return coord;
        })
        .collect();
    return nums;
}

#[aoc(day24, part1)]
fn solve1(inp: &Vec<Coord>) -> usize {
    let mut map: HashMap<Coord, bool> = HashMap::default();
    inp.iter().for_each(|&c| {
        *map.entry(c).or_insert(false) ^= true;
    });
    // white = 0 = false
    // black = 1 = true
    map.values().filter(|&&a| a).count()
}

use dashmap::DashMap;
#[aoc(day24, part2)]
fn solve2(inp: &Vec<Coord>) -> usize {
    let mut map: HashMap<_,_> = HashMap::new();
    inp.into_iter().for_each(|&c| {
        *map.entry(c).or_insert(false) ^= true;
    });
    let mut field: HashSet<_> = map
        .into_iter()
        .filter_map(|(c, v)| if v { Some(c) } else { None })
        .collect();

    let mut counts: HashMap<_, u8 > = HashMap::new();
    for _ in 0..100 {
        counts.clear();
        field.iter().flat_map(|&c|adj(c)).for_each(|c| {
                *counts.entry(c).or_insert(0) += 1;
        });

        field = counts.iter()
            .filter(|(&current, &count)| {
            // .filter(|m|{
            //     let (&current , &count) = m.pair();
                return count==2 || (field.contains(&current) && count ==1)
                // let f = field.contains(&current);
                // if f && (count==0 || count >2) || !f && (count != 2) {
                //     return false
                // } else {
                //    return true
               // }
            })
            //.map(|m|*m.key())
            .map(|(k,_)|*k)
            .collect();
    }
    // white = 0
    // black = 1
    field.iter().count()
}

const NEIGHBORS3: [[C; 3]; 6] = [
    [0, -1, 1],
    [1, -1, 0],
    [1, 0, -1],
    [0, 1, -1],
    [-1, 1, 0],
    [-1, 0, 1],
];
const NEIGHBORS: [[C; 2]; 6] = [
    [0, -1],
    [1, -1],
    [1, 0],
    [0, 1],
    [-1, 1],
    [-1, 0],
];
fn adj_p(c: Coord) -> impl ParallelIterator<Item = Coord> {
    unsafe {
        NEIGHBORS.par_iter().map(move |v| {
            return Coord {
                x: c.x.unchecked_add(v[0]),
                y: c.y.unchecked_add(v[1]),
                //z: c.z.unchecked_add(v[2]),
            };
        })
    }
}
fn adj(c: Coord) -> impl Iterator<Item = Coord> {
    unsafe {
        NEIGHBORS.iter().map(move |v| {
            return Coord {
                x: c.x.unchecked_add(v[0]),
                y: c.y.unchecked_add(v[1]),
                //z: c.z.unchecked_add(v[2]),
            };
        })
    }
}
