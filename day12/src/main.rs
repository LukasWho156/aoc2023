use std::{error::Error, collections::HashMap};
use std::str::FromStr;
use aoc::{self, PuzzlePart, ParseLineError};

// Interesting, basically a partial Nonogram solver. Let's see what we can do.
// Neat, seems like all lines are <= 32 length, so we can do funny bitmask stuff with u64s.
// Alright, doing complete overkill here, but at least it's fun.
// Oooh no, bitmask shenanigans stop working for part 2 :( at least the simple approach
// let's see if we can figure out a way to handle that.
// okay, found a workaround, but this is gonna take a bit. Let's hope we don't run out of RAM.
// phew, that was a struggle. Took me a while to realize where I was doing unnecessary
// calculations. Finally made it, though.

#[derive(Debug, Clone)]
struct Block {
    length: usize,
    min: usize,
    max: usize,
    valid_positions: Vec<usize>,
    combinations: HashMap<usize, usize>,
}

impl Block {

    fn new(length: usize, offset: usize, clearance: usize) -> Block {
        Block {
            length,
            min: offset,
            max: offset + clearance,
            valid_positions: (offset..offset + clearance + 1).into_iter().collect(),
            combinations: HashMap::new(),
        }
    }

    fn bitmask_at_pos(&self, pos: usize, line_length: usize) -> (u128, u128) {
        let mut empty = 0;
        let mut filled = 0;
        // block
        for i in 0..self.length {
            filled += 1 << (pos + i);
        }
        // left space
        if pos > 0 {
            empty += 1 << (pos - 1);
        }
        // right space
        if pos + self.length < line_length {
            empty += 1 << (pos + self.length)
        }
        (empty, filled)
    }

    fn check_valid_positions(&mut self, line_length: usize, line_bitmask: (u128, u128)) -> bool {
        let mut found_invalid = false;
        self.valid_positions = self.valid_positions.iter().map(|p| *p).filter(|pos| {
            let my_bitmask = self.bitmask_at_pos(*pos, line_length);
            //println!("{:#b}, {:#b}", my_bitmask.0, line_bitmask.0);
            //println!("{:#b}, {:#b}", my_bitmask.1, line_bitmask.1);
            if ((!line_bitmask.0) & my_bitmask.0) > 0 || ((!line_bitmask.1) & my_bitmask.1) > 0 {
                found_invalid = true;
                false
            } else {
                true
            }
        }).collect();
        found_invalid
    }

    fn forward_prop(&mut self, amount: usize) {
        self.valid_positions = self.valid_positions.iter()
            .map(|p| *p)
            .filter(|p| p - self.min >= amount)
            .collect();
    }

    fn backward_prop(&mut self, amount: usize) {
        //println!("backward prop {:?} by {}", self, amount);
        self.valid_positions = self.valid_positions.iter()
            .map(|p| *p)
            .filter(|p| self.max - p >= amount)
            .collect();
    }

    fn remaining_positions(&self, cursor: usize) -> Vec<usize> {
        self.valid_positions.iter()
            .map(|p| *p)
            .filter(|p| *p >= cursor)
            .collect()
    }

    fn matches(&self, existing: &ExistingBlock) -> Option<Vec<usize>> {
        if existing.length > self.length {
            return None;
        }
        let mut min = existing.pos as i32 + existing.length as i32 - self.length as i32;
        if min < 0 {
            min = 0;
        }
        let min = min as usize;
        let max = existing.pos;
        let valid: Vec<usize> = self.valid_positions.iter()
            .map(|p| *p)
            .filter(|p| p >= &min && p <= &max)
            .collect();
        match valid.len() > 0 {
            true => Some(valid),
            false => None,
        }
    }

    fn adjust_first_block(&mut self, existing: &ExistingBlock) {
        let max = existing.pos;
        self.valid_positions = self.valid_positions.iter()
            .map(|p| *p)
            .filter(|p| p <= &max)
            .collect();
    }

    fn adjust_last_block(&mut self, existing: &ExistingBlock) {
        let mut min = existing.pos as i32 + existing.length as i32 - self.length as i32;
        if min < 0 {
            min = 0;
        }
        let min = min as usize;
        self.valid_positions = self.valid_positions.iter()
            .map(|p| *p)
            .filter(|p| p >= &min)
            .collect();
    }

}

#[derive(Debug)]
struct ExistingBlock {
    length: usize,
    pos: usize,
}

impl ExistingBlock {

    fn new(length: usize, pos: usize) -> ExistingBlock {
        ExistingBlock {
            length,
            pos,
        }
    }

}

#[derive(Debug)]
struct Line {
    length: usize,
    blocks: Vec<Block>,
    existing_blocks: Vec<ExistingBlock>,
    bitmask: (u128, u128),
}

fn existing_from_str(s: &str) -> Vec<ExistingBlock> {
    let mut existing_blocks: Vec<ExistingBlock> = Vec::new();
    let mut len = 0;
    let mut pos = 0;
    for (i, c) in s.chars().enumerate() {
        if len > 0 {
            if c == '#' {
                len += 1;
            } else {
                existing_blocks.push(ExistingBlock::new(len, pos));
                len = 0;
            }
        } else {
            if c == '#' {
                pos = i;
                len = 1;
            }
        }
    }
    if len > 0 {
        existing_blocks.push(ExistingBlock::new(len, pos));
    }
    existing_blocks
}

fn bitmask_from_str(s: &str) -> (u128, u128) {
    let mut empty = 0;
    let mut filled = 0;
    for (i, c) in s.chars().enumerate() {
        let bits = match c {
            '.' => (1, 0),
            '#' => (0, 1),
            '?' => (1, 1),
            _ => (0, 0),
        };
        empty |= bits.0 << i;
        filled |= bits.1 << i;
    }
    (empty, filled)
}

impl FromStr for Line {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        if split.len() != 2 {
            return Err(ParseLineError::new("Line", s));
        }
        let known = match aoc::puzzle_part() {
            PuzzlePart::PartOne => String::from(split[0]),
            //PuzzlePart::PartTwo => format!("{}?{}?{}", split[0], split[0], split[0]),
            PuzzlePart::PartTwo => format!("{}?{}?{}?{}?{}", split[0], split[0], split[0], split[0], split[0]),
        };
        let line_length = known.len();
        let existing_blocks = existing_from_str(known.as_str());
        //println!("{:?}", existing_blocks);
        let bitmask = bitmask_from_str(known.as_str());
        //println!("{:#b}", bitmask);
        let blocks = match aoc::puzzle_part() {
            PuzzlePart::PartOne => String::from(split[1]),
            //PuzzlePart::PartTwo => format!("{},{},{}", split[1], split[1], split[1]),
            PuzzlePart::PartTwo => format!("{},{},{},{},{}", split[1], split[1], split[1], split[1], split[1]),
        };
        let block_lengths: Vec<usize> = blocks.split(",")
            .filter_map(|n| match n.parse() {
                Ok(i) => Some(i),
                Err(_) => None,
            })
            .collect();
        let total_block_length = block_lengths.iter().sum::<usize>() + block_lengths.len() - 1;
        let clearance = line_length - total_block_length;
        let mut offset = 0;
        let blocks: Vec<Block> = block_lengths.iter().map(|len| {
            let block = Block::new(*len, offset, clearance);
            offset += len + 1;
            block
        }).collect();
        Ok(Line {
            length: line_length,
            blocks,
            existing_blocks,
            bitmask
        })
    }
}

impl Line {

    fn propagate_changes(&mut self, changes: &Vec<usize>) {
        for &i in changes {
            //println!("-----");
            let b = self.blocks[i].clone();
            //println!("{:?}", b);
            let forward_prop = b.valid_positions.iter().min().unwrap() - b.min;
            let backward_prop = b.max - b.valid_positions.iter().max().unwrap();
            if forward_prop > 1 {
                //println!("forward prop by {}", forward_prop);
                for j in i + 1..self.blocks.len() {
                    self.blocks[j].forward_prop(forward_prop);
                }
            }
            if backward_prop > 1 && i > 0 {
                //println!("backward prop by {}", backward_prop);
                for j in 0..i {
                    self.blocks[j].backward_prop(backward_prop);
                }
            }
        }
    }
    
    fn step(&mut self) {
        let mut changes: Vec<usize> = Vec::new();
        //println!("=====");
        self.blocks.iter_mut().enumerate().for_each(|(i, b)| {
            //println!("{:?}", b);
            if b.check_valid_positions(self.length, self.bitmask) {
                //println!("=> {:?}", b);
                changes.push(i);
            }
        });
        let done = changes.len() == 0;
        self.propagate_changes(&changes);
        changes.clear();
        self.existing_blocks.iter().for_each(|existing| {
            let matches: Vec<(usize, Vec<usize>)> = self.blocks.iter_mut()
                .enumerate()
                .filter_map(|(i, b)| {
                    match b.matches(&existing) {
                        Some(p) => Some((i, p)),
                        None => None,
                    }
                }).collect();
            if matches.len() == 1 {
                let i = matches[0].0;
                changes.push(i);
                let b = &mut self.blocks[i];
                //println!("Found unique block: {:?}, {:?}", existing, b);
                b.valid_positions = matches[0].1.clone();
            }
        });
        self.propagate_changes(&changes);
        if !done {
            self.step();
        }
    }

    fn check_validity(&self, pos: &Vec<usize>) -> bool {
        //println!("{:?}", pos);
        let mut filled = 0;
        for i in 0..pos.len() {
            let b = &self.blocks[i];
            let p = pos[i];
            let bm = b.bitmask_at_pos(p, self.length);
            filled |= bm.1;
        }
        let empty = !filled & ((1 << self.length) - 1);
        //solution_bitmask = !0 & solution_bitmask;
        //println!("{:#b} / {:#b}", filled, self.bitmask.1);
        //println!("{:#b} / {:#b}", empty, self.bitmask.0);
        ((!self.bitmask.1) & filled) == 0 && ((!self.bitmask.0) & empty) == 0
    }

    fn calculate_positions(&mut self) {
        let mut i = self.blocks.len() - 1;
        loop {
            let next = self.blocks.get(i + 1);
            let current = &self.blocks[i];
            let mut combinations: HashMap<usize, usize> = HashMap::new();
            for p in &current.valid_positions {
                let cursor = p + current.length + 1;
                let next_existing = self.existing_blocks.iter().find(|e| e.pos >= cursor);
                let c = match next {
                    Some(n) => {
                        n.valid_positions.iter()
                            .filter(|p| *p >= &cursor)
                            .filter(|p| match next_existing {
                                Some(next) => *p <= &next.pos,
                                None => true,
                            })
                            .filter_map(|p| n.combinations.get(p))
                            .sum()
                    },
                    None => 1,
                };
                combinations.insert(*p, c);
            }
            self.blocks[i].combinations = combinations;
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

}

fn plausible_positions(line: &Line, blocks: &Vec<Block>, current: usize, cursor: usize) -> usize {
    let current_block = &blocks[current];
    let next_existing = line.existing_blocks.iter().find(|e| e.pos >= cursor);
    match current < blocks.len() - 1 {
        true => {
            current_block.remaining_positions(cursor).iter().filter(|p| {
                match next_existing {
                    Some(ex) => **p <= ex.pos,
                    None => true,
                }
            }).map(|p| {
                plausible_positions(line, blocks, current + 1, p + current_block.length + 1)
            }).sum()
        }
        false => current_block.remaining_positions(cursor).iter().filter(|p| {
            let last_pos = match std::panic::catch_unwind(|| { 
                let last = &line.existing_blocks[line.existing_blocks.len() - 1];
                last.pos + last.length - current_block.length
            }) {
                Ok(v) => v,
                Err(_) => 0
            };
            match next_existing {
                Some(ex) => **p <= ex.pos && **p >= last_pos,
                None => true,
            }
        }).count(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //std::panic::set_hook(Box::new(|_info| {
    //    // do nothing
    //}));
    let input = aoc::read_input()?;
    let mut lines: Vec<Line> = input.iter()
        .filter_map(|s| match s.parse() {
            Ok(line) => Some(line),
            Err(_) => None,
        })
        .collect();
    lines.iter_mut().for_each(|line| {
        //println!("********");
        if line.existing_blocks.len() > 0 {
            let i = line.blocks.len()  - 1;
            let j = line.existing_blocks.len() - 1;
            line.blocks[i].adjust_last_block(&line.existing_blocks[j]);
            line.blocks[0].adjust_first_block(&line.existing_blocks[0]);
        }
        line.step();
        //let filtered: Vec<&Block> = line.blocks.iter().filter(|b| b.valid_positions.len() == 1).collect();
        //println!("{:?}", filtered);
    });
    let len = lines.len();
    let res: usize = lines.iter_mut().enumerate().map(|(i, line)| {
        //let r = plausible_positions(&line, &line.blocks, 0, 0);
        line.calculate_positions();
        let first = &line.blocks[0];
        let res: usize = first.valid_positions.iter()
            .filter_map(|p| first.combinations.get(p))
            .sum();
        //let r2 = plausible_positions(line, &line.blocks, 0, 0);
        //if res != r2 {
        //    println!("Deviation in line {}! A: {} combinations were valid. B: {} were valid.", i + 1, r2, res);
        //}
        println!("Solved {} / {} lines; {} combinations were valid.", i + 1, len, res);
        res
    }).sum();
    println!("{}", res);
    Ok(())
}
