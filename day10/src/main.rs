use std::{collections::HashMap, f32::consts::E};
use aoc::{self, PuzzlePart};
use std::error::Error;

use aoc::pipes::{self, Pipe, Partition};

// this one almost broke me

fn part_1(pipes: &mut Vec<Vec<Pipe>>, start: (usize, usize)) {
    let mut heads = vec![start];
    let mut distance = 0;
    while heads.len() > 0 {
        //println!("{:?}", heads);
        let mut new_heads: Vec<(usize, usize)> = vec![];
        heads.iter().for_each(|h| {
            {
                let p = &mut pipes[h.1][h.0];
                p.distance = Some(distance);
            }
            let p = &pipes[h.1][h.0];
            if p.north() {
                let hn = (h.0, h.1 - 1);
                let p2 = &pipes[hn.1][hn.0];
                if p2.south() && p2.distance.is_none() {
                    new_heads.push(hn);
                }
            }
            if p.east() {
                let hn = (h.0 + 1, h.1);
                let p2 = &pipes[hn.1][hn.0];
                if p2.west() && p2.distance.is_none() {
                    new_heads.push(hn);
                }
            }
            if p.south() {
                let hn = (h.0, h.1 + 1);
                let p2 = &pipes[hn.1][hn.0];
                if p2.north() && p2.distance.is_none() {
                    new_heads.push(hn);
                }
            }
            if p.west() {
                let hn = (h.0 - 1, h.1);
                let p2 = &pipes[hn.1][hn.0];
                if p2.east() && p2.distance.is_none() {
                    new_heads.push(hn);
                }
            }
        });
        distance += 1;
        heads = new_heads;
        //std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{}", distance - 1);
}

fn part_2(pipes: &mut Vec<Vec<Pipe>>, start: (usize, usize)) {
    let mut dir: (i32, i32) = (0, 0);
    let mut connections = 0;
    // silly bit of code to find an initial starting direction.
    if pipes[start.1 - 1][start.0].south() {
        dir = (0, 1);
        connections |= 0b1000;
    }
    if pipes[start.1][start.0 + 1].west() {
        dir = (-1, 0);
        connections |= 0b0100;
    }
    if pipes[start.1 + 1][start.0].north() {
        dir = (0, -1);
        connections |= 0b0010;
    }
    if pipes[start.1][start.0 - 1].east() {
        dir = (1, 0);
        connections |= 0b0001;
    }
    {
        pipes[start.1][start.0].connections = connections;
    }
    println!("{:?}", pipes[start.1][start.0]);
    aoc::pipes::follow_trail(pipes, start, dir)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    //   NESW
    // 0b????
    let mut key: HashMap<char, u8> = HashMap::new();
    key.insert('.', 0b0000);
    key.insert('-', 0b0101);
    key.insert('|', 0b1010);
    key.insert('L', 0b1100);
    key.insert('F', 0b0110);
    key.insert('7', 0b0011);
    key.insert('J', 0b1001);
    key.insert('S', 0b1111);
    let mut start = (0, 0);
    let mut pipes: Vec<Vec<Pipe>> = input.iter().enumerate().filter_map(|(y, line)| {
        let res: Vec<Pipe> = line.replace("\r", "").chars().enumerate()
            .filter_map(|(x, c)| {
                if c == 'S' {
                    start = (x, y);
                }
                key.get(&c)
            })
            .map(|c| Pipe { connections: *c, distance: None, partition: None })
            .collect();
        match res.len() {
            0 => None,
            _ => Some(res)
        }
    }).collect();
    //println!("{:?}", pipes);
    match aoc::puzzle_part() {
        PuzzlePart::PartOne => part_1(&mut pipes, start),
        PuzzlePart::PartTwo => part_2(&mut pipes, start),
    }
    Ok(())
}
