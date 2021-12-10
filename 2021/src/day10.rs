use crate::utils::read_input_string;
use std::collections::{HashMap,LinkedList};


fn find_corrupt_score(a: &String, table: &HashMap<char, i32>, conjugate: &HashMap<char, char>) -> i32 {
    let mut last = LinkedList::new();
    for j in a.chars() {
        match j {
            '(' | '[' | '<' | '{' => last.push_back(j),
            ')' | ']' | '>' | '}' => {
                let top = last.back();
                match top {
                    Some(val) => {
                        if *val != conjugate[&j] {
                            return table[&j];
                        } else {
                            last.pop_back();
                        }
                    },
                    None => {
                        return table[&j];
                    }
                }
            },
            _ => ()
        }
    }
    0
}


fn find_incomplete_score(a: &String, table: &HashMap<char, i32>, conjugate: &HashMap<char, char>) -> i64 {
    let mut last = LinkedList::new();
    for j in a.chars() {
        match j {
            '(' | '[' | '<' | '{' => last.push_back(j),
            ')' | ']' | '>' | '}' => {
                let top = last.back();
                match top {
                    Some(val) => {
                        if *val != conjugate[&j] {
                            return 0;
                        } else {
                            last.pop_back();
                        }
                    },
                    None => {
                        return 0;
                    }
                }
            },
            _ => ()
        }
    }

    let mut score: i64 = 0;
    while !last.is_empty() {
        let top = last.back();
        match top {
            Some(val) => score = score * 5 + table[val] as i64, 
            None => ()
        }
        last.pop_back();
    }
    score
}

fn solve_pt1(a: Vec<String>) {
    let table = HashMap::from([
        (')', 3),
        (']', 57), 
        ('}', 1197), 
        ('>', 25137),
    ]);
    let conjugate = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let score: i32 = a.iter().map(|x| find_corrupt_score(&x, &table, &conjugate)).sum();

    println!("{}", score);
}

fn solve_pt2(a: Vec<String>) {
    let table = HashMap::from([
        ('(', 1),
        ('[', 2), 
        ('{', 3), 
        ('<', 4),
    ]);
    let conjugate = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut scores = a.iter().map(|x| find_incomplete_score(&x, &table, &conjugate)).filter(|x| *x != 0).collect::<Vec<i64>>();
    scores.sort();
    println!("{:?}", scores[scores.len()/2]);
}



pub fn run(part: char) {
    let v = read_input_string("data/day10.txt");
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