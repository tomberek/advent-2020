#![allow(dead_code)]

use aoc_runner_derive::{aoc, aoc_generator};

use scan_fmt::scan_fmt;

struct Policy {
    range_start: usize,
    range_end: usize,
    character: char,
    password: String,
}

#[aoc_generator(day02)]
fn input_generator(inp: &str) -> Vec<Policy> {
    let nums = inp.lines().map(|line| parse(line)).collect();
    return nums;
}

//fn parse(line: &str) -> Option<(usize, usize, char, String)> {
fn parse(line: &str) -> Policy {
    let (min, max, c, pass) = scan_fmt!(&line, "{d}-{d} {}: {}", _, _, _, _).unwrap();
    return Policy {
        range_start: min,
        range_end: max,
        character: c,
        password: pass,
    };
}

#[aoc(day02, part1)]
fn part1(ps: &Vec<Policy>) -> usize {
    return ps
        .iter()
        .map(
            |Policy {
                 range_start,
                 range_end,
                 character,
                 password,
             }| {
                let b = password.chars().filter(|c| c == character).count();
                return *range_start <= b && b <= *range_end;
            },
        )
        .filter(|p| *p)
        .count();
}

#[aoc(day02, part2)]
fn part2(ps: &Vec<Policy>) -> usize {
    return ps
        .iter()
        .map(
            |Policy {
                 range_start,
                 range_end,
                 character,
                 password,
             }| {
                let fst = password.as_bytes()[range_start - 1] as char;
                let snd = password.as_bytes()[range_end - 1] as char;
                return (fst == *character) ^ (snd == *character);
            },
        )
        .filter(|p| *p)
        .count();
}
