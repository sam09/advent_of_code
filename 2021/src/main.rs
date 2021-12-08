use std::env;
use std::process;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn print_help() {
    println!("Run using advent_of_code --day day")
}

fn run(a: i32, part: char) {
    match a {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        4 => day4::run(part),
        5 => day5::run(part),
        6 => day6::run(part),
        7 => day7::run(part),
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
