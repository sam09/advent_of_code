use std::collections::HashMap;

fn solve(a: Vec<usize>, g: usize) -> usize {
    let mut turn = a.len()-1;
    let mut map = HashMap::new();

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

pub fn run(part: char) {
    let values = vec![13,0,10,12,1,5,8];
    let ans = if part == 'a' { solve(values,2020) } else { solve(values, 3_000_000) };
    println!("{}", ans);
}