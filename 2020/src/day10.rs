use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::utils::read_input_int;

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
fn solve_pt1(mut a: Vec<i32>) -> i64 {
    a.sort();
    let mut ones = 1;
    let mut threes = 1;
    for i in 0..a.len()-1 {
        match a[i+1] - a[i] {
            1 => ones += 1,
            3 => threes += 1,
            _ => ()
        }
    }
    ones * threes
}

fn solve_pt2(mut a: Vec<i32>) -> i64 {
    a.sort();
    let max = a[a.len()-1];
    a.push( max + 3);
    let set = HashSet::<i32>::from_iter(a.iter().cloned());
    let mut map = HashMap::<i32, i64>::new();
    f(max + 3, &set, &mut map)
}

pub fn run(part: char) {
    let v = read_input_int("data/day10.txt");
    match v {
        Ok(values) =>{
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans);
        }
        _ => println!("error occurred parsing input")
    };
}