extern crate regex;
use std::convert::TryInto;
use crate::utils::read_input_char_vec;

fn find_coordinate_pt2(a: &Vec<Vec<char>>, n:i32, m:i32, row:i32, col:i32, dx:i32, dy:i32) -> (i32, i32) {
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

fn find_coordinate_pt1(row:i32, col:i32, dx:i32, dy:i32) -> (i32, i32) {
    (row + dx, col + dy)
}

fn find_coordinate(a: &Vec<Vec<char>>, n:i32, m:i32, row:i32, col:i32, dx:i32, dy:i32, part: char) -> (i32, i32) {
    return if part == 'a' {
        find_coordinate_pt1(row, col, dx, dy)
    } else {
        find_coordinate_pt2(a, n, m, row, col, dx, dy)
    }
}


fn get_valid_cords(row: i32, col: i32, n:i32, m:i32, a: &Vec<Vec<char>>, part: char ) -> Vec<(i32, i32)> {
    let mut set = Vec::new();
    for i in vec![-1, 0, 1] {
        for j in vec![-1, 0, 1] {
            if i != 0 || j !=0 {
                let (x,y) = find_coordinate(a, n, m, row, col, i, j, part);
                if x >= 0 && x < n && y >= 0 && y < m {
                    set.push((x, y));
                }
            }
        }
    }
    return set;
}

fn should_switch(val: char, row: i32, col: i32, n:i32, m:i32, a: &Vec<Vec<char>>, part: char ) -> (bool, char) {
    let mut change = true;
    let mut opp = '#';
    let set = get_valid_cords(row, col, n, m, a, part);
    let should_switch_threshold = if part == 'a' { 4 } else { 5 };
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
        if ct >= should_switch_threshold {
            change = true;
            opp = 'L';
        }
    }
    (change, opp)
}

fn solve(mut a: Vec<Vec<char>>, part: char) -> i64 {
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
                let (flag, val) = should_switch(k, i.try_into().unwrap(), j.try_into().unwrap(), n.try_into().unwrap(), m.try_into().unwrap(), &a, part);
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

pub fn run(part: char) {
    let v = read_input_char_vec("data/day11.txt");
    match v {
        Ok(values) => {
            let ans = solve(values, part);
            println!("{}", ans);
        },
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