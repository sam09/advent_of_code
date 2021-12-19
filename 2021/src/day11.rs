use crate::utils::read_input_2d_vec;
use std::collections::{HashSet};

fn solve_pt1(a: &mut Vec<Vec<i32>>) {
    let n = a.len();
    let m = a[0].len();
    let mut steps = 100;
    let mut flashes = 0;
    while steps > 0 {
        let mut first_iter = true;
        let mut visited = HashSet::new();
        loop {
            let mut did_flash = false;
            for i in 0..n {
                for j in 0..m {
                    if first_iter {
                        a[i][j] += 1;
                    }
                    if a[i][j] >= 10 && !visited.contains(&(i, j)){
                        visited.insert((i, j));
                        flashes += 1;
                        did_flash = true;
                        if i > 0 {
                            a[i-1][j] += 1;
                        }
                        if i < n-1 {
                            a[i+1][j] += 1;
                        }
                        if j > 0 {
                            a[i][j-1] += 1;
                        }
                        if j < m-1 {
                            a[i][j+1] += 1;
                        }
                        if i > 0 && j > 0 {
                            a[i-1][j-1] += 1;
                        }
                        if i > 0 && j < m-1 {
                            a[i-1][j+1] += 1;
                        }
                        if i < n-1 && j > 0 {
                            a[i+1][j-1] += 1;
                        }
                        if i < n -1  && j < m-1 {
                            a[i+1][j+1] += 1;
                        }
                    }
                }
            }
            first_iter = false;
            if did_flash == false {
                break;
            }
            for i in 0..n {
                for j in 0..m {
                    if visited.contains(&(i, j)) {
                        a[i][j] = 0;
                    }
                }
            }
        }
        steps-=1;
    }
    println!("{}", flashes);
}


fn solve_pt2(a: &mut Vec<Vec<i32>>) {
    let n = a.len();
    let m = a[0].len();
    let mut all_flashed = false;
    let mut steps = 0;
    while all_flashed == false {
        let mut first_iter = true;
        let mut visited = HashSet::new();
        let mut flashes = 0;
        loop {
            let mut did_flash = false;
            for i in 0..n {
                for j in 0..m {
                    if first_iter {
                        a[i][j] += 1;
                    }
                    if a[i][j] >= 10 && !visited.contains(&(i, j)){
                        visited.insert((i, j));
                        flashes += 1;
                        did_flash = true;
                        if i > 0 {
                            a[i-1][j] += 1;
                        }
                        if i < n-1 {
                            a[i+1][j] += 1;
                        }
                        if j > 0 {
                            a[i][j-1] += 1;
                        }
                        if j < m-1 {
                            a[i][j+1] += 1;
                        }
                        if i > 0 && j > 0 {
                            a[i-1][j-1] += 1;
                        }
                        if i > 0 && j < m-1 {
                            a[i-1][j+1] += 1;
                        }
                        if i < n-1 && j > 0 {
                            a[i+1][j-1] += 1;
                        }
                        if i < n -1  && j < m-1 {
                            a[i+1][j+1] += 1;
                        }
                    }
                }
            }
            first_iter = false;
            if did_flash == false {
                break;
            }
            for i in 0..n {
                for j in 0..m {
                    if visited.contains(&(i, j)) {
                        a[i][j] = 0;
                    }
                }
            }
        }
        steps += 1;
        if flashes == n*m {
            all_flashed = true;
        }
    }
    println!("{}", steps);
}


pub fn run(part: char) {
    let v = read_input_2d_vec("data/day11.txt");
    match v {
        Ok(mut values) => {
            match part {
                'a' => solve_pt1(&mut values),
                'b' => solve_pt2(&mut values),
                _ => println!("error occurred parsing options")
            }
        },
        _ => println!("error occurred parsing input")
    };
}