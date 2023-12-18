use std::{error::Error, collections::HashMap};
use aoc::{self, PuzzlePart};

struct Plate {
    width: usize,
    height: usize,
    stationary: Vec<(usize, usize)>,
    movable: Vec<(usize, usize)>,
}

impl From<&[String]> for Plate {
    fn from(value: &[String]) -> Self {
        let width = value[0].len();
        let height = value.len();
        let mut stationary: Vec<(usize, usize)> = Vec::new();
        let mut movable: Vec<(usize, usize)> = Vec::new();
        value.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let p = (x, y);
                match c {
                    '#' => stationary.push(p),
                    'O' => movable.push(p),
                    _ => (),
                }
            });
        });
        Plate { width, height, stationary, movable }
    }
}

impl Plate {

    fn relax(&mut self, dir: (i32, i32)) {
        let mut existing: HashMap<(usize, usize), ()> = HashMap::new();
        self.movable.iter().for_each(|p| {
            existing.insert(*p, ());
        });
        for (x, y) in existing.keys() {
            self.movable.iter_mut()
                .filter(|(px, py)| px == x && py == y)
                .enumerate()
                .for_each(|(i, p)| {
                    p.0 = (p.0 as i32 + dir.0 * i as i32) as usize;
                    p.1 = (p.1 as i32 + dir.1 * i as i32) as usize;
                });
        }
    }

    fn move_up(&mut self) {
        self.movable.iter_mut().for_each(|p| {
            p.1 = match self.stationary.iter().rev().find(|s| s.0 == p.0 && s.1 < p.1) {
                Some(stat) => stat.1 + 1,
                None => 0,
            }
        });
        self.relax((0, 1));
    }

    fn move_left(&mut self) {
        self.movable.iter_mut().for_each(|p| {
            p.0 = match self.stationary.iter().rev().find(|s| s.0 < p.0 && s.1 == p.1) {
                Some(stat) => stat.0 + 1,
                None => 0,
            }
        });
        self.relax((1, 0));
    }

    fn move_down(&mut self) {
        self.movable.iter_mut().for_each(|p| {
            p.1 = match self.stationary.iter().find(|s| s.0 == p.0 && s.1 > p.1) {
                Some(stat) => stat.1 - 1,
                None => self.height - 1,
            }
        });
        self.relax((0, -1));
    }

    fn move_right(&mut self) {
        self.movable.iter_mut().for_each(|p| {
            p.0 = match self.stationary.iter().find(|s| s.0 > p.0 && s.1 == p.1) {
                Some(stat) => stat.0 - 1,
                None => self.width - 1,
            }
        });
        self.relax((-1, 0));
    }

    fn print_state(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut c = '.';
                if self.stationary.iter().find(|(px, py)| px == &x && py == &y).is_some() {
                    c = '#';
                }
                if self.movable.iter().find(|(px, py)| px == &x && py == &y).is_some() {
                    c = 'O';
                }
                print!("{}", c);
            }
            print!("\n");
        }
    }

    fn cycle(&mut self) {
        let mut cycle = 0;
        let mut seen_configs: HashMap<Vec<u128>, usize> = HashMap::new();
        'outer: while cycle < 1_000_000_000 {
            //let current_rocks = self.movable.clone();
            self.move_up();
            //self.print_state();
            //println!("----");
            self.move_left();
            //self.print_state();
            //println!("----");
            self.move_down();
            //self.print_state();
            //println!("----");
            self.move_right();
            //self.print_state();
            println!("----");
            let config: Vec<u128> = (0..self.height).map(|y| {
                let mut mask = 0;
                for x in 0..self.width {
                    if self.movable.iter().find(|(cx, cy)| cx == &x && cy == &y).is_some() {
                        mask += 1 << x;
                    }
                }
                mask
            }).collect();
            //println!("{:?}", config);
            cycle += 1;
            let exists = seen_configs.insert(config, cycle);
            //println!("{:?}", exists);
            if let Some(c) = exists {
                println!("Meta-Cycle detected: Cycle {} is equal to cycle {}", cycle, c);
                let cycle_length = cycle - c;
                let offset = c;
                println!("Meta-Cycle-Length: {}, Offset: {}", cycle_length, offset);
                let remainder = (1_000_000_000 - offset) % cycle_length;
                println!("Cycle 1.000.000.000 is equal to cycle {}", offset + remainder);
                //continue 'outer;
            } else {
                println!("Cycle {} finished with weight {}", cycle, self.northern_load());
                continue 'outer;
            }
            break;
        }
        //println!("Done!");
    }

    fn northern_load(&self) -> usize {
        self.movable.iter().map(|(_, y)| {
            self.height - y
        }).sum()
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut plates: Vec<Plate> = input.split(|line| line == "")
        .filter(|p| p.len() > 0)
        .map(|p| Plate::from(p))
        .collect();
    println!("{}, {}, {}", plates[0].width, plates[0].height, plates[0].movable.len());
    match aoc::puzzle_part() {
        PuzzlePart::PartOne => plates.iter_mut().for_each(|p| p.move_up()),
        PuzzlePart::PartTwo => plates.iter_mut().for_each(|p| p.cycle()),
    }
    //println!("{:?}", plates[0].movable);
    let res: usize = plates.iter()
        .map(|p| p.northern_load())
        .sum();
    println!("{}", res);
    Ok(())
}
