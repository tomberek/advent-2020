#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::{scan_fmt,scan_fmt_some};
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use array2d::Array2D;

use itertools::{Itertools,repeat_n};
use nohash_hasher::{IntSet,IntMap};

// pub struct Cell {
//     pub x: i64,
//     pub y: i64,
//     pub z: i64,
// }
#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    point : Vec<i64>,
}

pub struct Field(HashSet<Cell>,usize);

type Counts = HashMap<Cell, u64>;
type Neighbors = Vec<Cell>;


impl Cell {
    fn neighbors(&self) -> Neighbors {
        (0..self.point.len()).map(|_| &[-1, 0, 1]).multi_cartesian_product().map(|n| {
            Cell{point:self.point.iter().zip(&n).map(|(a,&b)|a+b).collect()}
        }).collect()
    }
}
trait GameField {
    fn new() -> Self;
    fn from(desc: &str) -> Self;
    fn step() -> Self;
}

impl Field {
    fn new(dims:usize) -> Field {
        return Field(HashSet::new(),dims);
    }

    pub fn from(desc: &str,dims: usize) -> Field {
        let mut field = Field::new(dims);

        for (y, line) in desc.split('\n').enumerate() {
            for (x, elem) in line.chars().enumerate() {
                if elem == '#' {
                    let mut v = vec![x as i64,y as i64];
                    v.extend(repeat_n(0,dims-2));
                    field.0.insert(Cell {
                        point: v,
                    });
                }
            }
        }
        return field;
    }

    fn add(&mut self, cell: Cell) {
        self.0.insert(cell);
    }

    fn neighbor_counts(&self) -> Counts {
        let mut counts: Counts = HashMap::new();

        for cell in &self.0 {
            let neighbors = cell.neighbors();
            for neighbor in &neighbors {
                let found = match counts.get_mut(neighbor) {
                    Some(count) => {
                        *count += 1;
                        true
                    }
                    None => false,
                };
                if !found {
                    counts.insert(neighbor.clone(), 1);
                }
            }
        }

        return counts;
    }

    pub fn step(&self) -> Field {
        let mut field = Field::new(self.0.iter().next().unwrap().point.len());

        for (cell, count) in self.neighbor_counts() {
            // include self
            if count == 3 || self.0.contains(&cell) && count == 4 {
                field.add(cell);
            }
        }

        return field;
    }

    fn to_string(&self, f: &mut dyn fmt::Write, padding: i64) -> fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "empty");
        }

        let minx = &self.0.iter().map(|c|c.point[0]).min().unwrap();
        let maxx = &self.0.iter().map(|c|c.point[0]).max().unwrap();
        let miny = &self.0.iter().map(|c|c.point[1]).min().unwrap();
        let maxy = &self.0.iter().map(|c|c.point[1]).max().unwrap();
        let minz = &self.0.iter().map(|c|c.point[2]).min().unwrap();
        let maxz = &self.0.iter().map(|c|c.point[2]).max().unwrap();

        for z in minz - padding..maxz + 1 + padding {
            for y in miny - padding..maxy + 1 + padding {
                for x in minx - padding..maxx + 1 + padding {
                    if self.0.contains(&Cell { point: vec![x,y,z] }) {
                        f.write_char('#')?;
                    } else {
                        f.write_char('.')?;
                    }
                }
                f.write_char('\n')?;
            }
            f.write_char('\n')?;
            f.write_char('\n')?;
        }

        write!(f, "")
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.to_string(f, 0);
    }
}


//#[aoc_generator(day17)]
//fn input_generator(inp: &str) -> &str {
//    return inp
//    //return Field::from(inp)
//}

#[aoc(day17, part1)]
fn part1(inp: &str) -> usize {
    let mut s = Field::from(inp,3);
    for i in 0..6{
        //println!("{}",s);
        s = s.step();
    }
    return s.0.len()
}
#[aoc(day17, part2)]
fn part2(inp: &str) -> usize {
    let mut s = Field::from(inp,4);
    for i in 0..6{
        //println!("{}",s);
        s = s.step();
    }
    return s.0.len()
}
