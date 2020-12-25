#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

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
use std::cell::*;

#[aoc_generator(day25)]
fn input_generator(inp: &str) -> Vec<usize> {
    let nums = inp.lines().map(|line| line.parse().unwrap()).collect();
    return nums;
}

#[aoc(day25, part1)]
fn solve1(inp: &Vec<usize>) -> usize {
    let size =  baby_step_giant_step(MOD,7,inp[1] ).unwrap();
    return pow(inp[0],size,MOD);

    let mut value = 1;
    let (size, public) = (1..)
        .filter_map(|i| {
            value = (value * 7) % MOD;
            if inp.contains(&value) {
                return Some((i, value.clone()));
            }
            None
        })
        .next()
        .unwrap();

    let p = if public == inp[0] {inp[1]} else {inp[0]};
    return pow(p,size,MOD);
    //return (0..size).fold(1, |s, _| (s * p) % 20201227);
}
const MOD : usize= 20201227;
use std::ops::{Rem,Mul};
use num::traits::One;

fn baby_step_giant_step(n: usize, alpha: usize, beta: usize) -> Result<usize, &'static str> {
    let m = (n as f64).sqrt().ceil() as usize;
    let mut precomp = IntMap::default();

    let mut a = 1;
    for j in 0..m {
        precomp.insert(a,j);
        a = (a * alpha) % n
        //precomp.insert(pow(alpha, j, n), j);
    }

    let invgenerator = mod_inv(pow(alpha, m, n), n);
    let mut y: usize = beta;
    let mut found = false;
    let mut res: usize = 0;

    // can search in parallel
    for i in 0..m {
        if precomp.contains_key(&y) {
            match precomp.get(&y) {
                Some(value) => res = (i * m) + value,
                None => return Err("internal error"),
            }
            found = true;
            break;
        }
        y = y * invgenerator % n;
    }

    if !found {
        return Err("not found");
    }

    Ok(res)
}

fn mod_inv(base:usize,n:usize) -> usize {
    return pow(base,n-2,n)
}
pub fn pow<T: Clone + One + Mul<T, Output = T> + Rem<Output=T> >(mut base: T, mut exp: usize,modulus: T) -> T {
    let mut acc = base.clone();
    while exp > 1 {
        exp >>= 1;
        base = (base.clone() * base) % modulus.clone();
        if exp & 1 == 1 {
            acc = (acc * base.clone()) % modulus.clone();
        }
    }
    acc
}
