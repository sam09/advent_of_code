extern crate regex;
use regex::Regex;
use crate::utils::read_input_string;

fn solve_pt1(a: Vec<String>) -> usize {
    let mut search = 0;
    let mut valid_passports = 0;
    let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z0-9#]+)").unwrap();
    for i in a {
        if i.is_empty() {
            if search >= 7 {
                valid_passports += 1;
            }
            search = 0;
        } else {
            for cap in re.captures_iter(&i) {
                match &cap[1] {
                    "eyr" => search += 1,
                    "byr" => search += 1,
                    "iyr" => search += 1,
                    "hgt" => search += 1,
                    "hcl" => search += 1,
                    "ecl" => search += 1,
                    "pid" => search += 1,
                    _ => ()
                }
            }
        }
    }
    if search >= 7 {
        valid_passports += 1;
    }
    valid_passports
}

fn solve_pt2(a: Vec<String>) -> usize {
    let mut search = 0;
    let mut valid_passports = 0;
    let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z0-9#]+)").unwrap();
    for i in a {
        if i.is_empty() {
            if search >= 7 {
                valid_passports += 1;
            }
            search = 0;
        } else {
            for cap in re.captures_iter(&i) {
                match &cap[1] {
                    "eyr" => {
                        if &cap[2] >= "2020" && &cap[2] <= "2030"  && cap[2].len() == 4 {
                            search += 1;
                        }
                    },
                    "byr" => {
                        if &cap[2] >= "1920" && &cap[2] <= "2002" && cap[2].len() == 4 {
                            search += 1;
                        }
                    },
                    "iyr" => {
                        if &cap[2] >= "2010" && &cap[2] <= "2020" && cap[2].len() == 4 {
                            search += 1;
                        }
                    },
                    "hgt" => {
                        let sz = cap[2].len();
                        if sz == 5 {
                            let val = &cap[2][..3];
                            let unit = &cap[2][3..];
                            if unit == "cm" && val >= "150" && val <= "193" {
                                search += 1;
                            }
                        } else if sz == 4 {
                            let val = &cap[2][..2];
                            let unit = &cap[2][2..];
                            if unit == "in" && val >= "59" && val <= "76" {
                                search += 1;
                            }
                        }
                    },
                    "hcl" => {
                        if &cap[2] >= "#000000" && &cap[2] <= "#ffffff" && cap[2].len() == 7 {
                            search += 1;
                        }
                    },
                    "ecl" => {
                        if vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&cap[2]) {
                            search += 1;
                        }
                    },
                    "pid" => {
                        if &cap[2] >= "000000000" && &cap[2] <= "999999999" && cap[2].len() == 9 {
                            search += 1;
                        }
                    },
                    _ => ()
                }
            }
        }
    }
    if search >= 7 {
        valid_passports += 1;
    }
    valid_passports
}

pub fn run(part: char) {
    let v = read_input_string("data/day4.txt");
    match v {
        Ok(values) => {
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans)
        },
        _ => println!("error occurred parsing input")
    };
}