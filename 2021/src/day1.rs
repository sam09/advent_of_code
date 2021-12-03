use crate::utils::read_input_int;

fn solve_pt1(a: Vec<i32>) {
    let n = a.len();
    let mut prev = a[0];
    let mut increased = 0;
    for i in 1..n {
        if a[i] > prev {
            increased += 1;
        }
        prev = a[i];
    }

    println!("{}", increased);
}

fn solve_pt2(a: Vec<i32>) {
    let n = a.len();
    let mut prev = a[0] + a[1] + a[2];
    let mut increased = 0;
    for i in 1..n-2 {
        let window = a[i] + a[i+1] + a[i+2];
        if window > prev {
            increased += 1;
        }
        prev = window;
    }

    println!("{}", increased);
}



pub fn run(part: char) {
    let v = read_input_int("data/day1.txt");
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