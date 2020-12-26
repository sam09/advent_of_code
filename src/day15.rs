use std::collections::HashMap;

fn solve(a: Vec<usize>) -> usize {
    let mut turn = a.len()-1;
    let mut map = HashMap::new();
    let g = 3_000_000;

    for i in 0..a.len() - 1 {
        map.insert(a[i], i);
    }
    let mut last = a[turn];
    let mut next = last;
    while turn < g {
        last = next;
        next = match map.get(&last) {
            Some(val) => turn - val,
            None => 0
        };
        map.insert(last, turn);
        turn += 1;
    }
    last
}

pub fn run() {
    let vals = vec![13,0,10,12,1,5,8];
    println!("{}", solve(vals));
}