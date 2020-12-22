extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day4.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap()
    } ).collect())
}


fn solve(a: Vec<String>) -> usize {
    let mut search = 0;
    let mut valid_passports = 0;
    let re = Regex::new(r"([a-zA-Z]+):([a-zA-Z0-9#]+)").unwrap();
    /*let re = RegexSet::new(&[
            r"(eyr):(202[0-9]|30)",
            r"(byr):(19[2-9][0-9]|200[0-2])",
            r"(iyr):(201[0-9][0-9]|20)",
            r"(hcl):(#[0-9a-f]{6})",
            r"(pid):([0-9]{9})"
            r"(ecl):(amb|blu|brn|gry|grn|hzl|oth)"
            r"(hgt):(1[5-7][0-9]|9[0-3]cm)",
            r"(hgt):(59|6[0-9]|7[[0-6]in)",
        ]
    );*/
    
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

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}