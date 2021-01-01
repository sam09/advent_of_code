extern crate num_bigint;
extern crate num_traits;
use crate::utils::read_input_string;
use num_bigint::{BigInt, ToBigInt};
use num_traits::{Zero, One};

fn solve_pt1(a: Vec<String>) -> i32 {
    let earliest_time = a[0].parse::<i32>().unwrap();
    let temp = a[1].split(",").collect::<Vec<&str>>();
    let mut buses = Vec::new();
    for i in temp {
        if i != "x" {
            buses.push(i.parse::<i32>().unwrap());
        }
    }
    let mut min_v = earliest_time;
    let mut min_i = -1;
    for i in 0..buses.len() {
        let tp = buses[i] - (earliest_time % buses[i]);
        if tp < min_v {
            min_v = tp;
            min_i = buses[i];
        }
    }
    min_v * min_i
}

fn gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    if *a == Zero::zero() {
        return (Zero::zero(), One::one());
    }
    let (i, j) = gcd(&(b%a), a);
    (j - (b/a) * i.clone() , i)
}

fn inverse(b: &BigInt, m: &BigInt) -> BigInt {
    let (x, _) = gcd(b, m);
    (x % m + m)%m
}

fn solve_pt2(a: Vec<String>) -> BigInt {
    let temp = a[1].split(",").collect::<Vec<&str>>();
    let mut buses = Vec::new();
    let mut prod: BigInt = One::one();
    for i in 0..temp.len() {
        if temp[i] != "x" {
            let tp = temp[i].parse::<i32>().unwrap().to_bigint().unwrap();
            prod = prod * tp.clone();
            buses.push((tp, i));
        }
    }

    let mut sum: BigInt = Zero::zero();
    for bus in &buses {
        let b = &prod/&bus.0;
        let bi = inverse(&b, &bus.0);
        sum = ((bus.1 * b * bi)%&prod + sum) % &prod;
    }
    sum = sum % &prod;
    sum = &prod - sum;
    sum
}

pub fn run(part: char) {
    let v = read_input_string("data/day13.txt");
    match v {
        Ok(values) => {
            if part == 'a' {
                println!("{}", solve_pt1(values));
            } else {
                println!("{}", solve_pt2(values));
            };
        }
        _ => println!("error occurred parsing input")
    };
}