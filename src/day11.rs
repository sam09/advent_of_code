extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::convert::TryInto;

fn read_input()-> io::Result<Vec<Vec<char>>> {
    let filename = "./data/day11.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

fn find_cordinate(a: &Vec<Vec<char>>, n:i32, m:i32, row:i32, col:i32, dx:i32, dy:i32) -> (i32, i32) {
    let mut i = row+dx;
    let mut j = col+dy;
    while i >= 0 && i < n && j >= 0 && j < m {
        if a[i as usize][j as usize] != '.' {
            break;
        }
        i += dx;
        j += dy;
    }
    (i, j)
}

fn get_valid_cords(row: i32, col: i32, n:i32, m:i32, a: &Vec<Vec<char>> ) -> Vec<(i32, i32)> {
    let mut set = Vec::new();
    for i in vec![-1, 0, 1] {
        for j in vec![-1, 0, 1] {
            if i != 0 || j !=0 {
                let (x,y) = find_cordinate(a, n, m, row, col, i, j);
                if x >= 0 && x < n && y >= 0 && y < m {
                    set.push((x, y));
                }
            }
        }
    }
    return set;
}

fn should_switch(val: char, row: i32, col: i32, n:i32, m:i32, a: &Vec<Vec<char>> ) -> (bool, char) {
    let mut change = true;
    let mut opp = '#';
    let set = get_valid_cords(row, col, n, m, a);
    if val == '.' {
        return (false, '.');
    }
    if val == 'L' {
        for (x, y) in set {
            let i = x as usize;
            let j = y as usize;
            if a[i][j] == '#' {
                change = false;
                opp = 'L';
            }
        }
        return (change, opp)
    } else if val == '#' {
        change = false;
        let mut ct = 0;
        for (x, y) in set {
            let i = x as usize;
            let j = y as usize;
            if a[i][j] == '#' {
                ct+=1;
            }
        }
        if ct >= 5 {
            change = true;
            opp = 'L';
        }
    }
    (change, opp)
}

fn solve(mut a: Vec<Vec<char>>) -> i64 {
    let mut changed = true;
    let mut b = a.clone();
    let n = a.len();
    let m = a[0].len();
    let mut count = 0;

    while changed == true {
        a = b.clone();
        changed = false;
        count = 0;
        for i in 0..n {
            for j in 0..m {
                let k = a[i][j];
                let (flag, val) = should_switch(k, i.try_into().unwrap(), j.try_into().unwrap(), n.try_into().unwrap(), m.try_into().unwrap(), &a);
                if flag == true {
                    b[i][j] = val;
                    changed = true;
                }
                if b[i][j] == '#' {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}

/*
[
    ['#', '.', '#', '#', '.', '#', '#', '.', '#', '#'],
    ['#', '#', '#', '#', '#', '#', '#', '.', '#', '#'],
    ['#', '.', '#', '.', '#', '.', '.', '#', '.', '.'],
    ['#', '#', '#', '#', '.', '#', '#', '.', '#', '#'],
    ['#', '.', '#', '#', '.', '#', '#', '.', '#', '#'],
    ['#', '.', '#', '#', '#', '#', '#', '.', '#', '#'],
    ['.', '.', '#', '.', '#', '.', '.', '.', '.', '.'],
    ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ['#', '.', '#', '#', '#', '#', '#', '#', '.', '#'],
    ['#', '.', '#', '#', '#', '#', '#', '.', '#', '#']]
*/