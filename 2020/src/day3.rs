use crate::utils::read_input_string;

fn find_slope(a: &Vec<String>, right_inc: usize, down_inc: usize) -> i64 {
    let mut right = 0;
    let mut down = 0;
    let mut trees = 0;
    let n = a.len();
    let str_n = a[0].len();
    while down < n {
        let x = a[down].chars().nth(right).unwrap();
        if x == '#' {
            trees += 1
        }
        down += down_inc;
        right = (right + right_inc)%str_n;
    }
    trees
}

fn solve(a: Vec<String>, part: char) -> i64{
    return if part == 'b' {
        find_slope(&a, 1, 1) * find_slope(&a, 3, 1)
            * find_slope(&a, 5, 1) * find_slope(&a, 7, 1)
            * find_slope(&a, 1, 2)
    } else {
        find_slope(&a, 3, 1)
    }
}

pub fn run(part: char) {
    let v = read_input_string("data/day3.txt");
    match v {
        Ok(values) => println!("{}", solve(values, part)),
        _ => println!("error occurred parsing input")
    };
}