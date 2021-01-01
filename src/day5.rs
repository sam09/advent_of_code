use crate::utils::read_input_string;

fn bin_search(n: i32, row_str: &str, f: char, b: char) -> i32 {
    let mut min = 0;
    let mut max = n;
    let mut mid;

    for i in row_str.chars() {
        mid = (min+max)/2;
        //println!("{} {} {}", min, max, mid);
        if i == f {
            max = mid;
        } else if i == b {
            min = mid+1;
        }
    }
    (max + min)/2
}

fn find_seat_id(a: String) -> i32 {
    let row_str = &a[..7];
    let col_str = &a[7..];

    let row = bin_search(127, row_str, 'F', 'B');
    let col = bin_search(7, col_str, 'L', 'R');
    //println!("{} {} {}", row, col, row*8+col);
    row * 8 + col
}

fn solve_pt1(a: Vec<String>) -> i32 {
    let mut max_seat_id = 0;
    for i in a {
        let seat_id = find_seat_id(i);
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    max_seat_id
}

fn solve_pt2(a: Vec<String>) -> i32{
    let mut v: Vec<i32>  = Vec::new();
    for i in a {
        v.push(find_seat_id(i));
    }
    v.sort();

    for i in 1..v.len() {
        if (v[i] - 1) != v[i-1]  {
            return v[i] - 1;
        }
    }
    -1
}

pub fn run(part: char) {
    let v = read_input_string("data/day5.txt");
    match v {
        Ok(values) => {
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans)
        },
        _ => println!("error occurred parsing input")
    };
}