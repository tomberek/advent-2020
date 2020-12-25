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
use bloom::{ASMS,BloomFilter};


const EXPECTED_NUM_ITEMS :usize= 100;
const FALSE_POSITIVE_RATE : f64= 0.0001;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Decks {
    a: VecDeque<u8>,
    b: VecDeque<u8>,
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
    let mut a: VecDeque<u8> = a_lines.map(|a| a.parse().unwrap()).collect();
    a.reserve(64);

    let mut b_lines = all.next().unwrap().lines();
    b_lines.next();
    let mut b: VecDeque<u8> = b_lines.map(|a| a.parse().unwrap()).collect();
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
    return c.iter().rev().zip(1..).map(|(&a, b)| a as usize * b as usize).sum();
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
    let res = c.rev().zip(1..).map(|(&a, b)| a as u16 * b as u16).sum::<u16>() as isize;
    return res * (if decks.a.len() == 0 { -1 } else { 1 });
}
const PRIMES : [usize;84]= [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433];
const PRIMES2 : [usize;84] = [439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997];

fn play(inp: &mut Problem) -> bool {
    // Check if repeat game or if decks are empty
    let mut count = false;
    while inp.decks.a.len() != 0 && inp.decks.b.len() != 0 {
        // Hash, don't store the item.
        // let a_hash = inp.decks.a.iter().rev().zip(1..).map(|(a, b)| a * b).sum::<usize>() as isize;
        // let b_hash = inp.decks.b.iter().rev().zip(1..).map(|(a, b)| a * b).sum::<usize>() as isize;
        if count {
            let mut s = DefaultHasher::new();
            //println!("{} {}",inp.decks.a.len(),inp.decks.b.len());
            //println!("{}",h);
            inp.decks.a.hash(&mut s);
            inp.decks.b.hash(&mut s);
            let h = s.finish() as usize;

            // let ha : usize= inp.decks.a
            //     .iter()
            //     .zip(PRIMES.iter())
            //     .map(|(&a,b)|b * (a as usize )).product();
            // let hb : usize= inp.decks.b
            //     .iter()
            //     .zip(PRIMES2.iter())
            //     .map(|(&a,b)|b * (a as usize  )).product();
            // let h = ha*hb;

            if !inp.state.insert(h){
                break
            }
        }
        count = !count;

        // Length has already been checked
        let ac = inp.decks.a.pop_front().unwrap();
        let bc = inp.decks.b.pop_front().unwrap();

        // Not enough cards
        if ac > inp.decks.a.len() as u8 || bc > inp.decks.b.len() as u8 {
            match ac.cmp(&bc) {
                Ordering::Less => { inp.decks.b.push_back(bc); inp.decks.b.push_back(ac); }
                Ordering::Greater => { inp.decks.a.push_back(ac); inp.decks.a.push_back(bc); }
                Ordering::Equal => panic!("war"),
            };
        }
        // Recursive Game!
        else {
            // Copy the decks
            // Copy then truncate is faster than rebuilding
            let mut a_copy = inp.decks.a.clone();
            let mut b_copy = inp.decks.b.clone();
            a_copy.truncate(ac.into());
            b_copy.truncate(bc.into());
            let mut new_deck = Decks {
                a: a_copy,
                b: b_copy,
            };

            // let mut new_deck = Decks {
            //     a: inp.decks.a.iter().take(ac.into()).copied().collect(),
            //     b: inp.decks.b.iter().take(bc.into()).copied().collect(),
            // };
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
