use std::collections::HashSet;
use crate::utils::read_input_string;

fn solve_pt1(a: Vec<String>) -> usize {
    let mut set:HashSet<char> = HashSet::new();
    let mut sum = 0;
    for i in a {
        if i.is_empty() {
            sum += set.len();
            set.clear();
        } else {
            for j in i.chars() {
                set.insert(j);
            }
        }
    }
    sum + set.len()
}


fn solve_pt2(a: Vec<String>) -> usize {
    let mut v = vec![0; 26];
    let mut members = 0;
    let mut sum = 0;
    for i in a {
        if i.is_empty() {
            for j in 0..26 {
                if v[j] == members {
                    sum += 1;
                }
            }
            v = vec![0; 26];
            members = 0;
        } else {
            for j in i.chars() {
                let k = (j as usize) - 97;
                v[k] += 1;
            }
            members += 1;
        }
    }
    for j in 0..26 {
        if v[j] == members {
            sum += 1;
        }
    }
    sum
}

pub fn run(part: char) {
    let v = read_input_string("data/day6.txt");
    match v {
        Ok(values) => {
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans)
        }
        _ => println!("error occurred parsing input")
    };
}