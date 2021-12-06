use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod utils;

fn print_help() {
    println!("Run using advent_of_code --day day")
}

fn run(a: i32, part: char) {
    match a {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        _ => println!("Not found")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "--day" || args[1] == "-d" {
        let day = args[2].parse::<i32>().unwrap();
        let part = if args.len() > 3 && (args[3] == "--part" || args[3] == "-p") {args[4].chars().next().unwrap()} else { 'a' };
        run(day, part);
    } else {
        print_help();
        process::exit(1)
    }

}
