#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use scan_fmt::{scan_fmt, scan_fmt_some, scanln_fmt};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use array2d::Array2D;

use itertools::Itertools;
use nohash_hasher::{IntMap, IntSet};

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
enum Rule {
    A,
    B,
    Multi(Vec<Vec<usize>>),
}

// #[aoc_generator(day19)]
fn input_generator(inp: &str) -> Problem {
    let mut rules = HashMap::new();
    let mut lines = inp.lines();
    loop {
        if let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            if let (Some(num), Some(s)) =
                scan_fmt_some!(line, "{d}: {/[ |0-9ab\"]*/}", usize, String)
            {
                if s == "\"a\"" {
                    rules.insert(num, Rule::A);
                    continue;
                }
                if s == "\"b\"" {
                    rules.insert(num, Rule::B);
                    continue;
                }
                rules.insert(
                    num,
                    Rule::Multi(
                        s.split("|")
                            .map(|c| c.split(" ").filter_map(|n| n.parse().ok()).collect())
                            .collect(),
                    ),
                );
                continue;
            }
        }
    }
    let messages = lines.map(|s| s.to_string()).collect();
    let mut v: Vec<(usize, Rule)> = rules.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    let r: Vec<Rule> = v.iter().map(|a| a.1.clone()).collect();

    return Problem { rules: r, messages };
    //return Problem{rules,messages}
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    rules: Vec<Rule>,
    messages: Vec<String>,
}
fn is_valid<'a>(rule: usize, msg: &'a str, rules: &Vec<Rule>) -> Vec<&'a str> {
    match &rules[rule] {
        Rule::A => {
            if msg.chars().next() == Some('a') {
                return vec![&msg[1..]];
            }
        }
        Rule::B => {
            //return msg.chars().next().map_or(vec![],|a|if a == 'b' {return vec![&msg[1..]]} else {return vec![]})
            if msg.chars().next() == Some('b') {
                return vec![&msg[1..]];
            }
        }
        Rule::Multi(set) => {
            return set
                .iter()
                .flat_map(|r| {
                    r.iter().fold(vec![msg], |m, &subrule| {
                        m.iter()
                            .flat_map(|thing| is_valid(subrule, thing, rules))
                            .collect()
                    })
                })
                .collect()
        }
    }
    return vec![];
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let inp = input_generator(input);
    inp.messages
        .par_iter()
        .map(|msg| match is_valid(0, msg, &inp.rules).iter().next() {
            Some(m) => {
                if m.len() == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
            None => return 0,
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let mut inp = input_generator(input);
    inp.rules[8] = Rule::Multi(vec![vec![42], vec![42, 8]]);
    inp.rules[11] = Rule::Multi(vec![vec![42, 31], vec![42, 11, 31]]);
    inp.messages
        .par_iter()
        .map(|msg| match is_valid(0, msg, &inp.rules).iter().next() {
            Some(m) => {
                if m.len() == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
            None => return 0,
        })
        .sum()
}
