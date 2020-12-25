use std::env;
use std::process;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day13;
mod day12;

fn print_help() {
    println!("Run using advent_of_code --day day")
}

fn run(a: i32) {
    match a {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        10 => day10::run(),
        11 => day11::run(),
        12 => day12::run(),
        13 => day13::run(),
        _ => println!("Not found")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "--day" || args[1] == "-d" {
        let day = args[2].parse::<i32>().unwrap();
        run(day);
    } else {
        print_help();
        process::exit(1)
    }

}
