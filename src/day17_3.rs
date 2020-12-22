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

use itertools::Itertools; 
use nohash_hasher::{IntSet,IntMap};

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub struct Field(HashSet<Cell>);

type Counts = HashMap<Cell, u64>;
type Neighbors = [Cell; 26];


impl Cell {
    fn neighbors(&self, neighbors: &mut Neighbors) {
        let mut i = 0;
        for x in self.x - 1..self.x + 2 {
            for y in self.y - 1..self.y + 2 {
                for z in self.z - 1..self.z + 2 {
                    if (x, y, z) != (self.x, self.y, self.z) {
                        neighbors[i] = Cell { x: x, y: y, z:z };
                        i += 1;
                    }
                }
            }
        }
    }
}
trait GameField {
    fn new() -> Self;
    fn from(desc: &str) -> Self;
    fn step() -> Self;
}

impl Field {
    fn new() -> Field {
        return Field(HashSet::new());
    }

    pub fn from(desc: &str) -> Field {
        let mut field = Field::new();

        for (y, line) in desc.split('\n').enumerate() {
            for (x, elem) in line.chars().enumerate() {
                if elem == '#' {
                    field.0.insert(Cell {
                        x: x as i64,
                        y: y as i64,
                        z: 0,
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

        let mut neighbors: Neighbors = [Cell { x: 10, y: 10, z:10 }; 26];
        for cell in &self.0 {
            cell.neighbors(&mut neighbors);
            for neighbor in &neighbors {
                let found = match counts.get_mut(neighbor) {
                    Some(count) => {
                        *count += 1;
                        true
                    }
                    None => false,
                };
                if !found {
                    counts.insert(*neighbor, 1);
                }
            }
        }

        return counts;
    }

    pub fn step(&self) -> Field {
        let mut field = Field::new();

        for (cell, count) in self.neighbor_counts() {
            if count == 3 || self.0.contains(&cell) && count == 2 {
                field.add(cell);
            }
        }

        return field;
    }

    fn to_string(&self, f: &mut dyn fmt::Write, padding: i64) -> fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "empty");
        }

        let mut minx = i64::max_value();
        let mut maxx = i64::min_value();
        let mut miny = i64::max_value();
        let mut maxy = i64::min_value();
        let mut minz = i64::max_value();
        let mut maxz = i64::min_value();

        for cell in &self.0 {
            if cell.x < minx {
                minx = cell.x;
            }
            if cell.x > maxx {
                maxx = cell.x;
            }
            if cell.y < miny {
                miny = cell.y;
            }
            if cell.y > maxy {
                maxy = cell.y;
            }
            if cell.z < minz {
                minz = cell.z;
            }
            if cell.z > maxz {
                maxz = cell.z;
            }
        }

        for z in minz - padding..maxz + 1 + padding {
            for y in miny - padding..maxy + 1 + padding {
                for x in minx - padding..maxx + 1 + padding {
                    if self.0.contains(&Cell { x: x, y: y ,z: z }) {
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
    let mut s = Field::from(inp);
    for i in 0..6{
        s = s.step();
    }
    println!("{}",s);
    return s.0.len()
}
