use std::collections::{HashMap, VecDeque, HashSet};
use crate::utils::read_input_string;

struct Graph {
    edges: Vec<Vec<(usize, i32)>>,
    nodes: HashMap<String, usize>,
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}


fn get_colors_list(a: &Vec<String>) -> HashMap<String, usize> {
    let mut colors = HashMap::new();
    let mut index = 0;
    for i in a {
        let temp = i.split("contain").collect::<Vec<&str>>();
        if !colors.contains_key(temp[0]) {
            colors.insert(remove_whitespace(temp[0]), index);
            index += 1;
        }
    }
    colors
}

fn create_rev_graph(a: Vec<String>) -> Graph {
    let colors = get_colors_list(&a);
    let mut edge_list = vec![vec![(0,0); 0]; colors.len()];
    
    for i in &a {
        let temp = i.split("contain").collect::<Vec<&str>>();
        let edges = temp[1][..temp[1].len()-1].split(",").collect::<Vec<&str>>();
        let source_index = colors[&remove_whitespace(temp[0])];

        for edge in edges {
            if edge != " no other bags" {
                let str_sep = edge.split(" ").collect::<Vec<&str>>();
                let value = str_sep[1];
                let dest_color = str_sep[2..].join("") + if value == "1" { "s" } else { "" };
                let dest_index = *colors.get(&dest_color).unwrap();
                edge_list[dest_index].push((source_index, value.parse::<i32>().unwrap()));
            }
        }
    }
    Graph {
        edges: edge_list,
        nodes: colors,
    }
}

fn create_graph(a: Vec<String>) -> Graph {
    let colors = get_colors_list(&a);
    let mut edge_list = vec![vec![(0,0); 0]; colors.len()];

    for i in &a {
        let temp = i.split("contain").collect::<Vec<&str>>();
        let edges = temp[1][..temp[1].len()-1].split(",").collect::<Vec<&str>>();
        let source_index = colors[&remove_whitespace(temp[0])];

        for edge in edges {
            if edge != " no other bags" {
                let str_sep = edge.split(" ").collect::<Vec<&str>>();
                let value = str_sep[1];
                let dest_color = str_sep[2..].join("") + if value == "1" { "s" } else { "" };
                let dest_index = *colors.get(&dest_color).unwrap();
                edge_list[source_index].push((dest_index, value.parse::<i32>().unwrap()));
            }
        }
    }

    Graph {
        edges: edge_list,
        nodes: colors,
    }
}

fn solve_pt2(a: Vec<String>) -> i32 {
    let graph = create_graph(a);
    let target_bag = "shinygoldbags";
    let source_index = graph.nodes.get(target_bag).unwrap();
    let mut stack = VecDeque::new();
    stack.push_front((source_index, 1));
    let mut fin = 0;
    while !stack.is_empty() {
        match stack.pop_front() {
            Some((y, value)) => {
                for i in &graph.edges[*y] {
                    stack.push_front((&i.0, value * &i.1));
                }
                fin += value;
            },
            None => ()
        }
    }
    fin - 1
}

fn solve_pt1(a: Vec<String>) -> i32 {
    let graph = create_rev_graph(a);
    let target_bag = "shinygoldbags";
    let source_index = graph.nodes.get(target_bag).unwrap();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(source_index);
    
    while !queue.is_empty() {
        match queue.pop_front() {
            Some(y) => {
                visited.insert(y);
                for i in &graph.edges[*y] {
                    if !visited.contains(&i.0) {
                       queue.push_back(&i.0);
                    }
                }
            },
            None => ()
        }
    }
    (visited.len() - 1) as i32
}

pub fn run(part: char) {
    let v = read_input_string("data/day7.txt");
    match v {
        Ok(values) => {
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans)
        }
        _ => println!("error occurred parsing input")
    };
}

/*
Red    
Orange
White - Red, Orange
Yellow - Red, Orange
Gold - White, Yellow
Olive - Gold
Plum - Gold
Blue - Yellow, Olive, Plum
Black - Olive, Plum


*/