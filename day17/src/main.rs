use std::collections::BinaryHeap;
use std::error::Error;
use std::hash::Hash;
use core::fmt::Debug;
use std::time::Instant;

use aoc::{self, PuzzlePart};
use aoc::astar::{self, AStarGraph, AStarState};

// A* with some additional constraints? Should be fun!
// messed up the algorithm for a bit, but finally worked it out.

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Crucible {
    x: usize,
    y: usize,
    str_dir: (i32, i32),
    str_len: u8,
}

impl Crucible {

    fn move_by(&self, (dx, dy): (i32, i32), (width, height): (usize, usize)) -> Option<Crucible> {
        let next_x = self.x as i32 + dx;
        let next_y = self.y as i32 + dy;
        if next_x < 0 {
            return None;
        }
        if next_y < 0 {
            return None;
        }
        if next_x >= width as i32 {
            return None;
        }
        if next_y >= height as i32 {
            return None;
        }
        let next_len = if dx == self.str_dir.0 && dy == self.str_dir.1 {
            self.str_len + 1
        } else {
            1
        };
        if next_len > 3 {
            return None;
        }
        Some(Crucible {
            x: next_x as usize,
            y: next_y as usize,
            str_dir: (dx, dy),
            str_len: next_len,
        })
    }

    fn move_by_ultra(&self, (dx, dy): (i32, i32), (width, height): (usize, usize)) -> Option<Crucible> {
        let next_x = self.x as i32 + dx;
        let next_y = self.y as i32 + dy;
        if next_x < 0 {
            return None;
        }
        if next_y < 0 {
            return None;
        }
        if next_x >= width as i32 {
            return None;
        }
        if next_y >= height as i32 {
            return None;
        }
        let next_len = if dx == self.str_dir.0 && dy == self.str_dir.1 {
            self.str_len + 1
        } else {
            1
        };
        if next_len > 10 {
            return None;
        }
        Some(Crucible {
            x: next_x as usize,
            y: next_y as usize,
            str_dir: (dx, dy),
            str_len: next_len,
        })
    }

    fn get_neighbours(&self, (width, height): (usize, usize)) -> Vec<Crucible> {
        //println!("{}, {}", self.x, self.y);
        let mut res = Vec::new();
        if let Some(next) = self.move_by(self.str_dir, (width, height)) {
            res.push(next);
        }
        if let Some(next) = self.move_by((self.str_dir.1, self.str_dir.0), (width, height)) {
            res.push(next);
        }
        if let Some(next) = self.move_by((-self.str_dir.1, -self.str_dir.0), (width, height)) {
            res.push(next);
        }
        //println!("{:?}", res);
        res
    }

    fn get_neighbours_ultra(&self, (width, height): (usize, usize)) -> Vec<Crucible> {
        //println!("{}, {}", self.x, self.y);
        let mut res = Vec::new();
        if let Some(next) = self.move_by_ultra(self.str_dir, (width, height)) {
            res.push(next);
        }
        if self.str_len >= 4 {
            if let Some(next) = self.move_by_ultra((self.str_dir.1, self.str_dir.0), (width, height)) {
                res.push(next);
            }
            if let Some(next) = self.move_by_ultra((-self.str_dir.1, -self.str_dir.0), (width, height)) {
                res.push(next);
            }
        }
        //println!("{:?}", res);
        res
    }

}


struct Map {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    heuristic_map: Option<Vec<Vec<Option<usize>>>>,
}

impl Map {

    fn generate_heuristics(&mut self) {
        let mut visited: Vec<Vec<Option<usize>>> = vec![vec![None; self.width]; self.height];
        let mut expanded: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        let start = Crucible { x: self.width - 1, y: self.height - 1, str_dir: (-1, 0), str_len: 0 };
        visited[start.y][start.x] = Some(0);
        let mut open_list: BinaryHeap<AStarState<Crucible>> = BinaryHeap::new();
        open_list.push(AStarState { node: start, estimated_cost: 0 });
        while let Some(next) = open_list.pop() {
            if expanded[next.node.y][next.node.x] {
                continue;
            }
            expanded[next.node.y][next.node.x] = true;
            let current_cost = visited[next.node.y][next.node.x].unwrap();
            //println!("{}", current_cost);
            let add_cost = self.map[next.node.y][next.node.x];
            //println!("{}", current_cost);
            self.get_neighbours(&next.node).into_iter().for_each(|(mut node, _)| {
                node.str_len = 0;
                let next_cost = current_cost + add_cost;
                let compare = visited[node.y][node.x];
                if let Some(c) = compare {
                    if c < next_cost {
                        return;
                    }
                }
                visited[node.y][node.x] = Some(next_cost);
                let estimated = next_cost;
                //println!("cost: {}, {}", next_cost, estimated);
                open_list.push(AStarState { node, estimated_cost: estimated });
            });
            //println!("{:?}", open_list);
            //println!("{:?}", visited);
            //thread::sleep(Duration::from_secs(1));
        }
        self.heuristic_map = Some(visited);
    }

}

impl From<Vec<String>> for Map {

    fn from(value: Vec<String>) -> Self {
        let map: Vec<Vec<usize>> = value.iter().filter(|line| line.len() > 0).map(|line| {
            line.bytes().map(|c| {
                (c - 0x30) as usize
            }).collect()
        }).collect();
        let width = map[0].len();
        let height = map.len();
        Map { map, width, height, heuristic_map: None }
    }

}

impl AStarGraph for Map {

    type Node = Crucible;

    fn get_neighbours(&self, current: &Self::Node) -> Vec<(Self::Node, usize)> {
        let neighbours = match aoc::puzzle_part() {
            PuzzlePart::PartOne => current.get_neighbours((self.width, self.height)),
            PuzzlePart::PartTwo => current.get_neighbours_ultra((self.width, self.height))
        };
        neighbours.into_iter().map(|node| {
            let cost = self.map[node.y][node.x];
            (node, cost)
        }).collect()
    }

    fn get_heuristic(&self, target: &Self::Node) -> usize {
        if let Some(map) = &self.heuristic_map {
            if let Some(h) = map[target.y][target.x] {
                //println!("{}", h);
                return h;
            }
        }
        self.width + self.height - target.x - target.y
    }

    fn is_goal(&self, target: &Self::Node) -> bool {
        target.x == self.width - 1 && target.y == self.height - 1
            && (aoc::puzzle_part() == PuzzlePart::PartOne || target.str_len >= 4)
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut map = Map::from(input);
    let time = Instant::now();
    map.generate_heuristics();
    println!("Time to create heuristic map: {} s", time.elapsed().as_secs_f64());
    //println!("{:?}", map.heuristic_map);
    let start = Crucible { x: 0, y: 0, str_dir: (1, 0), str_len: 0 };
    let time = Instant::now();
    let res = astar::a_star(&map, start);
    println!("Shortest path: {}, found in {} s", res.unwrap(), time.elapsed().as_secs_f64());
    Ok(())
}
