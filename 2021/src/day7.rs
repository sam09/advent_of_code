use crate::utils::{read_input_string, convert_string_to_int_vec};

fn find_cost_pt1(a: &Vec<i32>, pos: i32) -> i32 {
    let mut cost = 0;
    for i in 0..a.len() {
        cost += (pos - a[i]).abs();
    }
    cost
}


fn find_cost_pt2(a: &Vec<i32>, pos: i32) -> i32 {
    let mut cost = 0;
    for i in 0..a.len() {
        let n = (pos - a[i]).abs();
        cost += (n * (n+1))/2;
    }
    cost
}


fn solve(a: Vec<i32>, f: &dyn Fn(&Vec<i32>, i32) -> i32) {
    let max = a.iter().max().unwrap();
    let mut min_cost = i32::MAX;
    for i in 0..*max {
        let c = f(&a, i);
        if c < min_cost {
            min_cost = c;
        }
    }
    println!("{}", min_cost);
}


pub fn run(part: char) {
    let v = read_input_string("data/day7.txt");
    match v {
        Ok(values) => {
            let a = convert_string_to_int_vec(&values[0], ',');
            match part {
                'a' => solve(a, &find_cost_pt1),
                'b' => solve(a, &find_cost_pt2),
                _ => println!("error occurred parsing options")
            }
        },
        _ => println!("error occurred parsing input")
    };
}