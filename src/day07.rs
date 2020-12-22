#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt_some;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use nohash_hasher::{IntSet,IntMap};

#[aoc_generator(day07)]
fn input_generator(inp: &str) -> HashMap<Bag, Vec<Bagging>> {
    return inp.lines()
        .filter_map(|line| {
            let (adj_t, color_t, rem) =
                scan_fmt_some!(line, "{} {} bags {/.*/}", String, String, String);
            Some((adj_t?, color_t?, rem?))
        })
        .enumerate()
        .map(|(i,(adj_t, color_t, rem))| {
            let mut tgts: Vec<Bagging> = Vec::new();
            let mut rem = rem;
            while let (_, Some(num), Some(adj), Some(color), Some(rm)) = scan_fmt_some!(
                rem.as_str(), "{} {d} {} {} {/.*/}", String, usize, String, String, String) {
                tgts.push(Bagging {
                    bag: Bag { id: 0, adj, color },
                    num: num,
                });
                rem = rm;
            }
            (Bag { id: i ,adj: adj_t, color: color_t, } , tgts)
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Bag {
    id: usize,
    adj: String,
    color: String,
}
impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.adj == other.adj && self.color == other.color
    }
}
impl Eq for Bag {}
use std::hash::{Hash,Hasher};
impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.adj.hash(state);
        self.color.hash(state);
    }
}

struct Bagging {
    bag: Bag,
    num: usize,
}

#[aoc(day07, part1)]
fn part1(inp: &HashMap<Bag, Vec<Bagging>>) -> usize {
    let shiny : Bag = Bag {
        id: 0,
        adj: "shiny".to_string(),
        color:"gold".to_string(),
    };
    return inp.par_iter().map(|(bag, _)|find(&mut IntSet::default(), &shiny, &bag, &inp)).sum();
}

fn find(res: &mut IntSet<usize>, tgt: &Bag, bag: &Bag, bags: &HashMap<Bag, Vec<Bagging>>) -> usize {
    bags.get(bag).map_or(0,|bs|{ bs.iter()
        .map(|b| {
            if res.contains(&b.bag.id) || b.bag == *tgt{
                return true;
            }
            let d= find(res,tgt, &b.bag, bags);
            if d == 1 {
                res.insert(bag.id);
                return true
            } else {
                false
            }
        })
        .any(|p|p) as usize
    })
}

#[aoc(day07, part2)]
fn part2(inp: &HashMap<Bag, Vec<Bagging>>) -> usize {
    return finder( &Bag { id: 0, adj: "shiny".to_string(), color: "gold".to_string(), }, &inp);
}
fn finder(bag: &Bag, bags: &HashMap<Bag, Vec<Bagging>>) -> usize {
    bags.get(bag).map_or(0,|bs|{ bs.iter()
        .map(|b| {
            return b.num * (1 + finder(&b.bag, bags));
        })
        .sum::<usize>()
    })
}

