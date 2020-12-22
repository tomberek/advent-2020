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
enum Op {
    // set to 1, set to 0
    Mask(usize,usize),
    Store(usize,usize),
}
#[aoc_generator(day14)]
fn input_generator(inp: &str) -> Vec<Op> {
    let nums = inp
        .lines()
        .map(|line|{
            let mask = scan_fmt!(line,"mask = {[X01]}",String).ok();
            let store = scan_fmt!(line,"mem[{d}] = {d}",usize,usize).ok();
            match (mask,store) {
                (Some(mask),None) => {
                    let res = mask.chars().rev().fold((0,0,1),|(a,b,i),c|{
                        match c {
                            'X' => (a,b,i<<1),
                            '0' => (a|i,b,i<<1),
                            '1' => (a,b|i,i<<1),
                            _ => panic!("bad parse"),
                        }
                    });
                    return Op::Mask(!(0xFFFF_FFF0_0000_0000|res.0),res.1)
                },
                (None,Some((addr,val))) => Op::Store(addr,val),
                _ => panic!("bad input"),
            }
        })
        .collect::<Vec<Op>>();
    return nums
}

#[aoc(day14, part1)]
fn part1(inp: &Vec<Op>) -> usize {
    let mut map = HashMap::new();
    let mut mask = Op::Mask(0,0);
    inp.iter().for_each(|p|{
        //println!("{:?} {:?}",p,map);
        match p {
            Op::Store(addr,val) => {
                if let Op::Mask(m0,m1) = mask {
                    //println!("{}",val&m0|m1);
                    map.insert(addr,val&m0|m1);
                }
                ()
            },
            Op::Mask(_,_) => {
                mask = *p
            }
        };
    });
    map.into_values().sum()
}

fn update(addr:usize,val:usize,i: usize,floats: usize,map: &mut HashMap<usize,usize>){
    if i == 0 {
        map.insert(addr,val);
        return
    }
    if i&floats ==0 {
        update(addr,val,i<<1,floats,map);
        return
    }
    update(addr|(i&floats),val,i<<1,floats,map);
    update(addr&!(i&floats),val,i<<1,floats,map);
    return
}

#[aoc(day14, part2)]
fn part2(inp: &Vec<Op>) -> usize {
    let mut map = HashMap::new();
    let mut mask = Op::Mask(0,0);
    inp.iter().for_each(|p|{
        //println!("{:?} {:?}",p,map);
        match p {
            Op::Store(addr,val) => {
                if let Op::Mask(m0,m1) = mask {
                    //println!("addr: 0x{:x} 0x{:x} 0x{:x} addr:0x{:x}",m0,m1, (!m0|m1), addr);
                    let floats = !(!m0|m1);
                    update(addr|m1,*val,1,floats,&mut map);
                    ////println!("floats: {:x}",floats);
                    //for p in 1..64 {
                    //    let i = 1<<(p-1);
                    //    if i&floats == 0 {
                    //        continue
                    //    }
                    //    println!("{}",p);
                    //    println!("insert {} {}",(addr|m1)|i,val);
                    //    println!("insert {} {}",(addr|m1)&!i,val);
                    //    map.insert(addr|m1|i,val);
                    //    map.insert(addr|m1&!i,val);
                    //}
                }
                ()
            },
            Op::Mask(_,_) => {
                mask = *p
            }
        };
    });
    map.into_values().sum()
}
