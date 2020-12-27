use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::VecDeque;

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day18.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

fn is_greater_or_equal(a: &char, b: char) -> bool {
    match a {
        '+' => true,
        '*' => match b {
            '*' => true,
            _ => false
        },
        _ => false
    }
}

fn get_post_fix(a: &str) -> Vec<char> {
    let mut post_fix = Vec::new();
    let mut stack = VecDeque::new();
    for i in a.chars() {
        if i == '+' || i == '*' {
            if stack.is_empty() || stack.front().unwrap() == &'(' || i == '+' {
                stack.push_front(i);
            } else {
                while !stack.is_empty() && stack.front().unwrap() != &'(' && is_greater_or_equal(stack.front().unwrap(), i) {
                    post_fix.push(stack.pop_front().unwrap());   
                }
                stack.push_front(i);
            }
        } else if i == '(' {
            stack.push_front(i);
        } else if i == ')' {
            while stack.front().unwrap() != &'(' {
                post_fix.push(stack.pop_front().unwrap());
            }
            stack.pop_front();
        } else if i == ' ' {
            continue;
        } else {
            post_fix.push(i);
        }
    }
    while !stack.is_empty() {
        post_fix.push(stack.pop_front().unwrap());
    }

    post_fix
}


fn eval(a: &str) -> i64 {
    let post_fix = get_post_fix(a);
    let mut stack = VecDeque::new();
    for i in post_fix {
        if i == '+' {
            let val = stack.pop_front().unwrap() + stack.pop_front().unwrap();
            stack.push_front(val);
        } else if i == '*' {
            let val = stack.pop_front().unwrap() * stack.pop_front().unwrap();
            stack.push_front(val);
        } else {
            stack.push_front(i as i64 - 48);
        }
    }
    //println!("{}", stack.front().unwrap());
    stack.pop_front().unwrap()
}

fn solve(a: Vec<String>) -> i64 {
    let mut sum = 0;
    for i in &a {
        sum += eval(i);
    }
    sum
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}
