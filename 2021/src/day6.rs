use crate::utils::{read_input_string, convert_string_to_int_vec};
use std::collections::HashMap;

fn fishes(a: Vec<i32>, days: i32) -> i64 {
    let mut fish_count = HashMap::new();
    for i in 0..9 {
        for j in 0..i+1 {
            fish_count.insert((i, j), 1);
        }
    }

    for j in 1..days + 1 {
        for i in 0..9 {
            let new_days = j - i - 1;
            if new_days >= 0 {
                let tp = fish_count[&(6, new_days)] + fish_count[&(8, new_days)];
                fish_count.insert((i, j), tp);
            }
        }
    }
    
   let mut sum = 0 as i64;
    for i in 0..a.len() {
        sum += fish_count[&(a[i], days)];
    }
    sum
}


fn solve(a: Vec<i32>, days: i32) {
    println!("{}", fishes(a, days));
}


pub fn run(part: char) {
    let v = read_input_string("data/day6.txt");
    match v {
        Ok(values) => {
            let a = convert_string_to_int_vec(&values[0], ',');
            match part {
                'a' => solve(a, 80),
                'b' => solve(a, 256),
                _ => println!("error occurred parsing options")
            }
        },
        _ => println!("error occurred parsing input")
    };
}