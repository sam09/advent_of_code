use crate::utils::read_input_string;

fn solve_pt1(a: Vec<String>) {
    let n = a.len();
    let m = a[0].len();

    let mut count = vec![0; m];

    for i in 0..n {
        for j in 0..m {
            let c = a[i].chars().nth(j).unwrap();
            match c {
                '0' => count[j] += 1,
                '1' => count[j] -= 1,
                _ => println!("Invalid input")
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..m {
        if count[i] >= 0 {
            gamma = gamma * 2 + 0;
            epsilon = epsilon * 2 + 1;
        } else {
            gamma = gamma * 2 + 1;
            epsilon = epsilon * 2 + 0;
        }
    }

    println!("{}", gamma * epsilon);
}


fn is_oxygen(x: String, i: usize, count: i32) -> bool {
    let c = x.chars().nth(i).unwrap();
    return c == if count > 0 { '0' } else { '1' };
}

fn is_carbon(x: String, i: usize, count: i32) -> bool {
    let c = x.chars().nth(i).unwrap();
    return c == if count > 0 { '1' } else { '0' };
}

fn find_rating(a: &Vec<String>, f: &dyn Fn(String, usize, i32) -> bool) -> i32 {
    let mut b = a.clone();
    let m = a[0].len();

    for i in 0..m {
        let mut count = 0;
        for j in 0..b.len() {
            let c = b[j].chars().nth(i).unwrap();
            match c {
                '0' => count += 1,
                '1' => count -= 1,
                _ => println!("Invalid input")
            }
        }
        
        b = b.into_iter().filter(|x| f(x.to_string(), i, count)).collect();
        if b.len() == 1 {
            break
        }
    }
    let mut o = 0;
    for i in 0..m {
        o = o * 2 + ( if b[0].chars().nth(i).unwrap() == '0' { 0 } else { 1 } );
    }
    o
}


fn solve_pt2(a: Vec<String>) {
    let oxygen = find_rating(&a, &is_oxygen);
    let carbon  = find_rating(&a, &is_carbon);

    println!("{}", oxygen * carbon);
}


pub fn run(part: char) {
    let v = read_input_string("data/day3.txt");
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