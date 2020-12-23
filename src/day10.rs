extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn read_input()-> io::Result<Vec<i32>> {
    let filename = "./data/day10.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().parse::<i32>().unwrap()
    } ).collect())
}


fn f(v: i32, a: &HashSet<i32>, dp: &mut HashMap<i32, i64>) -> i64 {
    if v == 0 {
        return 1
    }
    if !a.contains(&v) {
        return 0;
    }

    if dp.contains_key(&v) {
        return dp[&v];
    }

    let ans = f(v-1, a, dp) + f(v-2, a, dp) + f(v-3, a, dp);
    dp.insert(v, ans);
    ans
}

fn solve(mut a: Vec<i32>) -> i64 {
    a.sort();
    let max = a[a.len()-1];
    a.push( max + 3);
    let set = HashSet::<i32>::from_iter(a.iter().cloned());
    let mut map = HashMap::<i32, i64>::new();
    f(max + 3, &set, &mut map)
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}