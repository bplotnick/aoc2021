use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn part1() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut x = 0;
    let mut y = 0;
    buf.lines().for_each(|line| {
        let what = line.unwrap(); // how do i make the borrow checker happy
        let (dir, dist) = what.split_once(" ").unwrap();
        if dir.eq("forward") {
            x += dist.parse::<i32>().unwrap();
        } else if dir.eq("down") {
            y += dist.parse::<i32>().unwrap();
        } else if dir.eq("up") {
            y -= dist.parse::<i32>().unwrap();
        } else {
            println!("wut");
        }
    });
    println!("{}", x*y);
}

fn part2() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    buf.lines().for_each(|line| {
        let what = line.unwrap(); // how do i make the borrow checker happy
        let (dir, dist) = what.split_once(" ").unwrap();
        let idist = dist.parse::<i32>().unwrap();
        if dir.eq("forward") {
            x += idist;
            y += aim*idist;
        } else if dir.eq("down") {
            aim += idist;
        } else if dir.eq("up") {
            aim -= idist;
        } else {
            println!("wut");
        }
    });
    println!("{}", x*y);
}

fn main() {
    part1();

    part2();
}