#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt_some;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools; 
use nohash_hasher::{IntSet,IntMap};

#[aoc_generator(day10)]
fn input_generator(inp: &str) -> Vec<usize> {
    let mut res = inp.lines()
        .filter_map(|line| {
            let num =
                scan_fmt_some!(line, "{d}", usize)?;
            Some(num)
        })
        .collect::<Vec<usize>>();
    res.push(0);
    res.sort_unstable();
    res.push(res[res.len()-1]+3);
    return res
}

#[aoc(day10, part1)]
fn part1(inp: &Vec<usize>) -> usize {
    let items = inp
        .windows(2)
        .map(|v|{
            let res = v[1]-v[0];
            if res != 1 && res != 3{
                //println!("{} {}",v[1],v[0]);
            }
            res
        }).collect::<Vec<usize>>();
    let a = items.iter().filter(|&&p|p==1).count();
    let b = items.iter().filter(|&&p|p==3).count();
    return (7 as usize).pow(a as u32) * (4 as usize).pow(b as u32)
}

#[aoc(day10, part2)]
fn part2(inp: &Vec<usize>) -> usize {
    let mut ways : Vec<usize> = vec![0;inp.len()];
    ways[0]=1;
    inp.iter().enumerate().for_each(|(i,v)|{
        &inp[i+1..].iter()
            .enumerate()
            .filter(|(_,&w)|w-v<=3)
            .for_each(|(j,_)|{
                ways[i+1+j]+=ways[i];
            });
    });
    //println!("{:?}",ways);
    return ways.last().copied().unwrap();
}

