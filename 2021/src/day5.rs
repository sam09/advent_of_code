use crate::utils::{read_input_string, convert_string_to_usize_vec};
use std::cmp::max;
use std::cmp::min;


fn parse(a: Vec<String>) -> Vec<Vec<Vec<usize>>> {
    let mut points = Vec::new();
    for i in 0..a.len() {
        let tp = a[i].split("->").map(|x| x.trim()).collect::<Vec<&str>>();
        let x = convert_string_to_usize_vec(&tp[0].to_string(), ',');
        let y = convert_string_to_usize_vec(&tp[1].to_string(), ',');
        points.push(vec![x, y]);
    }
    points
}

fn solve(a: Vec<String>, diag: bool) {
    let points = parse(a);
    let mut x_max = 0 as usize;
    let mut y_max = 0 as usize;

    for i in 0..points.len() {
        let p1 = &points[i][0];
        let p2 = &points[i][1];
        if p1[0] > x_max {
            x_max = p1[0];
        }
        if p2[0] > x_max {
            x_max = p2[0];
        }
        if p1[1] > y_max {
            y_max = p1[1];
        }
        if p2[1] > y_max {
            y_max = p2[1];
        }
    }

    let mut grid = vec![vec![0; y_max+1]; x_max+1];

    for i in 0..points.len() {
        let p1 = &points[i][0];
        let p2 = &points[i][1];
        
        if p1[0] == p2[0] {
            let start = min(p1[1], p2[1]);
            let end = max(p1[1], p2[1]) + 1;
            for j in start..end {
                grid[p1[0]][j] += 1;
            }
        } else if p2[1] == p1[1] {
            let start = min(p1[0], p2[0]);
            let end = max(p1[0], p2[0]) + 1;
            for j in start..end {
                grid[j][p1[1]] += 1;
            }
        } else if diag {
            let slope = (p2[1] as f32 - p1[1] as f32) / (p2[0] as f32 - p1[0] as f32);
            let intercept = p1[1] as f32 - slope * p1[0] as f32;
            if slope == 1.0 || slope == (-1.0) {
                let end_x = max(p1[0], p2[0]) + 1;
                let start_x = min(p1[0], p2[0]);
                for j in start_x .. end_x {
                    let y = (slope * (j as f32) + intercept as f32) as usize;
                    grid[j][y] += 1;
                }
            }
        }
    }
    let mut c = 0;
    for i in 0..x_max+1 {
        for j in 0..y_max+1 {
            if grid[i][j] > 1 {
                c += 1;
            }
        }
    }
    println!("{}", c);
}


pub fn run(part: char) {
    let v = read_input_string("data/day5.txt");
    match v {
        Ok(values) => {
            match part {
                'a' => solve(values, false),
                'b' => solve(values, true),
                _ => println!("error occurred parsing options")
            }
        },
        _ => println!("error occurred parsing input")
    };
}