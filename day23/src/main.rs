// interesting. From what I've read, this seems to be an NP-hard problem, so not
// sure how much optimization I can do here. Maybe I'll just let it run for
// a while.

// I guess one thing I can do is to use loops instead of recursion.

// never mind, the program finished before I could refactor. So I guess this
// solution is good enough.

use std::{error::Error, collections::{HashMap, HashSet}};

use aoc::{self, PuzzlePart};

#[derive(Debug)]
struct Tile {
    enter_dirs: u8,
    exit_dirs: u8,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        let (enter_dirs, exit_dirs) = match aoc::puzzle_part() {
            PuzzlePart::PartOne => match value {
                '.' => (0b1111, 0b1111),
                '#' => (0b0000, 0b0000),
                '^' => (0b1111, 0b1000),
                '>' => (0b1111, 0b0100),
                'v' => (0b1111, 0b0010),
                '<' => (0b1111, 0b0001),
                _ => (0b0000, 0b0000),
            },
            PuzzlePart::PartTwo => match value {
                '#' => (0b0000, 0b0000),
                _ => (0b1111, 0b1111),
            }
        };
        Tile { enter_dirs, exit_dirs }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Graph {
    goal: Node,
    nodes: HashMap<Node, Option<usize>>,
    edges: HashMap<Node, Vec<(Node, usize)>>,
    longest_path: Option<usize>,
}

impl Graph {

    fn tile_neighbours(map: &Vec<Vec<Tile>>, node: &Node) -> Vec<Node> {
        let mut res = Vec::new();
        let x = node.x;
        let y = node.y;
        let w = map[0].len();
        let h = map.len();
        let cur_tile = &map[y][x];
        if y > 0 && cur_tile.exit_dirs & 0b1000 > 0 {
            if map[y - 1][x].enter_dirs & 0b1000 > 0 {
                res.push(Node { x: x, y: y - 1 });
            }
        }
        if x > 0 && cur_tile.exit_dirs & 0b0001 > 0 {
            if map[y][x - 1].enter_dirs & 0b0001 > 0 {
                res.push(Node { x: x - 1, y: y });
            }
        }
        if y < h - 1 && cur_tile.exit_dirs & 0b0010 > 0 {
            if map[y + 1][x].enter_dirs & 0b0010 > 0 {
                res.push(Node { x: x, y: y + 1 });
            }
        }
        if x < w - 1 && cur_tile.exit_dirs & 0b0100 > 0 {
            if map[y][x + 1].enter_dirs & 0b0100 > 0 {
                res.push(Node { x: x + 1, y: y });
            }
        }
        res
    }

    fn new(map: &Vec<Vec<Tile>>) -> Graph {
        let mut nodes: HashMap<Node, Option<usize>> = HashMap::new();
        let w = map[0].len();
        let h = map.len();
        nodes.insert(Node { x: 1, y: 0, }, Some(0));
        nodes.insert(Node { x: w - 2, y: h - 1, }, None);
        for x in 0..w {
            for y in 0..h {
                let node = Node { x, y };
                let cons = Self::tile_neighbours(&map, &node);
                //println!("{:?}, {:?}", node, cons);
                if cons.len() >= 3 {
                    nodes.insert(node, None);
                }
            }
        }
        //println!("{:?}", nodes);
        let mut edges: HashMap<Node, Vec<(Node, usize)>> = HashMap::new();
        for node in nodes.keys() {
            let mut connected_nodes = Vec::new();
            let mut heads = vec![node.clone()];
            let mut new_heads = Vec::new();
            let mut d = 0;
            let mut visited: HashSet<Node> = HashSet::new();
            visited.insert(node.clone());
            while heads.len() > 0 {
                while let Some(h) = heads.pop() {
                    if &h != node && nodes.get(&h).is_some() {
                        connected_nodes.push((h.clone(), d));
                        continue
                    }
                    let mut neighbours = Self::tile_neighbours(&map, &h);
                    //println!("{:?}", neighbours);
                    while let Some(nh) = neighbours.pop() {
                        if visited.get(&nh).is_some() {
                            continue;
                        }
                        visited.insert(nh.clone());
                        new_heads.push(nh);
                    }
                }
                while let Some(nh) = new_heads.pop() {
                    heads.push(nh.clone());
                }
                d += 1;
            }
            edges.insert(node.clone(), connected_nodes);
        }
        //for (k, v) in &edges {
        //    println!("{:?} -> {:?}", k, v);
        //}
        Graph { goal: Node { x: w - 2, y: h - 1}, nodes, edges, longest_path: None }
    }

    fn propagate(&mut self, current: &Node, current_cost: usize, visited: &mut HashSet<Node>) {
        if current == &self.goal {
            //println!("Found path to exit: Length {}", current_cost);
            self.longest_path = Some(match self.longest_path {
                Some(l) => l.max(current_cost),
                None => current_cost,
            });
            return;
        }
        visited.insert(current.clone());
        let edges = self.edges.get(current).unwrap().clone();
        for (next, cost) in edges {
            if visited.contains(&next) {
                continue;
            }
            self.propagate(&next, current_cost + cost, visited);
        }
        visited.remove(current);
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let map = aoc::read_map::<Tile>()?;
    let mut graph = Graph::new(&map);
    let mut visited: HashSet<Node> = HashSet::new();
    graph.propagate(&Node { x: 1, y: 0 }, 0, &mut visited);
    let end_cost = graph.longest_path;
    println!("{:?}", end_cost);
    Ok(())
}
