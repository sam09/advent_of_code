extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashSet;

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day6.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap()
    } ).collect())
}


fn solve_pt1(a: Vec<String>) -> usize {
    let mut set:HashSet<char> = HashSet::new();
    let mut sum = 0;
    for i in a {
        if i.is_empty() {
            sum += set.len();
            set.clear();
        } else {
            for j in i.chars() {
                set.insert(j);
            }
        }
    }
    sum + set.len()
}


fn solve(a: Vec<String>) -> usize {
    let mut v = vec![0; 26];
    let mut members = 0;
    let mut sum = 0;
    for i in a {
        if i.is_empty() {
            for j in 0..26 {
                if v[j] == members {
                    sum += 1;
                }
            }
            v = vec![0; 26];
            members = 0;
        } else {
            for j in i.chars() {
                let k = (j as usize) - 97;
                v[k] += 1;
            }
            members += 1;
        }
    }
    for j in 0..26 {
        if v[j] == members {
            sum += 1;
        }
    }

    sum
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}