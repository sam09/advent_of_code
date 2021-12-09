use crate::utils::read_input_char_vec;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

fn find_low_points(a: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, u32) {
    let mut low_points = Vec::new();
    let mut score = 0;
    let n = a.len();
    let m = a[0].len();

    for i in 0..n {
        for j in 0..m {
            let mut left = false;
            let mut right = false;
            let mut up = false;
            let mut down = false;
            if i == 0 || a[i-1][j] > a[i][j] {
                left = true;
            }
            if i == (n-1) || a[i+1][j] > a[i][j] {
                right = true;
            }
            if j == 0 || a[i][j-1] > a[i][j] {
                up = true;
            }
            if j == m-1 || a[i][j+1] > a[i][j] {
                down = true;
            }

            if up && down && left && right {
                score += a[i][j].to_digit(10).unwrap() + 1;
                low_points.push((i, j));
            }
        }
    }
    (low_points, score)
}


fn find_basin_size(a: &Vec<Vec<char>>, x: usize, y: usize, n: usize, m: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
    if x == n || y == m {
        return 0;
    }
    visited.insert((x, y));
    
    if a[x][y] == '9' {
        return 0;
    }

    let right = if !visited.contains(&(x+1, y)) { find_basin_size(a, x+1, y, n, m, visited) } else { 0 };

    let down = if !visited.contains(&(x, y+1)) { find_basin_size(a, x, y+1, n, m, visited) } else { 0 };
    
    let up = if y != 0 && !visited.contains(&(x, y-1)) { find_basin_size(a, x, y-1, n, m, visited) } else { 0 };

    let left = if x != 0 && !visited.contains(&(x-1, y)) { find_basin_size(a, x-1, y, n, m, visited) } else { 0 };
    return 1 + left + right + up + down;
}

fn solve_pt1(a: Vec<Vec<char>>) {
    let (_, score) = find_low_points(&a);
    println!("{}", score);
}

fn solve_pt2(a: Vec<Vec<char>>) {
    let (low_points, _) = find_low_points(&a);
    let n = a.len();
    let m = a[0].len();
    let mut heap = BinaryHeap::new();

    for i in low_points {
        let mut visited = HashSet::new();
        let s = find_basin_size(&a, i.0, i.1, n, m, &mut visited);
        if heap.len() < 3 {
            heap.push(Reverse(s));
        } else {
            if heap.peek() > Some(&Reverse(s)) {
                heap.pop();
                heap.push(Reverse(s));
            }
        }
    }
    let Reverse(v1) = heap.pop().unwrap();
    let Reverse(v2) = heap.pop().unwrap();
    let Reverse(v3) = heap.pop().unwrap();
    println!("{:?}", v1*v2*v3);
}



pub fn run(part: char) {
    let v = read_input_char_vec("data/day9.txt");
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