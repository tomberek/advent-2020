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

#[aoc_generator(day09)]
fn input_generator(inp: &str) -> Vec<usize> {
    let res = inp.lines()
        .filter_map(|line| {
            line.parse().ok()
        })
        .collect::<Vec<usize>>();
    return res
}


#[aoc(day09, part1)]
fn part1(inp: &Vec<usize>) -> usize {
    let preamble = 25;
    let mut bad = 0;
    inp.iter()
        .enumerate()
        .filter(|(i,num)|{
            if i < &preamble {
                return false
            }
            let valid = &inp[i-preamble..*i]
                .iter().tuple_combinations()
                .map(|(&a,&b)|{
                    if a + b == inp[*i] {
                        return true;
                    }
                    return false
                }).any(|p|p);
            if !valid {
                //println!("failed: {}",inp[*i]);
                bad = inp[*i];
                return true
            }
            return false
        }).map(|(a,b)|b).next().copied().unwrap()
}
#[aoc(day09, part2)]
fn part2(inp: &Vec<usize>) -> usize {
    let preamble = 25;
    let mut bad = 0;
    inp.iter()
        .enumerate()
        .map(|(i,num)|{
            if i < preamble {
                return true
            }
            let valid = &inp[i-preamble..i]
                .iter().tuple_combinations()
                .map(|(&a,&b)|{
                    if a + b == inp[i] {
                        return true;
                    }
                    return false
                }).any(|p|p);
            if !valid {
                // println!("failed: {}",inp[i]);
                bad = inp[i];
                return false
            }
            return true
        }).all(|p|p);
    inp.iter()
        .enumerate()
        .filter(|(i,num)|{
            let mut sum = 0;
            for (index,x) in inp[*i..].to_vec().iter().enumerate() {
                if sum > bad || *x > bad {
                    break
                }
                sum += x;
                if sum == bad {
                    let mut answer = inp[*i..*i+index+1].to_vec().clone();
                    answer.sort_unstable();
                    //println!("found {} {} {}",
                             // answer[0],answer[answer.len()-1],
                             // answer[0]+answer[answer.len()-1],
                             // );
                    return true
                }
            }
            return false
        })
        .next().map(|(_,a)|a).copied().unwrap()
}

