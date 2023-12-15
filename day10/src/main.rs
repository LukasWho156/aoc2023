use std::{collections::HashMap, f32::consts::E};
use aoc::{self, PuzzlePart};
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Partition {
    Loop,
    Left,
    Right,
}

#[derive(Debug)]
struct Pipe {
    connections: u8,
    distance: Option<i32>,
    partition: Option<Partition>,
}

impl Pipe {

    fn north(&self) -> bool {
        (&self.connections & 0b1000) > 0
    }

    fn east(&self) -> bool {
        (&self.connections & 0b0100) > 0
    }

    fn south(&self) -> bool {
        (&self.connections & 0b0010) > 0
    }

    fn west(&self) -> bool {
        (&self.connections & 0b0001) > 0
    }

    fn out_dir(&self, in_dir: (i32, i32)) -> (i32, i32) {
        let h = match in_dir.0 {
            -1 => 0b0100,
            1 => 0b0001,
            _ => 0,
        };
        let v = match in_dir.1 {
            -1 => 0b0010,
            1 => 0b1000,
            _ => 0,
        };
        let in_con = h | v;
        let out_con = self.connections ^ in_con;
        match out_con {
            0b1000 => (0, -1),
            0b0100 => (1, 0),
            0b0010 => (0, 1),
            0b0001 => (-1, 0),
            _ => (0, 0),
        }
    }

    fn adjacencies(&self, in_dir: (i32, i32)) -> Vec<Partition> {
        let h = match in_dir.0 {
            -1 => 0b0100,
            1 => 0b0001,
            _ => 0,
        };
        let v = match in_dir.1 {
            -1 => 0b0010,
            1 => 0b1000,
            _ => 0,
        };
        let in_con = h | v;
        let out_con = self.connections ^ in_con;
        vec![0b1000, 0b0100, 0b0010, 0b0001].iter().map(|c| *c)
            .map(|c| {
                if c == in_con || c == out_con {
                    return Partition::Loop;
                }
                if in_con > out_con {
                    if c > in_con {
                        return Partition::Right;
                    }
                    if c < out_con {
                        return Partition::Right;
                    }
                    return Partition::Left;
                }
                if c > out_con {
                    return Partition::Left;
                }
                if c < in_con {
                    return Partition::Left;
                }
                return Partition::Right;
            })
            .collect()
    }

}

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
    let mut head = start;
    let mut dir: (i32, i32) = (0, 0);
    let mut connections = 0;
    // silly bit of code to find an initial starting direction.
    if pipes[start.1 - 1][start.0].south() {
        dir = (0, 1);
        connections &= 0b1000;
    }
    if pipes[start.1][start.0 + 1].west() {
        dir = (-1, 0);
        connections &= 0b0100;
    }
    if pipes[start.1 + 1][start.0].north() {
        dir = (0, -1);
        connections &= 0b0010;
    }
    if pipes[start.1][start.0 - 1].east() {
        dir = (1, 0);
        connections &= 0b0001;
    }
    {
        pipes[start.1][start.0].connections = connections;
    }
    let mut distance = 0;
    // follow the loop and mark stuff as either belonging to the loop, being on its left or on its right
    loop {
        {
            let p = &mut pipes[head.1][head.0];
            p.partition = Some(Partition::Loop);
        }
        let p = &pipes[head.1][head.0];
        let adjacencies = p.adjacencies(dir);
        // println!("{:?}", adjacencies);
        dir = p.out_dir(dir);
        if head.1 > 0 {
            let p_n = &mut pipes[head.1 - 1][head.0];
            if p_n.partition.is_none() {
                p_n.partition = Some(adjacencies[0]);
            }
        }
        if head.0 < pipes[0].len() - 1 {
            let p_n = &mut pipes[head.1][head.0 + 1];
            if p_n.partition.is_none() {
                p_n.partition = Some(adjacencies[1]);
            }
        }
        if head.1 < pipes.len() - 1 {
            let p_n = &mut pipes[head.1 + 1][head.0];
            if p_n.partition.is_none() {
                p_n.partition = Some(adjacencies[2]);
            }
        }
        if head.0 > 0 {
            let p_n = &mut pipes[head.1][head.0 - 1];
            if p_n.partition.is_none() {
                p_n.partition = Some(adjacencies[3]);
            }
        }
        head.0 = (head.0 as i32 + dir.0) as usize;
        head.1 = (head.1 as i32 + dir.1) as usize;
        distance += 1;
        if head.0 == start.0 && head.1 == start.1 {
            break;
        }
    }
    println!("Full loop: {}", distance);
    // turn this into strings for easier flood fills
    let partition_map = pipes.iter().map(|line| {
        line.iter().map(|pipe| {
            match pipe.partition {
                None => "?",
                Some(t) => match t {
                    Partition::Loop => "!",
                    Partition::Left => "L",
                    Partition::Right => "R",
                }
            }
        }).fold(String::from(""), |acc, cur| acc + cur)
    });
    // silly "flood fill"
    let partition_map: Vec<String> = partition_map.map(|mut line| {
        while line.contains("R?") {
            line = line.replace("R?", "RR");
        }
        while line.contains("?R") {
            line = line.replace("?R", "RR");
        }
        while line.contains("L?") {
            line = line.replace("L?", "LL");
        }
        while line.contains("?L") {
            line = line.replace("?L", "LL");
        }
        println!("{}", line); // I'll leave this in because it looks pretty.
        line
    }).collect();
    // now all that's left to do is count the Rs
    let rights: usize = partition_map.iter().map(|line| {
        line.chars().filter(|c| *c == 'R').count()
    }).sum();
    let lefts: usize = partition_map.iter().map(|line| {
        line.chars().filter(|c| *c == 'L').count()
    }).sum();
    println!("Lefts: {}; Rights: {}", lefts, rights);
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
