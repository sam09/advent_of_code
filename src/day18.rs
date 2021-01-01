use std::collections::VecDeque;
use crate::utils::read_input_string;

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

fn get_post_fix_pt1(a: &str) -> Vec<char> {
    let mut post_fix = Vec::new();
    let mut stack = VecDeque::new();
    for i in a.chars() {
        if i == '+' || i == '*' {
            if stack.is_empty() || *stack.front().unwrap() == '('  {
                stack.push_front(i);
            } else {
                while !stack.is_empty() && *stack.front().unwrap() != '(' {
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

fn get_post_fix_pt2(a: &str) -> Vec<char> {
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


fn eval(a: &str, part: char) -> i64 {
    let post_fix = if part == 'a' { get_post_fix_pt1(a) } else {get_post_fix_pt2(a) };
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
    stack.pop_front().unwrap()
}

fn solve(a: Vec<String>, part: char) -> i64 {
    let mut sum = 0;
    for i in &a {
        sum += eval(i, part);
    }
    sum
}

pub fn run(part: char) {
    let v = read_input_string("data/day18.txt");
    match v {
        Ok(values) =>{
            let ans = solve(values, part);
            println!("{}", ans);
        }
        _ => println!("error occurred parsing input")
    };
}
