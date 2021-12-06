use crate::utils::read_input_string;

fn solve_pt1(a: Vec<String>) {
    let n = a.len();
    let mut h = 0;
    let mut v = 0;

    for i in 0..n {
        let temp = a[i].split(" ").collect::<Vec<&str>>();
        let val = temp[1].parse::<i32>().unwrap();
        match temp[0] {
            "forward" => h += val,
            "down" => v+= val,
            "up" => v -= val,
            _ => println!("Invalid input")
        }
    }

    println!("{}", h*v);
}


fn solve_pt2(a: Vec<String>) {
    let n = a.len();
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;

    for i in 0..n {
        let temp = a[i].split(" ").collect::<Vec<&str>>();
        let val = temp[1].parse::<i32>().unwrap();
        match temp[0] {
            "forward" => {
                h += val;
                v += (aim * val)
            },
            "down" => aim += val,
            "up" => aim -= val,
            _ => println!("Invalid input")
        }
    }

    println!("{}", h*v);
}


pub fn run(part: char) {
    let v = read_input_string("data/day2.txt");
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