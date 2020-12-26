#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use aoc_runner_derive::{aoc, aoc_generator};

use packed_simd::{Simd,i32x8,i16x8};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp;


use std::{cmp::Ordering};

#[aoc_generator(day01)]
fn input_generator(inp: &str) -> Vec<i16> {
    let nums = inp
        .split('\n')
        .map(|num| num.parse().unwrap())
        .collect();
    return nums
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[aoc(day01,part1)]
fn part1(nums: &[i16]) -> i32 {
    let c2020 : Simd<[i16;8]> = i16x8::from_slice_unaligned(&[2020;8]);

    // Parse input
    let nums_set: HashSet<i16> = nums.iter().cloned().collect();
    //let (nums, nums_set) = parse_input(inp);

    return nums.chunks_exact(i16x8::lanes())
        .map(i16x8::from_slice_unaligned)
        .filter_map(|num| {
            let a : i16x8 = 2020-num;
            for i in 0..8 {
                if nums_set.contains(&(a.extract(i))) {
                    return Some((num.extract(i) as i32) * (a.extract(i) as i32))
                };
            }
            return None
        }).next().unwrap();

}

const INPUT: &str = include_str!("../input/2020/day1.txt");

const TARGET: i32 = 2020;
// #[aoc(day01,part2,try)]
// pub fn part2_try(inputs: &[i16]) -> i32 {

//     let mut inputs = inputs.to_owned();
//     inputs.sort_unstable();
//     let len = inputs.len();

//     for (i, a) in inputs[0..(len - 2)].iter().enumerate() {
//         let mut left = i + 1;
//         let mut right = len - 1;
//         let remainder = 2020 - a;
//         while left < right {

//             let sum = inputs[left]+inputs[right];
//             match sum.cmp(&remainder) {
//                 Ordering::Less => {
//                     left += 1
//                 },
//                 Ordering::Greater => {
//                     right -= 1
//                 },
//                 Ordering::Equal => {
//                     let b = inputs[left];
//                     let c = inputs[right];
//                     return (*a as i32) * (b as i32) * (c as i32)
//                 }
//             }
//         }
//     }

//     unreachable!()
// }

#[aoc(day1,part2)]
fn part2(nums: &[i16]) -> i32 {
    let n = nums;
    let nums_set: HashSet<i16> = n.iter().cloned().collect();

    return n
        .iter()
        .tuple_combinations()
        .filter_map(|(&numa,&numb)| {
            let sum = numa + numb;
            if sum > 2020 {
                return None
            }
            if nums_set.contains(&(2020 - sum)) {
                return Some((numa as i32) * (numb as i32) * ((2020 - sum) as i32))
            };
            return None
        })
        .find(|_|true).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 1721 * 299)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 979 * 366 * 675)
    }
}
