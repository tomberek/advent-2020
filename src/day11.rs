#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt_some;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use array2d::Array2D;

use itertools::Itertools; 
use nohash_hasher::{IntSet,IntMap};

#[derive(PartialEq, Clone, Copy,Debug)]
enum Point {
    Floor,
    Empty,
    Occupied,
}
#[aoc_generator(day11)]
fn input_generator(inp: &str) -> Array2D<Point> {
    let nums = inp
        .lines()
        .map(|line| return line.chars().map(|c|{
            match c {
                '.' => Point::Floor,
                'L' => Point::Empty,
                '#' => Point::Occupied,
                _ => panic!("bad input"),
            }
        }).collect())
        .collect::<Vec<Vec<Point>>>();
    return Array2D::from_rows(&nums);
}
fn adjacent(inp: &Array2D<Point>, i: usize,j: usize) -> Vec<Point> {
    return NEIGHBORS.iter().filter_map(|a|{
            return inp.get(
                (i as isize + a[0]) as usize,
                (j as isize + a[1]) as usize)
        }).map(|&a|a).collect::<Vec<Point>>()
}

#[aoc(day11, part1)]
fn part1(inp: &Array2D<Point>) -> usize {
    //let mut current = Array2D::from_rows(&inp.as_rows()[0..2]);
    let mut current = inp.clone();
    let mut iter = 0;
    loop {
        let mut new = Array2D::from_rows(&current.as_rows());
        let mut change = false;
        for (i,row_iter) in current.rows_iter().enumerate() {
            for (j,&element) in row_iter.enumerate() {
                if element == Point::Floor {
                    continue;
                }
                let a = adjacent(&current,i,j);
                let occupied = a.iter().filter(|&&p|p==Point::Occupied).count();
                //println!("{} {} {} {:#?}",i,j,occupied,a);
                match element {
                    Point::Empty => {
                        if occupied == 0 {
                            let _ = new.set(i,j,Point::Occupied);
                            change = true;
                        }
                    },
                    Point::Occupied => {
                        if occupied >= 4 {
                            let _ = new.set(i,j,Point::Empty);
                            change = true;
                        }
                    },

                    _ => {},
                }
            }
        }
        iter +=1;
        current = Array2D::from_rows(&new.as_rows());
        if iter > 200 || change == false {
            break
        }
    }
    // println!("iter {:#?}",current);
    // println!("iter {}",iter);
    // 5032 too hight
    current.elements_row_major_iter().filter(|&&p|p==Point::Occupied).count()
}

const NEIGHBORS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, -0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
fn adjacent_2(inp: &Array2D<Point>, i: usize,j: usize) -> Vec<Point> {
    let ip = if i==0 {1} else { i };
    let jp = if j==0 {1} else { j };
    // return (0..=2).map(|r|{
    //     (0..=2).filter_map(|c|{
    //         if r == 1 && c == 1 {
    //             return None
    //         }
    //         let rp = r as isize - 1;
    //         let cp = c as isize - 1;
    return NEIGHBORS.iter().filter_map(|a|{
            let rp = a[0];
            let cp = a[1];
            let mut x = rp + i as isize;
            let mut y = cp + j as isize;

            while x >= 0 && y >= 0 && (x as usize) < inp.num_rows() && (y as usize) < inp.num_columns() {
                //println!("{} {},{} {}",x,y,rp,cp);
                match inp.get(x as usize,y as usize) {
                    Some(Point::Floor) => {
                        x+=rp;
                        y+=cp;
                        continue
                    },
                    Some(Point::Occupied) => return Some(Point::Occupied),
                    Some(Point::Empty) => return Some(Point::Empty),
                    _ => return None,
                };
            }
            return None
        }).map(|a|a).collect::<Vec<Point>>()
}

#[aoc(day11, part2)]
fn part2(inp: &Array2D<Point>) -> usize {
    //let mut current = Array2D::from_rows(&inp.as_rows()[0..2]);
    let mut current = inp.clone();
    let mut iter = 0;

    let mut new = current.clone();

    loop {
        let mut change = false;
        for (i,row_iter) in current.rows_iter().enumerate() {
            for (j,&element) in row_iter.enumerate() {
                // println!("{} {} {}",i,j,occupied);
                //println!("{} {} {} {:#?}",i,j,occupied,a);
                match element {
                    Point::Empty => {
                        let a = adjacent_2(&current,i,j);
                        let occupied = a.iter().filter(|&&p|p==Point::Occupied).count();
                        if occupied == 0 {
                            let _ = new.set(i,j,Point::Occupied);
                            change = true;
                        }
                    },
                    Point::Occupied => {
                        let a = adjacent_2(&current,i,j);
                        let occupied = a.iter().filter(|&&p|p==Point::Occupied).count();
                        if occupied >= 5 {
                            let _ = new.set(i,j,Point::Empty);
                            change = true;
                        }
                    },
                    Point::Floor => {},
                }
            }
        }
        iter +=1;
        current = new.clone();
        //let (mut current , mut new)= (new, current);
            //Array2D::from_rows(&new.as_rows());
        if iter > 100 || change == false {
            break
        }
    }
    // println!("iter {:#?}",current);
    // println!("iter {}",iter);
    // 5032 too hight
    current.elements_row_major_iter().filter(|&&p|p==Point::Occupied).count()
}
