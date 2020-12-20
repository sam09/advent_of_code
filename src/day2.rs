extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::convert::TryInto;

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day2.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    
    
    Ok(lines.map( |a| {
        a.unwrap()
    } ).collect())
}


fn check_count(min_num: i32, max_num: i32, ch: char, password: String) -> bool {
    let c = password.chars().filter(|&a| a==ch).count();
    c >= min_num.try_into().unwrap() && c <=  max_num.try_into().unwrap()
}

//Part 2
fn check_pos(min_pos: i32, max_pos: i32, ch: char, password: String) -> bool {
    let min_usize: usize = min_pos.try_into().unwrap();
    let max_usize: usize = max_pos.try_into().unwrap();
    if password.len() < min_usize {
        return false;
    } else if password.len() < max_usize {
        return true;
    }
    let char_at_first = password.chars().nth(min_usize - 1).unwrap();
    let char_at_second = password.chars().nth(max_usize - 1).unwrap();
    return ( (char_at_first == ch) && (char_at_second != ch) ) || ( (char_at_first != ch) && (char_at_second == ch) )
    
}

fn check(a: String) -> bool {
    let re = Regex::new(r"(\d{1,2})-(\d{1,2}) ([a-zA-Z]): ([a-zA-Z]+)").unwrap();
    for cap in re.captures_iter(&a) {
        let min_num = cap[1].parse::<i32>().unwrap();
        let max_num = cap[2].parse::<i32>().unwrap();
        return check_pos(min_num, max_num, cap[3].chars().next().expect("Not a char"), cap[4].to_string());
    }
    false
}

fn solve(a: Vec<String>) {
    let mut counter = 0;
    for i in a {
        if check(i) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => solve(values),
        _ => println!("error occurred parsing input")
    };
}