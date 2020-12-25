extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;


fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day14.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

fn overwrite(value: &str, mask: &str) -> i64 {
    let n = mask.len();
    let mut v = 0;
    let value_char = binary(value);
    let mask_char: Vec<char> = mask.chars().collect();
    for i in 0..n {
        let mut tp = value_char[i];
        if mask_char[i] != 'X' {
            tp = mask_char[i];
        }
        v = v*2 + (tp as i64 - 48);
    }
    v
}


fn binary(value: &str) -> Vec<char> {
    let mut value_char = Vec::new();
    let mut val = value.parse::<i32>().unwrap();

    while val > 0 {
        let rem = val % 2;
        match rem {
            0 => value_char.push('0'),
            1 => value_char.push('1'),
            _ => ()
        };
        val = val/2;
    }
    while value_char.len() < 36 {
        value_char.push('0');
    }
    value_char.reverse();
    value_char
}

fn solve_pt1(a: Vec<String>) -> i64 {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut map = HashMap::new();
    for i in &a {
        let temp = i.split(" = ").collect::<Vec<&str>>();
        if temp[0] == "mask" {
            mask = temp[1];
        } else {
            let add = &temp[0][4..temp[0].len()-1];
            let value = temp[1];
            let final_val = overwrite(value, &mask);
            map.insert(add, final_val);
        }
    }
    let mut sum = 0;
    for (_, j) in &map {
        sum += j;
    }
    sum
}


fn all_addresses(address: &str, mask: &str) -> Vec<i64> {
    let address_char = binary(address);
    let n = address_char.len();
    let mut final_address = Vec::new();
    let mask_char: Vec<char> = mask.chars().collect();
    final_address.push(0);
    for i in 0..n {
        let mut values = Vec::new();
        if mask_char[i] == '0' {
            values.push(address_char[i] as i64 - 48);
        } else if mask_char[i] == '1' {
            values.push(1);
        } else {
            values.push(0);
            values.push(1);
        }

        let mut temp = Vec::new();
        for f in &final_address {
            for v in &values {
                temp.push(f*2 + v);
            }
        }
        final_address.clear();
        final_address = temp.clone();
    }
    final_address
}

fn solve(a: Vec<String>) -> i64 {
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let mut map = HashMap::new();
    for i in &a {
        let temp = i.split(" = ").collect::<Vec<&str>>();
        if temp[0] == "mask" {
            mask = temp[1];
        } else {
            let value = temp[1];
            let add = &temp[0][4..temp[0].len()-1];
            let address_set = all_addresses(add, mask);
            for address in address_set {
                map.insert(address, value.parse::<i64>().unwrap());
            }
        }
    }
    let mut sum = 0;
    for (_i, j) in &map {
        sum += j;
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
