extern crate regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day12.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

fn f(pos: (i32, i32), v: i32, angle: i32) -> (i32, i32) {
    match angle {
        0 => (pos.0 + v, pos.1),
        90 => (pos.0, pos.1 + v),
        180 => (pos.0 - v, pos.1),
        270 => (pos.0, pos.1 - v),
        _ => pos
    }
}

fn abs(a: i32) -> u32 {
    if a < 0 {
        (a * (-1)) as u32
    } else {
        a as u32
    }
}

fn solve_pt1(a: Vec<String>) -> u32 {
    let mut pos = (0, 0);
    let mut angle = 0;

    for i in &a {
        let c = i.chars().nth(0).unwrap();
        let v = i[1..].parse::<i32>().unwrap();
        pos = match c {
            'E' => (pos.0 + v, pos.1),
            'W' => (pos.0 - v, pos.1),
            'N' => (pos.0, pos.1 + v),
            'S' => (pos.0, pos.1 - v),
            'F' => f(pos, v, angle),
            _   => pos
        };
        angle = match c {
            'R' => (angle + 360-v) % 360,
            'L' => (angle + v) % 360,
            _   => angle
        };
    }

    abs(pos.0) + abs(pos.1)
}

fn rotate_point(pos: (i32, i32), angle: i32) -> (i32, i32) {
    match angle {
        0 => pos,
        90 => (pos.1, -pos.0),
        180 => (-pos.0, -pos.1),
        270 => (-pos.1, pos.0),
        _ => pos
    }
}

fn solve(a: Vec<String>) -> u32 {
    let mut pos = (0, 0);
    let mut waypoint = (10, 1);

    for i in &a {
        let c = i.chars().nth(0).unwrap();
        let v = i[1..].parse::<i32>().unwrap();
        match c {
            'E' => {
                waypoint = (waypoint.0 + v, waypoint.1);
            }
            'W' => {
                waypoint = (waypoint.0 - v, waypoint.1);
            }
            'N' => {
                waypoint = (waypoint.0, waypoint.1 + v);
            }
            'S' => {
                waypoint = (waypoint.0, waypoint.1 - v);
            }
            'F' => {
                pos = (pos.0  + waypoint.0 * v, pos.1 + waypoint.1 * v)
            }
            'R' => {
               waypoint = rotate_point(waypoint, v); 
            }
            'L' => {
                waypoint = rotate_point(waypoint, 360-v);
            }
            _ => ()
        };
    }

    abs(pos.0) + abs(pos.1)
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}
