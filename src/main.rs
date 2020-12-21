use std::env;
use std::process;
mod day1;
mod day2;
mod day3;

fn print_help() {
    println!("Run using advent_of_code --day day")
}

fn run(a: i32) {
    match a {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
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
