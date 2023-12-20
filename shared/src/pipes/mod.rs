#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Partition {
    Loop,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Pipe {
    pub connections: u8,
    pub distance: Option<i32>,
    pub partition: Option<Partition>,
}

impl Pipe {

    pub fn north(&self) -> bool {
        (&self.connections & 0b1000) > 0
    }

    pub fn east(&self) -> bool {
        (&self.connections & 0b0100) > 0
    }

    pub fn south(&self) -> bool {
        (&self.connections & 0b0010) > 0
    }

    pub fn west(&self) -> bool {
        (&self.connections & 0b0001) > 0
    }

    pub fn out_dir(&self, in_dir: (i32, i32)) -> (i32, i32) {
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
        let out_con = self.connections & (!in_con);
        match out_con {
            0b1000 => (0, -1),
            0b0100 => (1, 0),
            0b0010 => (0, 1),
            0b0001 => (-1, 0),
            _ => (0, 0),
        }
    }

    pub fn adjacencies(&self, in_dir: (i32, i32)) -> Vec<Partition> {
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
        let out_con = self.connections & (!in_con);
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

pub fn follow_trail(pipes: &mut Vec<Vec<Pipe>>, start: (usize, usize), start_dir: (i32, i32)) {
    let mut head = start;
    let mut distance = 0;
    let mut dir: (i32, i32) = start_dir;
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
        //println!("{}", line); // I'll leave this in because it looks pretty.
        line
    }).collect();
    // now all that's left to do is count the Rs
    let rights: usize = partition_map.iter().map(|line| {
        line.chars().filter(|c| *c == 'R').count()
    }).sum();
    let lefts: usize = partition_map.iter().map(|line| {
        line.chars().filter(|c| *c == 'L').count()
    }).sum();
    let loops: usize = partition_map.iter().map(|line| {
        line.chars().filter(|c| *c == '!').count()
    }).sum();
    println!("Lefts: {}; Rights: {}, Loop: {}", lefts, rights, loops);
}