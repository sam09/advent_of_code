extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day5.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap()
    } ).collect())
}


fn bin_search(n: i32, row_str: &str, f: char, b: char) -> i32 {

    let mut min = 0;
    let mut max = n;
    let mut mid;

    for i in row_str.chars() {
        mid = (min+max)/2;
        //println!("{} {} {}", min, max, mid);
        if i == f {
            max = mid;
        } else if i == b {
            min = mid+1;
        }
    }
    (max + min)/2
}

fn find_seat_id(a: String) -> i32 {
    let row_str = &a[..7];
    let col_str = &a[7..];

    let row = bin_search(127, row_str, 'F', 'B');
    let col = bin_search(7, col_str, 'L', 'R');
    //println!("{} {} {}", row, col, row*8+col);
    row * 8 + col
}

fn solve_pt1(a: Vec<String>) {
    let mut max_seat_id = 0;
    for i in a {
        let seat_id = find_seat_id(i);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    println!("{}", max_seat_id);
}

fn solve(a: Vec<String>) {
    let mut v: Vec<i32>  = Vec::new();
    for i in a {
        v.push(find_seat_id(i));
    }
    v.sort();
    let min_v = v[0];
    let max_v = v[v.len() - 1];
    println!("{} {} {}", min_v, max_v, v.len());

    for i in 1..v.len() {
        if (v[i] - 1) != v[i-1]  {
            println!("{}", v[i] - 1);
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