use crate::utils::{read_input_string, convert_string_to_int_vec};

fn get_grids(a: &Vec<String>, n: usize) -> Vec<Vec<Vec<i32>>> {
    let mut grids = Vec::new();
    let mut tp = Vec::new();

    for i in 1..n {    
        if a[i].is_empty() {
            if tp.len() != 0 {
                grids.push(tp);
                tp = Vec::new();
            }
            continue
        }
        tp.push(convert_string_to_int_vec(&a[i], ' '));
    }
    if tp.len() != 0 {
        grids.push(tp);
        tp = Vec::new();
    }
    grids
}


fn check_grid(grid: &Vec<Vec<i32>>) -> bool {
    for i in 0..5 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for j in 0..5 {
            sum1 += grid[i][j];
            sum2 += grid[j][i];
        }

        if sum1 == -5 || sum2 == -5 {
            return true;
        }
    }
    false
}

fn leftover(grid: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if grid[i][j] != -1 {
                sum += grid[i][j];
            }
        }
    }
    sum
}

fn solve_pt1(a: Vec<String>) {
    let numbers = convert_string_to_int_vec(&a[0], ',');
    let n = a.len();
    let mut grids = get_grids(&a, n);
    let m = numbers.len();

    for i in 0..m {
        for j in 0..grids.len() {
            for k in 0..5 {
                for l in 0..5 {
                    if grids[j][k][l] == numbers[i] {
                        grids[j][k][l] = -1
                    }
                }
            }
            if check_grid(&grids[j]) {
                println!("{:?}", numbers[i] * leftover(&grids[j]));
                return;
            }
        }
    }
}

fn solve_pt2(a: Vec<String>) {
    let numbers = convert_string_to_int_vec(&a[0], ',');
    let n = a.len();
    let mut grids = get_grids(&a, n);
    let m = numbers.len();
    let mut won_grids = 0;
    let mut marked = vec![false; grids.len()];

    for i in 0..m {
        for j in 0..grids.len() {
            for k in 0..5 {
                for l in 0..5 {
                    if grids[j][k][l] == numbers[i] {
                        grids[j][k][l] = -1
                    }
                }
            }
            if check_grid(&grids[j]) && marked[j] == false {
                won_grids += 1;
                marked[j] = true;
                if won_grids == grids.len() {
                    println!("{:?}", numbers[i] * leftover(&grids[j]));
                    return;
                }
            }
        }
    }
}


pub fn run(part: char) {
    let v = read_input_string("data/day4.txt");
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