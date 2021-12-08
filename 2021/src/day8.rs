use crate::utils::read_input_string;
use std::collections::{HashSet, HashMap};

fn solve_pt1(a: Vec<String>) {
    let mut c = 0;
    for i in 0..a.len() {
        let temp = a[i].split("|").map(|x| x.trim()).collect::<Vec<&str>>();
        let output = temp[1];
        let words = output.split(" ").collect::<Vec<&str>>();
        for j in words {
            match j.len() {
                3 | 4 | 2 | 7 => c += 1,
                _ => ()
            }
        }
    }

    println!("{:?}", c);
}


fn diff2(a: &str, key: &str) -> bool {
    let mut mp = HashSet::new();
    for i in key.chars() {
        mp.insert(i);
    }
    let mut diff = 0;
    for i in a.chars() {
        if !mp.contains(&i) {
            diff += 1;
        }
    }
    diff == 2
}

fn is5(a: &str, nine: &str, three: &str) -> bool {
    let mut mp = HashSet::new();
    for i in nine.chars() {
        mp.insert(i);
    }
    for i in three.chars() {
        mp.remove(&i);
    }
    
    let mut diff = false;
    for i in a.chars() {
        if mp.contains(&i) {
            diff = true;
        }
    }
    diff
}

fn is0(a: &str, one: &str) -> bool {
    let mut mp = HashSet::new();
    for i in one.chars() {
        mp.insert(i);
    }

    let mut diff = 0;
    for i in a.chars() {
        if mp.contains(&i) {
            diff +=1;
        }
    }
    diff == one.len()
}

fn sort(a: &str) -> String {
    let mut tp = a.chars().collect::<Vec<char>>();
    tp.sort();
    return tp.into_iter().collect::<String>();
}
fn solve_pt2(a: Vec<String>) {
    let mut sum = 0;
    for i in 0..a.len() {
        let temp = a[i].split("|").map(|x| x.trim()).collect::<Vec<&str>>();
        let output = temp[1];
        let input = temp[0];
        let words = output.split(" ").map(|x| sort(x)).collect::<Vec<String>>();
        let patterns = input.split(" ").map(|x| sort(x)).collect::<Vec<String>>();

        let mut mp = HashMap::new();
        let mut rmp = HashMap::new();
        while mp.len() < 10 {
            for l in &patterns {
                match l.len() {
                    3 => {
                        mp.insert(l, 7);
                        rmp.insert(7, l);
                    },
                    4 => {
                        mp.insert(l, 4);
                        rmp.insert(4, l);
                    }
                    2 => {
                        mp.insert(l,1);
                        rmp.insert(1, l);
                    },
                    7 => {
                        mp.insert(l, 8);
                        rmp.insert(8, l);
                    },
                    5 => {
                        if rmp.contains_key(&7) && diff2(&l, &rmp[&7]) {
                            mp.insert(l, 3);
                            rmp.insert(3, l);
                        } else if rmp.contains_key(&3) && rmp.contains_key(&9) && is5(&l, &rmp[&9], &rmp[&3]) {
                            mp.insert(l, 5);
                            rmp.insert(5, l);
                        } else if rmp.contains_key(&7) && rmp.contains_key(&3) && rmp.contains_key(&9) {
                            mp.insert(l, 2);
                            rmp.insert(2, l);
                        }
                    },
                    6 => {
                        if rmp.contains_key(&4) && diff2(&l, &rmp[&4]) {
                            mp.insert(l, 9);
                            rmp.insert(9, l);
                        } else if rmp.contains_key(&1) && is0(&l, &rmp[&1]) {
                            mp.insert(l, 0);
                            rmp.insert(0, l);
                        } else if rmp.contains_key(&4) && rmp.contains_key(&1) {
                            mp.insert(l, 6);
                            rmp.insert(6, l);
                        }
                    },
                    _ => ()
                }
            }
        }
        let mut value = 0;
        for l in &words {
            let digit = mp.get(&l).unwrap();
            value = value * 10 + digit;
        }
        sum += value;
    }
    println!("{}", sum);

    /*
    eafb -> 4
    dab -> 7
    ab -> 1
    acedgfb -> 8
    (4 + 2) cefabd -> 9
    (7 + 2) fbcad -> 3
    (9-3) -> e
    cdfbe -> 5 (because it has e)
    gcdfa -> 2 (becuase it doesn't have e)
    cdfgeb -> 6 (because it has no 1)
    cagedb -> 0 (because it has 1)
    */
}



pub fn run(part: char) {
    let v = read_input_string("data/day8.txt");
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