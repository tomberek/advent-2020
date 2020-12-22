#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use scan_fmt::{scan_fmt, scan_fmt_some};
use std::collections::HashMap;
use std::collections::HashSet;

use array2d::Array2D;

use itertools::Itertools;
use nohash_hasher::{IntMap, IntSet};

struct Rules {
    map: HashMap<
        String,
        (
            std::ops::RangeInclusive<usize>,
            std::ops::RangeInclusive<usize>,
        ),
    >,
    nearby: Vec<Vec<usize>>,
    mine: Vec<usize>,
}
type RI = std::ops::RangeInclusive<usize>;
#[aoc_generator(day16)]
fn input_generator(inp: &str) -> Rules {
    let mut lines = inp.lines().rev().collect::<Vec<&str>>();
    let mut nearby = vec![];
    let mut map = HashMap::new();
    loop {
        let line = lines.pop().unwrap_or("");
        if line == "" {
            break;
        }
        //println!("{:?}",line);
        let r = scan_fmt!(line, "{[a-z ]}: {d}-{d} or {d}-{d}", _, _, _, _, _);
        match r {
            Ok((s, a, b, c, d)) => {
                map.insert(
                    s,
                    (
                        std::ops::RangeInclusive::new(a, b),
                        std::ops::RangeInclusive::new(c, d),
                    ),
                );
            }
            Err(e) => {}
        }
    }
    let mut mine = vec![];
    loop {
        let line = lines.pop().unwrap_or("");
        if line == "" {
            break;
        }
        let l: Vec<usize> = line.split(",").filter_map(|p| p.parse().ok()).collect();
        mine = l;
    }
    lines.pop();
    loop {
        let line = lines.pop().unwrap_or("");
        if line == "" {
            break;
        }
        let l: Vec<usize> = line.split(",").filter_map(|p| p.parse().ok()).collect();
        nearby.push(l);
    }
    return Rules { map, nearby, mine };
}
fn find_invalid(inp: &Rules) -> impl Iterator<Item = usize> + '_ {
    inp.nearby.iter().map(move |i| {
        i.iter()
            .map(|&v| {
                let valid = inp
                    .map
                    .values()
                    .map(|(a, b)| {
                        if (v >= *a.start() && v <= *a.end()) || (v >= *b.start() && v <= *b.end())
                        {
                            return true;
                        }
                        //println!("{} {}",v,a.start());
                        return false;
                    })
                    .any(|p| p);
                if valid {
                    return 0;
                }
                return v;
            })
            .sum::<usize>()
    })
}

#[aoc(day16, part1)]
fn part1(inp: &Rules) -> usize {
    find_invalid(&inp).sum()
}
#[aoc(day16, part2)]
fn part2(inp: &Rules) -> usize {
    let mut rows: Vec<&Vec<usize>> = inp
        .nearby
        .iter()
        .filter(|p| {
            p.iter().all(|&v| {
                inp.map.iter().any(|(k, (a, b))| {
                    if (v >= *a.start() && v <= *a.end()) || (v >= *b.start() && v <= *b.end()) {
                        return true;
                    }
                    return false;
                })
            })
        })
        .collect_vec();
    // not sure if this helps
    rows.push(&inp.mine);

    let mut possible: HashMap<String, HashSet<usize>> = inp
        .map
        .keys()
        .map(|k| (k.clone(), (0..rows[0].len()).collect()))
        .collect();

    rows.iter().for_each(|i| {
        i.iter().enumerate().for_each(|(ind, &v)| {
            inp.map.iter().for_each(|(k, (a, b))| {
                if (v >= *a.start() && v <= *a.end()) || (v >= *b.start() && v <= *b.end()) {
                    return;
                }
                let r = possible.get_mut(k).unwrap().remove(&ind);
                //println!("removing: {} {} {}",k,ind, r);
            });
        });
    });
    while possible.values().any(|p| p.len() > 1) {
        let singles: Vec<usize> = possible
            .values()
            .filter_map(|set| {
                if set.len() == 1 {
                    set.iter().next()
                } else {
                    None
                }
            })
            .copied()
            .collect();
        if singles.len() == 0 {
            panic!("not making progress");
        }
        singles.iter().for_each(|x| {
            possible.values_mut().for_each(|set| {
                if set.len() > 1 {
                    //println!("removing: {}",x);
                    set.remove(&x);
                }
            });
        });
    }
    //println!("{:?}",possible);
    possible
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                return Some(inp.mine[*v.iter().next().unwrap()]);
            }
            return None;
        })
        .product()
}
