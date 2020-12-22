#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use scan_fmt::{scan_fmt, scan_fmt_some};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use array2d::Array2D;

use itertools::Itertools;
use nohash_hasher::{IntMap, IntSet};

fn get_val<I, F>(it: &mut I, eval_func: F) -> Option<isize>
where
    I: Iterator<Item = char>,
    F: Fn(&mut I) -> Option<isize>,
{
    match it.next()? {
        ' ' => get_val(it, eval_func),
        '(' => eval_func(it),
        c => c.to_digit(10).map(|v| v as isize),
    }
}
fn eval<I>(it: &mut I) -> Option<isize>
where
    I: Iterator<Item = char>,
{
    let mut curr = get_val(it, eval)?;
    loop {
        curr = match it.next() {
            Some(')') => return Some(curr),
            Some('+') => curr + get_val(it, eval).expect("No rhs value for '+'"),
            Some('*') => curr * get_val(it, eval).expect("No rhs value for '*'"),
            Some(' ') => continue,
            Some(_) => panic!("Expected an operator!"),
            None => return Some(curr),
        };
    }
}
fn eval2<I>(it: &mut I) -> Option<isize>
where
    I: Iterator<Item = char>,
{
    let mut curr = get_val(it, eval2)?;
    loop {
        curr = match it.next() {
            Some(')') => return Some(curr),
            Some('+') => curr + get_val(it, eval2).expect("No rhs value for '+'"),
            Some('*') => return Some(curr * eval2(it).expect("No rhs value for '*'")),
            Some(' ') => continue,
            Some(_) => panic!("Expected an operator!"),
            None => return Some(curr),
        };
    }
}

#[aoc(day18, part1)]
fn part1(inp: &str) -> isize {
    inp.lines().filter_map(|s| eval(&mut s.chars())).sum()
}
#[aoc(day18, part2)]
fn part2(inp: &str) -> isize {
    inp.lines().filter_map(|s| eval2(&mut s.chars())).sum()
}
