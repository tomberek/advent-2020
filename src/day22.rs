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
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use fasthash::{metro, MetroHasher};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Decks {
    a: VecDeque<usize>,
    b: VecDeque<usize>,
}
#[derive(Debug, PartialEq, Eq)]
struct Problem<'a> {
    decks: &'a mut Decks,
    //state: &'a mut HashSet<Decks>,
    state: &'a mut IntSet<usize>,
}

#[aoc_generator(day22)]
fn input_generator(inp: &str) -> Decks {
    let mut all = inp.split("\n\n");
    let mut a_lines = all.next().unwrap().lines();
    a_lines.next();
    let mut a: VecDeque<usize> = a_lines.map(|a| a.parse().unwrap()).collect();
    a.reserve(64);

    let mut b_lines = all.next().unwrap().lines();
    b_lines.next();
    let mut b: VecDeque<usize> = b_lines.map(|a| a.parse().unwrap()).collect();
    b.reserve(64);

    return Decks { a, b };
}

#[aoc(day22, part1)]
fn part1(inp: &Decks) -> usize {
    let mut decks = inp.clone();
    //println!("{:?}",&decks);
    while decks.a.len() != 0 && decks.b.len() != 0 {
        let (ac, bc) = (decks.a.pop_front().unwrap(), decks.b.pop_front().unwrap());
        match ac.cmp(&bc) {
            Ordering::Less => {
                decks.b.push_back(bc);
                decks.b.push_back(ac);
            }
            Ordering::Greater => {
                decks.a.push_back(ac);
                decks.a.push_back(bc);
            }
            Ordering::Equal => panic!("war"),
        }
    }
    //println!("{:?}",&decks);
    let c = if decks.a.len() == 0 { decks.b } else { decks.a };
    return c.iter().rev().zip(1..).map(|(a, b)| a * b).sum();
}

#[aoc(day22, part2)]
fn part2(inp: &Decks) -> isize {
    let mut decks = inp.clone();
    let winner = play(&mut Problem {
        decks: &mut decks,
        state: &mut IntSet::default(),
    });
    let c = if !winner {
        decks.b.iter()
    } else {
        decks.a.iter()
    };
    let res = c.rev().zip(1..).map(|(a, b)| a * b).sum::<usize>() as isize;
    return res * (if decks.a.len() == 0 { -1 } else { 1 });
}

fn play(inp: &mut Problem) -> bool {
    // Check if repeat game or if decks are empty
    while inp.decks.a.len() != 0 && inp.decks.b.len() != 0 {
        // Hash, don't store the item.
        // let a_hash = inp.decks.a.iter().rev().zip(1..).map(|(a, b)| a * b).sum::<usize>() as isize;
        // let b_hash = inp.decks.b.iter().rev().zip(1..).map(|(a, b)| a * b).sum::<usize>() as isize;
        let mut s = DefaultHasher::new();
        inp.decks.hash(&mut s);
        let h = s.finish() as usize;
        if !inp.state.insert(h){
            break
        }

        // Length has already been checked
        let ac = inp.decks.a.pop_front().unwrap();
        let bc = inp.decks.b.pop_front().unwrap();

        // Not enough cards
        if ac > inp.decks.a.len() || bc > inp.decks.b.len() {
            match ac.cmp(&bc) {
                Ordering::Less => { inp.decks.b.push_back(bc); inp.decks.b.push_back(ac); }
                Ordering::Greater => { inp.decks.a.push_back(ac); inp.decks.a.push_back(bc); }
                Ordering::Equal => panic!("war"),
            };
        }
        // Recursive Game!
        else {
            // Copy the decks
            let mut new_deck = Decks {
                a: inp.decks.a.iter().take(ac).copied().collect(),
                b: inp.decks.b.iter().take(bc).copied().collect(),
            };
            let winner = play(&mut Problem {
                decks: &mut new_deck,
                state: &mut IntSet::default(),
            });
            match winner {
                false => { inp.decks.b.push_back(bc); inp.decks.b.push_back(ac); }
                true => { inp.decks.a.push_back(ac); inp.decks.a.push_back(bc); }
            }
        }
    }
    return !(inp.decks.a.len() == 0);
}
