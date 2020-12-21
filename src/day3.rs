extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day3.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap()
    } ).collect())
}

fn find_slope(a: &Vec<String>, right_inc: usize, down_inc: usize) -> i64 {
    let mut right = 0;
    let mut down = 0;
    let mut trees = 0;
    let n = a.len();
    let str_n = a[0].len();
    while down < n {
        let x = a[down].chars().nth(right).unwrap();
        if x == '#' {
            trees += 1
        }
        down += down_inc;
        right = (right + right_inc)%str_n;
    }
    println!("{}", trees);
    trees
}

fn solve(a: Vec<String>) {
    let b:i64 = find_slope(&a, 1, 1) * find_slope(&a, 3, 1) * find_slope(&a, 5, 1) * find_slope(&a, 7, 1) * find_slope(&a, 1, 2);
    println!("{}", b);
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => solve(values),
        _ => println!("error occurred parsing input")
    };
}