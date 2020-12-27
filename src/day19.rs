use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day19.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

type RuleId = usize;

#[derive(Debug, Clone)]
enum RuleType {
    Simple(char),
    Compound(Vec<RuleId>),
    Complex(Vec<RuleId>, Vec<RuleId>),
}

#[derive(Debug, Clone)]
struct Rule {
    rule_id: RuleId,
    rule: RuleType
}


fn and_recursive_11<'a>(map: &HashMap<RuleId, Rule>, input: &'a str, tail: i32) -> Result<&'a str, ()> {
    let mut tp = input.clone();
    let rule42 = &map[&42];
    let rule31 = &map[&31];
    match rule42.match_rule(tp, map) {
        Err(e) => {
            let mut i = 0;
            while i < tail {
                match rule31.match_rule(tp, map) {
                    Ok(v) => tp = v,
                    Err(e) => return Err(e),
                };
                i += 1;
            }
            if i == tail {
                return Ok(tp);
            } else {
                return Err(e);
            }
        },
        Ok(v) => {
            if v.is_empty() {
                return Err(());
            } else {
                return and_recursive_11(map, v, tail+1);
            }
        }
    }
}

fn and<'a>(rules: &Vec<RuleId>, map: &HashMap<RuleId, Rule>, input: &'a str) -> Result<&'a str, ()> {
    let mut tp = input.clone();
    //println!("{:?} {}", rules, input);
    for rule_id in rules {
        let rule = map.get(rule_id).unwrap();
        match rule.match_rule(tp, map) {
            Err(e) => return Err(e),
            Ok(v) => tp = v
        }
    }
    Ok(tp)
}

fn or<'a>(first: &Vec<RuleId>, second: &Vec<RuleId>, map: &HashMap<RuleId, Rule>, input: &'a str) -> Result<&'a str, ()> {
    //println!("{:?} {:?} {}", first, second, input);
    match and(first, map, input) {
        Ok(v) => Ok(v),
        Err(_) => and(second, map, input),
    }
}


impl Rule {
    fn match_rule<'a>(&self, input: &'a str, map: &HashMap<RuleId, Rule>) -> Result<&'a str, ()> {
        match &self.rule {
            RuleType::Simple(c) => {
                match input.chars().next() {
                    Some(v) if *c == v  => Ok(&input[1..]),
                    Some(_) => Err(()),
                    None => Ok(""),
                }
            },
            RuleType::Compound(v) => and(&v, map, input),
            RuleType::Complex(a, b) => {
                if self.rule_id == 11 {
                    return and_recursive_11(map, input, 0);
                } else {
                    return or(&a, &b, map, input);
                }
            }
        }
    }
}


fn parse_rule_id_to_vec(a: &str) -> Vec<RuleId> {
    a.split(" ").map(|a| a.parse::<RuleId>().unwrap()).collect()
}

fn parse_rule(a: &str) -> Rule {
    let temp = a.split(": ").collect::<Vec<&str>>();
    let rule_id = temp[0].parse::<RuleId>().unwrap();

    let rule_type= match temp[1].find("\"") {
        Some(_) => RuleType::Simple(temp[1][1..2].chars().next().unwrap()),
        _ => {
            match temp[1].find("|") {
                Some(_) => {
                    let v = temp[1].split(" | ").collect::<Vec<&str>>();
                    RuleType::Complex(parse_rule_id_to_vec(v[0]), parse_rule_id_to_vec(v[1]))
                },
                _ => {
                    RuleType::Compound(parse_rule_id_to_vec(temp[1]))
                }
            }
        }
    };
    Rule{rule_id: rule_id, rule: rule_type}
}

fn solve(a: Vec<String>) -> i64 {
    let mut rules = Vec::new();
    let mut map = HashMap::new();
    let mut i = 0;
    loop {
        if a[i].is_empty() {
            i += 1;
            break;
        }
        let mut rule = parse_rule(&a[i]);
        match rule.rule_id {
            8 => {
                rule.rule = RuleType::Complex(vec![42], vec![42, 8]);
            },
            11 => {
                rule.rule = RuleType::Complex(vec![42, 31], vec![42, 11, 31]);
            },
            _ => ()
        }
        map.insert(rule.rule_id, rule.clone());
        rules.push(rule);
        i+=1;
    }

    let rule_zero = map.get(&0).unwrap();
    let mut valid = 0;
    while i < a.len() {
        match rule_zero.match_rule(&a[i], &map) {
            Ok(v) => {
                if v.is_empty() {
                    valid +=1;
                    println!("{}", a[i]);
                }
            }
            Err(_) => (),
        }
        i += 1;
    }
    valid
}

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values)),
        _ => println!("error occurred parsing input")
    };
}
