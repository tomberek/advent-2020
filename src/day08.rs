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

#[derive(Clone, Debug)]
enum Inst {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[aoc_generator(day08)]
fn input_generator(inp: &str) -> Vec<Inst> {
    let res = inp.lines()
        .filter_map(|line| {
            let (op, num) =
                scan_fmt_some!(line, "{} {d}", String, isize);
            Some((op?, num?))
        })
        .map(|(op,num)|{
            //println!("{} {}",op, num);
            match op.as_str() {
                "acc" => Inst::Acc(num),
                "jmp" => Inst::Jmp(num),
                "nop" => Inst::Nop(num),
                _ => panic!("Unknown inst"),
            }
        })
        .collect::<Vec<Inst>>();
    return res
}


#[aoc(day08, part1)]
fn part1(inp: &Vec<Inst>) -> usize {
    let mut acc = 0;
    let mut prg = 0;
    let mut counter = 0;
    let mut state = HashSet::new();
    loop {
        counter +=1;
        if prg as usize == inp.len() {
            println!("Proper exit at: {} {} {}",counter,prg,acc);
            break
        }
        if state.contains(&prg){
            println!("Infinite loop at: {} {} {}",counter,prg,acc);
            break
        }
        state.insert(prg);
        if counter > 100 {
            println!(".");
        }
        match inp[prg as usize] {
            Inst::Nop(_) => { prg += 1},
            Inst::Acc(n) => {
                acc += n;
                prg += 1
            },
            Inst::Jmp(n) => {
                prg += n

            },
        }
    }
    return 0
}

#[aoc(day08, part2)]
fn part2(inp: &Vec<Inst>) -> usize {
    //let mut changed = HashSet::new();
    // let n = inp.map(|op|{
    //     match inp[prg as usize] {
    //         Inst::Nop(_) => { prg += 1},
    //         Inst::Acc(n) => {
    //             acc += n;
    //             prg += 1
    //         },
    //         Inst::Jmp(n) => {
    //             prg += n

    //     }
    // }).collect();

    let inp_clone= inp.clone();
    inp.iter().enumerate().for_each(|(i,p)|{
        let inp_new = &mut inp_clone.clone();
        match inp[i] {
            Inst::Nop(n) => { inp_new[i]=Inst::Jmp(n)},
            Inst::Acc(_) => {},
            Inst::Jmp(n) => { inp_new[i]=Inst::Nop(n)},
        };
        let inp = inp_new;

        let mut acc = 0;
        let mut prg = 0;
        let mut counter = 0;
        let mut state = HashSet::new();

        loop {
            counter +=1;
            if prg as usize == inp.len() {
                println!("Proper exit at: {} {} {} {}",i, counter,prg,acc);
                break
            }
            if state.contains(&prg){
                println!("Infinite loop at: {} {} {} {}",i, counter,prg,acc);
                break
            }
            state.insert(prg);
            //if counter > 100 {
            //    //println!(".");
            //}
            match inp[prg as usize] {
                Inst::Nop(_) => { prg += 1},
                Inst::Acc(n) => {
                    acc += n;
                    prg += 1
                },
                Inst::Jmp(n) => {
                    prg += n

                },
            }
        }
    });
    return 0
}
