#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::{scan_fmt,scan_fmt_some};
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use array2d::Array2D;

use itertools::Itertools; 
use nohash_hasher::{IntSet,IntMap};

#[derive(PartialEq, Clone, Copy,Debug)]
enum Dir {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}
#[aoc_generator(day12)]
fn input_generator(inp: &str) -> Vec<Dir> {
    let nums = inp
        .lines()
        .filter_map(|line|{
            let (dir,dist)  = scan_fmt!(line,"{[A-Z]}{d}",char,isize).unwrap();
            match dir {
                'N' => Some(Dir::N(dist)),
                'S' => Some(Dir::S(dist)),
                'E' => Some(Dir::E(dist)),
                'W' => Some(Dir::W(dist)),
                'L' => Some(Dir::L(dist)),
                'R' => Some(Dir::R(dist)),
                'F' => Some(Dir::F(dist)),
                _ => panic!("bad input"),
            }
        })
        .collect::<Vec<Dir>>();
    return nums
}
fn cw(inp: Dir, rot:isize)->Dir {
    return match inp {
        Dir::N(n) => Dir::E(n),
        Dir::E(n) => Dir::S(n),
        Dir::S(n) => Dir::W(n),
        Dir::W(n) => Dir::N(n),
        _ => panic!("cannot turn"),
    }
}
fn ccw(inp: Dir, rot: isize)->Dir {
    return match inp {
        Dir::N(n) => Dir::W(n),
        Dir::E(n) => Dir::N(n),
        Dir::S(n) => Dir::E(n),
        Dir::W(n) => Dir::S(n),
        _ => panic!("cannot turn"),
    }
}

#[aoc(day12, part1)]
fn part1(inp: &Vec<Dir>) -> isize {
    let (x,y,dir) = inp.iter()
        .fold( (0 as isize,0 as isize,Dir::E(0)),|(x,y,dir),op|{
            //println!("{},{}: {:?}",x,y,dir);
            match op {
                Dir::N(dist) => (x,y+dist,dir),
                Dir::E(dist) => (x+dist,y,dir),
                Dir::S(dist) => (x,y-dist,dir),
                Dir::W(dist) => (x-dist,y,dir),

                Dir::F(dist) => match dir {
                    Dir::N(_) => (x,y+dist,dir),
                    Dir::E(_) => (x+dist,y,dir),
                    Dir::S(_) => (x,y-dist,dir),
                    Dir::W(_) => (x-dist,y,dir),
                    _ => panic!("bad dir"),
                },
                Dir::R(deg) => (x,y,(0..(deg/90)).fold(dir,cw)),
                Dir::L(deg) => (x,y,(0..(deg/90)).fold(dir,ccw)),
            }
        });
    //println!("{},{}: {:?}",x,y,dir);
    return x.abs()+y.abs()
}

#[aoc(day12, part2)]
fn part2(inp: &Vec<Dir>) -> isize {
    let (x,y,dir) = inp.iter()
        .fold( (0 as isize,0 as isize,(10 as isize,1 as isize)),|(x,y,(wx,wy)),op|{
            //println!("{},{}: {},{}",x,y,wx,wy);
            match op {
                Dir::N(dist) => (x,y,(wx,wy+dist)),
                Dir::E(dist) => (x,y,(wx+dist,wy)),
                Dir::S(dist) => (x,y,(wx,wy-dist)),
                Dir::W(dist) => (x,y,(wx-dist,wy)),

                Dir::F(dist) => (0..*dist).fold((x,y,(wx,wy)),|(x,y,(wx,wy)),_|(x+wx,y+wy,(wx,wy))),

                Dir::R(deg) => (x,y,(0..(deg/90)).fold((wx,wy),|(wx,wy),_|{
                    (wy,wx * -1)
                })),
                Dir::L(deg) => (x,y,(0..(deg/90)).fold((wx,wy),|(wx,wy),_|{
                    (wy * -1,wx)
                })),
            }
        });
    //println!("{},{}: {:?}",x,y,dir);
    return x.abs()+y.abs()
}
