#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::{scan_fmt,scan_fmt_some};
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use array2d::Array2D;

use itertools::Itertools; 
use nohash_hasher::{IntSet,IntMap};

#[derive(PartialEq, Clone, Copy,Debug)]
enum Dir {
    N(isize),
}

#[aoc(day13, part1)]
fn part1(inp: &str) -> usize {
    let mut l = inp.lines();
    let target : usize= l.next().unwrap().parse().unwrap();
    let inst : Vec<usize> =
        l.next().unwrap().split(",").filter_map(|a|a.parse().ok()).collect();
    let mut rems : Vec<(usize,usize)> = inst.iter().map(|&val|{
        (val,val - target % val)
    }).collect();
    rems.sort_by(|(_,a),(_,b)|a.partial_cmp(b).unwrap());
    println!("{:?}",rems);
    rems[0].1 * rems[0].0
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
#[aoc(day13, part2)]
fn part2_crt(inp: &str) -> usize {
    let mut l = inp.lines();
    let target : i64= l.next().unwrap().parse().unwrap();
    let inst : Vec<Option<i64>> =
        l.next().unwrap().split(",").map(|a|a.parse().ok()).collect();
    let mut modulii : Vec<i64>= vec![];
    let mut residues : Vec<i64> = vec![];
    inst.iter().enumerate().for_each(|(i,a)|
        match a {
            Some(val) => {
                modulii.push(*val);
                residues.push((val - (i as i64)) % val);
            }
            None => {}
        }
    );
    match chinese_remainder(residues.as_slice(), modulii.as_slice()) {
        Some(sol) => sol as usize,
        None      => 0
    }
}

#[aoc(day13, part2,brute)]
fn part2(inp: &str) -> i64 {
    let mut l = inp.lines();
    let target : i64= l.next().unwrap().parse().unwrap();
    let inst : Vec<Option<i64>> =
        l.next().unwrap().split(",").map(|a|a.parse().ok()).collect();
    let target : i64 = 378_786_358_533_423;
    //let target : i64 = 379786358533420;
    (target..(target+10_000_000_000_000)).into_par_iter()
        .filter(|p|p%37==0)
        .filter_map(|x|{
        inst.iter().enumerate().map(|(ind,val)|{
            match val {
                None => true,
                Some(n) => {
                    //println!("{} {} {} {} {}",x,ind,n,x%n,(n-(ind as i64))%n);
                    (x + (ind as i64)) % n == 0
                },
            }
            }).all(|p|p).then_some(x)
        }
    ).find_any(|_|true).unwrap()
}
