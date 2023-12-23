// had to fight the borrow checker for a bit, but eventually didn't turn out too badly.
// still need to figure out how to handle references in rust.

use std::{error::Error, str::FromStr, collections::{HashSet, HashMap}};
use aoc::{self, PuzzlePart, ParseLineError};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Block {
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    h: usize,
    d: usize,
}

fn str_to_coords(s: &str) -> Vec<usize> {
    let coords: Vec<&str> = s.split(",").collect();
    coords.iter().filter_map(|c| match c.parse() {
        Ok(r) => Some(r),
        Err(_) => None,
    }).collect()
}

impl FromStr for Block {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("~");
        let (x, y, z) = if let Some(coords) = split.next() {
            let coords = str_to_coords(coords);
            if coords.len() != 3 {
                return Err(ParseLineError::new("Block", s));
            }
            (coords[0], coords[1], coords[2])
        } else {
            return Err(ParseLineError::new("Block", s));
        };
        let (w, h, d) = if let Some(coords) = split.next() {
            let coords = str_to_coords(coords);
            if coords.len() != 3 {
                return Err(ParseLineError::new("Block", s));
            }
            (coords[0] + 1 - x, coords[1] + 1 - y, coords[2] + 1 - z)
        } else {
            return Err(ParseLineError::new("Block", s));
        };
        Ok(Block { x, y, z, w, h, d })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut blocks: Vec<Block> = input.iter().filter_map(|line| match line.parse() {
        Ok(r) => Some(r),
        Err(_) => None,
    }).collect();
    blocks.sort_by(|a, b| a.z.cmp(&b.z));
    let total_blocks = blocks.len();
    //println!("{:?}", blocks);
    let mut z_map: Vec<Vec<(usize, Option<&Block>)>> = vec![vec![(0, None); 10]; 10];
    blocks.iter_mut().for_each(|b| {
        let mut max_height = 0;
        for x in b.x..b.x + b.w {
            for y in b.y..b.y + b.h {
                let height = z_map[y][x].0;
                if height > max_height {
                    max_height = height;
                }
            }
        }
        b.z = max_height + 1;
        for x in b.x..b.x + b.w {
            for y in b.y..b.y + b.h {
                z_map[y][x] = (b.z + b.d - 1, Some(b));
            }
        }
    });
    let mut z_map: Vec<Vec<(usize, Option<&Block>)>> = vec![vec![(0, None); 10]; 10];
    let mut required: HashSet<&Block> = HashSet::new();
    let mut support_map: HashMap<&Block, HashSet<&Block>> = HashMap::new();
    blocks.iter().for_each(|b| {
        let mut supports: HashSet<&Block> = HashSet::new();
        for x in b.x..b.x + b.w {
            for y in b.y..b.y + b.h {
                let height = z_map[y][x];
                if height.0 == b.z - 1 {
                    if let Some(support) = height.1 {
                        supports.insert(support);
                    }
                }
                z_map[y][x] = (b.z + b.d - 1, Some(b));
            }
        }
        if supports.len() == 1 {
            supports.iter().for_each(|s| {
                required.insert(s);
            });
        }
        support_map.insert(b, supports);
    });
    let res = match aoc::puzzle_part() {
        PuzzlePart::PartOne => total_blocks - required.len(),
        PuzzlePart::PartTwo => {
            required.iter().map(|r| {
                //println!("{:?}", r);
                let mut removed_list: HashSet<&Block> = HashSet::new();
                removed_list.insert(r);
                let mut new_removals = true;
                while new_removals {
                    new_removals = false;
                    blocks.iter().for_each(|b| {
                        if removed_list.get(b).is_some() {
                            return;
                        }
                        let supports = support_map.get(b).unwrap();
                        //println!("supports: {:?}", supports);
                        if supports.is_empty() {
                            return;
                        }
                        for s in supports {
                            if removed_list.get(s).is_none() {
                                return;
                            }
                        }
                        new_removals = true;
                        removed_list.insert(b);
                        //println!("remove {:?}", b);
                    });
                }
                removed_list.len() - 1
            }).sum()
        }
    };
    println!("{}", res);
    Ok(())
}
