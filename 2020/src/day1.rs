use std::collections::HashMap;
use crate::utils::read_input_int;

fn solve_pt1(a: Vec<i32>) {
    let final_val = 2020;
    let mut map = HashMap::new();

    let n = a.len();
    for i in 0..n {
        map.insert(&a[i], i);
    }

    for i in 0..n {
        if map.contains_key(&(final_val - &a[i])) {
            let b = map[&(final_val - &a[i])];
            if i == b {
                continue;
            }
            println!("{}", a[i] * a[b]);
            break;
        }
    }
}

fn solve_pt2(a: Vec<i32>) {
    let final_val = 2020;
    let mut map = HashMap::new();

    let n = a.len();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                map.insert(&a[i] + &a[j], (i, j));
            }
        }
    }

    for i in 0..n {
        if map.contains_key(&(final_val - &a[i])) {
            let b = map[&(final_val - &a[i])];
            if i == b.0 || i == b.1 {
                continue;
            }
            println!("{}", a[i] * a[b.0] * a[b.1]);
            break;
        }
    }
}


pub fn run(part: char) {
    let v = read_input_int("data/day1.txt");
    match v {
        Ok(values) => {
            match part {
                'a' => solve_pt1(values),
                'b' => solve_pt2(values),
                _ => println!("error occurred parsing options")
            }
        },
        _ => println!("error occurred parsing input")
    };
}