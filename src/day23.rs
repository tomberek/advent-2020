#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use ndarray::prelude::*;
use nohash_hasher::{IntMap, IntSet};
use rayon::prelude::*;
use scan_fmt::{scan_fmt, scan_fmt_some, scanln_fmt};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::collections::LinkedList;

use packed_simd::{Simd,u32x4};
use faster::*;

fn show(cups: &[u32]) -> String {
    let mut out = String::new();
    let mut i = 0;
    loop {
        i = cups[i] as usize;
        if i == 0 {
            break;
        }
        out.push_str(&format!("{}", i + 1));
    }
    out
}

fn to_linked(cups: &[u32]) -> Vec<u32> {
    let mut cups_linked = vec![0; cups.len()];
    for window in cups.windows(2) {
        cups_linked[window[0] as usize] = window[1];
    }
    //cups_linked[cups[0] as usize] = cups[1];
    cups_linked[cups[cups.len() - 1] as usize] = cups[0];
    cups_linked
}

 #[aoc(day23, part1)]
 fn solve1(inp: &str) -> String{
     let input : Vec<&str>= inp.lines().collect();
     let cups: Vec<_> = input[0]
         .chars()
         .map(|c| c.to_digit(10).unwrap() as u32 -1 )
         .collect();
     let mut current_cup = cups[0] as u32;
     let mut cups : Vec<_>= to_linked(cups.as_slice());
     let len = cups.len() as u32;
     for _ in 0..100 {
        step_day23(len, &mut cups, &mut current_cup);
        //println!("{}", show(cups.as_slice()));
     }
     println!("{}", show(cups.as_slice()));
     return  show(cups.as_slice())
 }

#[inline(always)]
unsafe fn prev(i: u32,m:u32) -> u32 {
    //return (i + m - 2 ) % m +1
    if m ==0 {
        std::hint::unreachable_unchecked();
    }
    return (i.unchecked_add(m).unchecked_sub(1)) % m
}
fn step_day23(len : u32, cups: &mut [u32], current_cup: &mut u32) {

    unsafe {
    if len ==0 {
        std::hint::unreachable_unchecked();
    }
    let  c = *current_cup;
    let mut dest_cup =  prev(c,len) as u32;

    let grab1 = *cups.get_unchecked(c as usize);
    let grab2 = *cups.get_unchecked(grab1 as usize);
    let grab3 = *cups.get_unchecked(grab2 as usize);
    while grab1 == dest_cup || grab2 == dest_cup || grab3 == dest_cup {
        dest_cup = prev(dest_cup ,len);
    }
    *cups.get_unchecked_mut(c as usize) = *cups.get_unchecked(grab3 as usize);
    let post_dest = *cups.get_unchecked(dest_cup as usize);
    *cups.get_unchecked_mut(dest_cup as usize) = grab1;
    *cups.get_unchecked_mut(grab1 as usize) = grab2;
    *cups.get_unchecked_mut(grab2 as usize) = grab3;
    *cups.get_unchecked_mut(grab3 as usize) = post_dest;
    *current_cup = *cups.get_unchecked(c as usize);
    }
}

#[aoc(day23, part2)]
fn solve2(inp: &str) -> String{
    let input : Vec<&str> = inp.lines().collect();
    let mut cups: Vec<u32> = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u32 - 1)
        .collect();
    cups.extend(cups.iter().copied().max().unwrap() + 1..=( 1_000_000  - 1));
    let mut current_cup = cups[0] as u32;
    let mut cups = to_linked(&cups);
    let len = cups.len() as u32;
    for _ in 0..10_000_000 {
        step_day23(len, &mut cups, &mut current_cup);
    }
    let l1 = cups[0] ;
    let l2 = cups[l1 as usize] ;
    return format!("{}",(l1  + 1) as usize * (l2 + 1) as usize );
}
