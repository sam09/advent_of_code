use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;


fn read_input()-> io::Result<Vec<i32>> {
    let filename = "./data/day1.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    
    
    Ok(lines.map( |a| {
        a.unwrap().parse::<i32>().unwrap()
    } ).collect())
}


fn solve(a: Vec<i32>) {
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

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => solve(values),
        _ => println!("error occurred parsing input")
    };
}