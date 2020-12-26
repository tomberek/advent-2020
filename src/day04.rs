#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use aoc_runner_derive::{aoc, aoc_generator};

use scan_fmt::scan_fmt;
//use std::num::ParseIntError;
use std::error;

#[derive(PartialEq, Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: usize,
    hcl: String,
    ecl: String,
    pid: usize,
    cid: usize,
}
impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: 0,
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: 0,
            cid: 0,
        }
    }
}

#[aoc_generator(day04)]
fn input_generator(inp: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut items: Vec<String> = vec![];
    for l in inp.lines() {
        if l == "" {
            //items.sort();
            res.push(items.join(" "));
            items.clear();
            continue;
        }
        items.append(&mut l.split(" ").map(String::from).collect::<Vec<String>>());
    }
    if items.len() > 0 {
        res.push(items.join(" "));
    }
    return res;
}

fn parse2(line: &String) -> Option<Passport> {
    let mut a = Passport::default();
    //println!("{:?}",line);
    let lm: Result<usize, Box<dyn error::Error>> = line
        .split(" ")
        .filter(|l| *l != "")
        .map(|item| {
            let items: Vec<&str> = item.split(":").collect();

            let (key, value) = (items[0], items[1]);
            match key {
                "byr" => a.byr = 1,
                "iyr" => a.iyr = 1,
                "eyr" => a.eyr = 1,
                "hgt" => a.hgt = 1,
                "hcl" => a.hcl = "a".to_string(),
                "ecl" => a.ecl = "b".to_string(),
                "pid" => a.pid = 1,
                // "byr" => a.byr = value.parse()?,
                // "iyr" => a.iyr = value.parse()?,
                // "eyr" => a.eyr = value.parse()?,
                // "hgt" => a.hgt = {
                //     value.chars().take_while(|x|x.is_digit(10)).collect::<String>().parse()?
                // },
                // "hcl" => a.hcl = String::from(value),
                // "ecl" => a.ecl = String::from(value),
                // "pid" => a.pid = value.parse()?,
                //"cid" => a.cid = value.parse()?,
                _ => (),
            };
            return Ok(1);
        })
        .sum();
    match lm {
        Err(_) => return None,
        Ok(l) => {
            //return Some(a);
            if a.byr == 0
                || a.iyr == 0
                || a.eyr == 0
                || a.hgt == 0
                || a.hcl == ""
                || a.ecl == ""
                || a.pid == 0
            // || a.cid == 0
            {
                return None;
            }
            return Some(a);
        }
    }
}
fn parse(line: &String) -> Option<Passport> {
    let mut a = Passport::default();
    let lm: Result<usize, Box<dyn error::Error>> = line
        .split(" ")
        .filter(|l| *l != "")
        .map(|item| {
            let mut items = item.split(":");
            let key = items.next().unwrap();
            let value = items.next().unwrap();

            //let (key, value) = (items[0], items[1]);
            match key {
                "byr" => {
                    let n = value.parse().unwrap();
                    if n >= 1920 && n <= 2002 {
                        a.byr = n;
                    }
                }
                "iyr" => {
                    let n = value.parse().unwrap();
                    if n >= 2010 && n <= 2020 {
                        a.iyr = n;
                    }
                }
                "eyr" => {
                    let n = value.parse().unwrap();
                    if n >= 2020 && n <= 2030 {
                        a.eyr = n;
                    }
                }
                "hgt" => {
                    let (n, r) = scan_fmt!(value, "{/^[0-9]*/}{/[a-z]{2}$/}", usize, String)?;
                    if r == "cm" && (n >= 150 && n <= 193) {
                        a.hgt = n;
                    }
                    if r == "in" && (n >= 59 && n <= 76) {
                        a.hgt = n;
                    }
                }
                "hcl" => {
                    //let n = scan_fmt!(value, "#{/^[a-f0-9]{6}$/}", String)?;
                    if value.len() == 7 {
                        a.hcl = String::from(value);
                    }
                }
                "ecl" => {
                    if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                        a.ecl = String::from(value);
                    }
                }
                "pid" => {
                    let n = value;
                    if value.len() == 9 {
                        a.pid = n.parse()?;
                    }
                }
                //"cid" => a.cid = value.parse()?,
                _ => (),
            };
            return Ok(1);
        })
        .sum();
    match lm {
        Err(_) => return None,
        Ok(l) => {
            //return Some(a);
            if a.byr == 0
                || a.iyr == 0
                || a.eyr == 0
                || a.hgt == 0
                || a.hcl == ""
                || a.ecl == ""
                || a.pid == 0
            // || a.cid == 0
            {
                return None;
            }
            return Some(a);
        }
    }
}

#[aoc(day04, part1)]
fn part1(ps: &Vec<String>) -> usize {
    return ps.iter().map(parse2).filter(|s| s.is_some()).count();
}

#[aoc(day04, part2)]
fn part2(ps: &Vec<String>) -> usize {
    //println!("lines: {}", ps.len());
    return ps.iter().map(parse).filter(|s| s.is_some()).count();
}
