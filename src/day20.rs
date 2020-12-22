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

#[aoc_generator(day20)]
fn input_generator(inp: &str) -> Problem {
    let mut tiles = HashMap::new();
    inp.split("\n\n").for_each(|group|{
        let mut lines = group.lines();
        let title = lines.next().unwrap();
        let img = lines
            .flat_map(|line| return line.chars().map(|c|{
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("bad input"),
                }
            }))
            .collect::<Vec<bool>>();
        if let Ok(num) = scan_fmt!(title, "Tile {d}:", usize) {
            tiles.insert(num,Array::from_shape_vec((10,10),img).unwrap());
        }
    });
    return Problem{tiles}
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    tiles: HashMap<usize,Array2<bool>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Orient {
    Id,
    Rot90,
    Rot180,
    Rot270,
}
fn is_aligned(a: &ArrayView2<bool>,b: &ArrayView2<bool>) -> Option<(isize,isize, Orient)>{
    if a.slice(s![9, ..]) == b.slice(s![0, ..]) { return Some((1,0,Orient::Id))}
    if a.slice(s![0, ..]) == b.slice(s![9, ..]) { return Some((-1,0,Orient::Id))}

    if a.slice(s![.., 9]) == b.slice(s![.. ,0]) { return Some((0,1,Orient::Id))}
    if a.slice(s![.., 0]) == b.slice(s![.. ,9]) { return Some((0,-1,Orient::Id))}

    // Rot90
    if a.slice(s![9, ..]) == b.slice(s![..;-1, 0]) { return Some((1,0,Orient::Rot90))}
    if a.slice(s![0, ..]) == b.slice(s![..;-1, 9]) { return Some((-1,0,Orient::Rot90))}

    if a.slice(s![.., 9]) == b.slice(s![9, ..]) { return Some((0,1,Orient::Rot90))}
    if a.slice(s![.., 0]) == b.slice(s![0 ,..]) { return Some((0,-1,Orient::Rot90))}

    // Rot180
    if a.slice(s![9, ..]) == b.slice(s![9,..;-1]) { return Some((1,0,Orient::Rot180))}
    if a.slice(s![0, ..]) == b.slice(s![0,..;-1]) { return Some((-1,0,Orient::Rot180))}

    if a.slice(s![.., 9]) == b.slice(s![..;-1,9]) { return Some((0,1,Orient::Rot180))}
    if a.slice(s![.., 0]) == b.slice(s![..;-1,0]) { return Some((0,-1,Orient::Rot180))}

    // Rot270
    if a.slice(s![9, ..]) == b.slice(s![..,9]) { return Some((1,0,Orient::Rot270))}
    if a.slice(s![0, ..]) == b.slice(s![..,0]) { return Some((-1,0,Orient::Rot270))}

    if a.slice(s![.., 9]) == b.slice(s![0,..;-1]) { return Some((0,1,Orient::Rot270))}
    if a.slice(s![.., 0]) == b.slice(s![9,..;-1]) { return Some((0,-1,Orient::Rot270))}

    return None
}

impl Problem {
    fn check(&self,a:usize,b:usize) -> Option<(isize,isize,Orient,bool)>{
        return is_aligned2(
            &self.tiles.get(&a).unwrap().view(),
            &self.tiles.get(&b).unwrap().view(),
        )
    }
}
fn is_aligned2(a: &ArrayView2<bool>,b: &ArrayView2<bool>) -> Option<(isize,isize, Orient,bool)>{
    return is_aligned(a,b)
        .map(|(x,y,z)|(x,y,z,false))
        .or(
            is_aligned(a,&b.t().view())
            .map(|(x,y,z)|(x,y,z,true))
        )
}

#[aoc(day20, part1)]
fn part1(inp: &Problem) -> usize {
    inp.tiles.iter().map(|(&num,_)|{
        let matching = inp.tiles.iter().filter_map(|(&a,_)|inp.check(num,a)).count();
        if matching == 3 {
            return num
        }
        return 1

    }).product()
}

struct Tile {
    tile:Array2::<bool>,
}

fn min_max(inp: &Array2<bool>) -> (usize,usize,usize,usize){
    let mut minx = inp.dim().1;
    let mut miny = inp.dim().0;
    let mut maxx = 0;
    let mut maxy = 0;
    inp.indexed_iter().filter(|(_,&v)|v).for_each(|((y,x),_)|{
        if y<miny { miny=y;}
        if x<minx { minx=x;}
        if y>maxy { maxy=y;}
        if x>maxx { maxx=x;}
    });
    return (miny,minx,maxy,maxx)
}

#[aoc(day20, part2)]
fn part2(inp: &Problem) -> usize {
    let l = (inp.tiles.len() as f64).sqrt().round() as usize;
    let mut img = Array::<bool,_>::from_elem((l*WIDTH*2,l*WIDTH*2),false);
    let (&current,_) = inp.tiles.iter().next().unwrap();

    //println!("{}",Tile{tile:inp.tiles[&current].clone()});
    let x = l * WIDTH ;
    let y = l * WIDTH ;
    //let widths = s![0..10,0..10];
    let widths = s![1..9,1..9];
    let mut queue = HashSet::new();
    img.slice_mut(s![y+0..y+WIDTH,x+0..x+WIDTH])
        .assign(&inp.tiles[&current].slice(widths).clone() );
    queue.insert(current);
    fill(&mut inp.clone(),current,y,x,&mut queue,&mut img);

    let (miny,minx,maxy,maxx) = min_max(&img);
    //println!("{} {} {} {}",miny,minx,maxy,maxx);
    let output =img.slice(s![miny..maxy+1,minx..maxx+1]);
    //println!("{}",Tile{tile:output.to_owned()});

    let mut monster = Array::from_elem((3,20),false);
    monster[[1,0]]=true;
    monster[[2,1]]=true;
    monster[[2,4]]=true;
    monster[[1,5]]=true;
    monster[[1,6]]=true;
    monster[[2,7]]=true;
    monster[[2,10]]=true;
    monster[[1,11]]=true;
    monster[[1,12]]=true;
    monster[[2,13]]=true;
    monster[[2,16]]=true;
    monster[[1,17]]=true;
    monster[[0,18]]=true;
    monster[[1,18]]=true;
    monster[[1,19]]=true;
    //println!("{}",Tile{tile:monster.to_owned()});
    let monsters : usize = [
        &monster.view(),
        &monster.t(),
        &monster.slice(s![..;-1,..]),
        &monster.slice(s![..,..;-1]),
        &monster.t().slice(s![..;-1,..]),
        &monster.t().slice(s![..,..;-1]),
        &monster.slice(s![..;-1,..;-1]),
        &monster.t().slice(s![..;-1,..;-1]),
    ].par_iter().map(|a|{
        return search(&output.to_owned(),a)
    }).sum();

    return output.iter().filter(|&&a|a).count()
         - monsters * monster.iter().filter(|&&a|a).count()
}
fn search(output: &Array2<bool>,monster: &ArrayView2<bool>) -> usize{
    output.indexed_iter().map(|((y,x),v)|{
        if y+monster.dim().0 > output.dim().0 || x+monster.dim().1 > output.dim().1 {
            return 0
        }
        //println!("{} {}",y,output.dim().0);
        let view = output.slice(s![y..y+monster.dim().0,x..x+monster.dim().1]).to_owned();

        if &view & monster == *monster {
            //println!("found one: {} {}",y,x);
            return 1
        }
        return 0
    }).sum()
}

const WIDTH : usize = 8;
fn fill(inp:&mut Problem,current: usize, y:usize,x:usize,queue: &mut HashSet<usize>,img : &mut Array2<bool>){
    let widths = s![1..9,1..9];
    //let widths = s![0..10,0..10];

    let list :Vec<usize>= inp.tiles.iter().map(|(&a,_)|a).collect();
    for a in list {
        if current == a { continue }
        let (s,tile) = match inp.check(current,a).map(|x|(a,x)) {
            None => { continue },
            Some(tile) => tile,
        };
        if queue.contains(&s) { continue }
        queue.insert(s);

        let mut output = inp.tiles[&s].to_owned();
        if tile.3 {
            output = output.t().to_owned();
        }
        let dy = (y as isize + (WIDTH as isize)*tile.0) as usize;
        let dx = (x as isize + (WIDTH as isize)*tile.1) as usize;
        match tile {
            (_,_,Orient::Id,_) => {
                inp.tiles.insert(s,output.to_owned());
            },
            (_,_,Orient::Rot90,_) => {
                inp.tiles.insert(s,output.t().slice(s![..,..;-1]).to_owned());
                },
            (_,_,Orient::Rot180,_) => {
                inp.tiles.insert(s,output.slice(s![..;-1,..;-1]).to_owned());
                },
            (_,_,Orient::Rot270,_) => {
                inp.tiles.insert(s,output.t().slice(s![..;-1,..]).to_owned());
                },
        };
        img.slice_mut(s![dy+0..dy+WIDTH,dx+0..dx+WIDTH])
            .assign(&inp.tiles[&s].slice(widths).clone() );
        //println!("{}",Tile{tile:output.clone()});
        //println!("{}",Tile{tile:img.clone()});
        fill(inp,s,dy,dx,queue,img);
    }
}


impl fmt::Display for Tile {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {

        let (miny,minx,maxy,maxx) = min_max(&self.tile);
        println!("{} {} {} {}",miny,minx,maxy,maxx);
        self.tile.slice(s![miny..maxy+1,minx..maxx+1]).outer_iter().enumerate().map(|(i,x)|{
            x.iter().map(|y|{
                if *y {
                write!(f,"#")
                } else {
                write!(f,".")
                }
            }).collect::<Result<_,_>>()?;
            if i%WIDTH==0 {write!(f,"-")?}

            write!(f,"\n")
        }).collect::<Result<_,_>>()?;
        write!(f,"\n")
    }
}

