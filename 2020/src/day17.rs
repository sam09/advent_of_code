use std::collections::HashSet;
use std::hash::Hash;
use crate::utils::read_input_string;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Cube {
    n: usize,
    values: Vec<i32>,
}

impl Cube {
    fn get_neighbours(&self) -> Vec<Cube> {
        let mut neighbours = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    if self.n == 4 {
                        for l in -1..2 {
                            if i != 0 || j != 0 || k != 0 || l != 0 {
                                let v = vec![self.values[0] + i, self.values[1] + j, self.values[2] + k, self.values[3] + l];
                                neighbours.push(Cube{n: self.n, values: v.clone()});
                            }
                        }
                    } else {
                        if i != 0 || j != 0 || k != 0 {
                            let v = vec![self.values[0] + i, self.values[1] + j, self.values[2] + k];
                            neighbours.push(Cube{n: self.n, values: v.clone()});
                        }
                    }
                }
            }
        }
        neighbours
    }

    fn is_neighbour(&self, cube: &Cube) -> bool {
        let mut diff;
        let mut all_zero = true;
        for i in 0..self.n {
            diff = self.values[i] - cube.values[i];
            if diff < -1 || diff > 1 {
                return false;
            }
            if diff != 0 {
                all_zero = false;
            }
        }
        !all_zero
    }

    fn is_equal(&self, cube: &Cube) -> bool {
        for i in 0..self.n {
            if self.values[i] != cube.values[i] {
                return false;
            }
        }
        true
    }

    fn is_active(&self, active_cubes: &Vec<Cube>) -> bool {
        for cube in active_cubes {
            if self.is_equal(cube) {
                return true;
            }
        }
        false
    }

    fn find_active_neighbours(&self, active_cubes: &Vec<Cube>) -> i32 {
        let mut active_neighbours = 0;
        for cube in active_cubes {
            if self.is_neighbour(cube) {
                active_neighbours += 1;
            }
        }
        active_neighbours
    }
    fn should_be_active(&self, active_cubes: &Vec<Cube>) -> bool {
        let active_neighbours = self.find_active_neighbours(active_cubes);
        let is_active = self.is_active(active_cubes);
        (is_active == true && active_neighbours == 2) || active_neighbours == 3
    }
}

fn solve(a: Vec<String>, dims: usize) -> usize {
    let mut active_cubes = Vec::new();
    let rounds = 6;
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if a[i].chars().nth(j).unwrap() == '#' {
                let mut v = Vec::new();
                v.push(i as i32);
                v.push(j as i32);
                for _ in 0..dims-2 {
                    v.push(0);
                }
                active_cubes.push(Cube{n: dims, values: v});
            }
        }
    }

    for _ in 0..rounds {
        let mut cubes_to_test = HashSet::new();
        for cube in &active_cubes {
            let neighbours = cube.get_neighbours();
            for neighbour in neighbours {
                cubes_to_test.insert(neighbour);
            }
        }
        let mut active_next_round = Vec::new();
        for cube in cubes_to_test {
            if cube.should_be_active(&active_cubes) {
                active_next_round.push(cube);
            }
        }
        active_cubes.clear();
        active_cubes = active_next_round.clone();
    }

    active_cubes.len()

}

pub fn run(part: char) {
    let v = read_input_string("data/day17.txt");
    match v {
        Ok(values) =>{
            let ans = if part == 'a' { solve(values, 3) } else { solve(values, 4) };
            println!("{}", ans);
        }
        _ => println!("error occurred parsing input")
    };
}


/*
    .#.
    ..#
    ###

    
Cycle 2 ()


    ...
    #..
    ..#
    .#.

    ...
    #.#
    .##
    .#.

(1, 0, -1), (2, 2, -1), (3, 1, -1), (1, 0, 1), (2, 2, 1), (3, 1, 1)
(1, 0, 0), (1, 2, 0), (2, 1, 0), (2, 2, 0), (3, 1, 0)


*/