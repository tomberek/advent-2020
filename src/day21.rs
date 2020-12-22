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
use ndarray::prelude::*;
use itertools::Itertools;
use nohash_hasher::{IntMap, IntSet};

// #[aoc_generator(day20)]
// fn input_generator(inp: &str) -> Problem {
//     let mut tiles = HashMap::new();
//     inp.split("\n\n").for_each(|group|{
//         let mut lines = group.lines();
//         let title = lines.next().unwrap();
//     return Problem{tiles}
// }

#[aoc(day21, part1)]
fn part1(inp: &str) -> usize {
    let mut map : HashMap<&str,HashSet<&str>> = HashMap::new();
    let mut ing = HashSet::new();
    let ings : Vec<HashSet<_>>= inp.lines().map(|line|{
        let mut s = line.split(" (contains ");
        let ingredients = s.next().unwrap().split(" ").collect::<HashSet<&str>>();
        let allergens = s.next().map(|a|a[0..a.len()-1].split(", ")).unwrap().collect::<HashSet<&str>>();

        ing = ing.union(&ingredients.clone()).copied().collect();
        (&allergens).into_iter().for_each(|a|{
            map.entry(a.clone())
                .and_modify(|e|{
                    *e=e.intersection(&ingredients.clone()).copied().collect();
                })
                .or_insert(ingredients.clone());
        });
        return ingredients
    }).collect();
    let res = map.values().fold(HashSet::new(),|a,s|a.union(s).copied().collect());

    let diff = ing.difference(&res).copied().collect::<HashSet<&str>>();
    return ings.iter().map(|v|v.intersection(&diff).count()).sum()
}

#[aoc(day21, part2)]
fn part2(inp: &str) -> String {
    let mut map : HashMap<&str,HashSet<&str>> = HashMap::new();
    //let mut ing = HashSet::new();
    let ings : Vec<HashSet<_>>= inp.lines().map(|line|{
        let mut s = line.split(" (contains ");
        let ingredients = s.next().unwrap().split(" ").collect::<HashSet<&str>>();
        let allergens = s.next().map(|a|a[0..a.len()-1].split(", ")).unwrap().collect::<HashSet<&str>>();

        //ing = ing.union(&ingredients.clone()).copied().collect();
        (&allergens).into_iter().for_each(|a|{
            map.entry(a.clone())
                .and_modify(|e|{
                    *e=e.intersection(&ingredients.clone()).copied().collect();
                })
                .or_insert(ingredients.clone());
        });
        return ingredients
    }).collect();

    // let possible_bad = map.values().fold(HashSet::new(),|a,s|a.union(s).copied().collect());

    // map.iter_mut().for_each(|(&k,v)|{
    //     *v = v.intersection(&possible_bad).copied().collect();
    // });

    while map.iter().filter(|(_,v)|v.len()!=1).count() > 0 {
        let known_bad : Vec<HashSet<&str>>= map.iter()
            .filter_map(|(_,v)|(v.len()==1).then_some(v.clone()))
            .collect();
        known_bad.iter().for_each(|b|{
            map.iter_mut().for_each(|(_,v)|{
                if v.len()==1 {
                    return
                }
                *v = v.difference(b).copied().collect();
            });
        });
    }
    //println!("map: {:?}",map);
    let mut fin : Vec<(&str,&str)> = map.iter().map(|(k,v)|(*k,*v.iter().next().unwrap())).collect();
    fin.sort_by(|a,b|a.0.partial_cmp(b.0).unwrap());

    return fin.iter().map(|a|a.1).join(",").to_string()
}
