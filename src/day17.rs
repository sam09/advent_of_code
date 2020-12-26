use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashSet;
use std::hash::Hash;

fn read_input()-> io::Result<Vec<String>> {
    let filename = "./data/day17.txt";
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Cube {
    fn get_neighbours(&self) -> Vec<Cube> {
        let mut neighbours = Vec::new();
        for i in vec![1, -1, 0] {
            for j in vec![1, -1, 0] {
                for k in vec![1, -1, 0] {
                    for l in vec![1, -1, 0] {
                        neighbours.push(Cube{x: self.x + i, y: self.y + j, z: self.z + k, w: self.w + l});
                    }
                }
            }
        }
        neighbours
    }

    fn is_neighbour(self, cube: &Cube) -> bool {
        let dx = self.x - cube.x;
        let dy = self.y - cube.y;
        let dz = self.z - cube.z;
        let dw = self.w - cube.w;

        (dx >= -1 && dx <= 1 && dy >= -1 && dy <= 1 && dz >= -1 && dz <= 1 && dw <= 1 && dw >= -1) && (dx != 0 || dy != 0 || dz != 0 || dw != 0)
    }

    fn find_active_neighbours(&self, active_cubes: &Vec<Cube>) -> (i32, bool) {
        let mut active_neighbours = 0;
        let mut is_active = false;
        for cube in active_cubes {
            if self.x == cube.x && self.y == cube.y && self.z == cube.z && self.w == cube.w {
                is_active = true;
            } else if self.is_neighbour(cube) {
                active_neighbours += 1;
            }
        }
        (active_neighbours, is_active)
    }
    fn should_be_active(&self, active_cubes: &Vec<Cube>) -> bool {
        let (active_neighbours, is_active) = self.find_active_neighbours(active_cubes);
        (is_active == true && active_neighbours == 2) || active_neighbours == 3
    }
}

fn solve(a: Vec<String>, rounds: i32) -> usize {
    let mut active_cubes = Vec::new();
    
    for i in 0..a.len() {
        for k in 0..a[i].len() {
            if a[i].chars().nth(k).unwrap() == '#' {
                active_cubes.push(Cube{x: i as i32, y: k as i32, z: 0, w: 0});
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

pub fn run() {
    let vals = read_input();
    match vals {
        Ok(values) => println!("{}", solve(values, 6)),
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