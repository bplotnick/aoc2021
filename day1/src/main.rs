use std::env;
use itertools::Itertools;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part1(filename: &String) -> usize{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: Vec<_> = buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(prev,next)| next > prev)
        .collect();
    return out.len();
}

fn part2(filename: &String) -> usize {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let out: Vec<_> = buf.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .tuple_windows::<(_,_,_)>()
        .map(|(first,second,third)| first+second+third)
        .tuple_windows()
        .filter(|(prev,next)| next > prev)
        .collect();
    return out.len();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Part 1: {}", part1(filename));

    println!("Part 2: {}", part2(filename));

}
