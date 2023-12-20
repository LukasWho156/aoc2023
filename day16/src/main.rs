use std::error::Error;
use aoc::{self, PuzzlePart};

// another pretty simple one. guess this could've been worse if loop detection
// had been worse?

struct Tile {
    energized_mask: u8,
    mapping_fn: fn(u8) -> u8,
}

fn passthrough(input: u8) -> u8 {
    input
}

fn mirror_1(input: u8) -> u8 {
    match input {
        0b1000 => 0b0100,
        0b0100 => 0b1000,
        0b0010 => 0b0001,
        0b0001 => 0b0010,
        _ => 0
    }
}

fn mirror_2(input: u8) -> u8 {
    match input {
        0b1000 => 0b0001,
        0b0100 => 0b0010,
        0b0010 => 0b0100,
        0b0001 => 0b1000,
        _ => 0
    }
}

fn split_h(input: u8) -> u8 {
    match input {
        0b1000 => 0b0101,
        0b0100 => 0b0100,
        0b0010 => 0b0101,
        0b0001 => 0b0001,
        _ => 0
    }
}

fn split_v(input: u8) -> u8 {
    match input {
        0b1000 => 0b1000,
        0b0100 => 0b1010,
        0b0010 => 0b0010,
        0b0001 => 0b1010,
        _ => 0
    }
}

impl Tile {

    fn process_light(&mut self, dir: u8) -> Vec<(i32, i32, u8)> {
        let mut res = Vec::new();
        if self.energized_mask & dir > 0 {
            return res;
        }
        self.energized_mask |= dir;
        let map = (self.mapping_fn)(dir);
        if map & 0b1000 > 0 {
            res.push((0, -1, 0b1000));
        }
        if map & 0b0100 > 0 {
            res.push((1, 0, 0b0100));
        }
        if map & 0b0010 > 0 {
            res.push((0, 1, 0b0010));
        }
        if map & 0b0001 > 0 {
            res.push((-1, 0, 0b0001));
        }
        res
    }

    fn new(mapping_fn: fn(u8) -> u8) -> Tile {
        Tile {
            energized_mask: 0,
            mapping_fn
        }
    }

}

fn print_energized(tile_map: &Vec<Vec<Tile>>) {
    tile_map.iter().for_each(|line| {
        line.iter().for_each(|tile| {
            if tile.energized_mask > 0 {
                print!("#");
            } else {
                print!(".");
            }
        });
        print!("\n");
    });
}

fn calculate_energized(start: (i32, i32, u8), tile_map: &mut Vec<Vec<Tile>>) -> usize {
    let width = tile_map[0].len();
    let height = tile_map.len();
    let mut heads: Vec<(i32, i32, u8)> = vec![start];
    while heads.len() > 0 {
        let mut new_heads = Vec::new();
        for h in heads {
            let tile = &mut tile_map[h.1 as usize][h.0 as usize];
            tile.process_light(h.2).iter_mut().for_each(|nh| {
                nh.0 += h.0;
                nh.1 += h.1;
                if nh.0 < 0 || nh.1 < 0 || nh.0 >= width as i32 || nh.1 >= height as i32 {
                    return;
                }
                new_heads.push(*nh);
            });
        }
        heads = new_heads;
    }
    tile_map.iter().map(|line| {
        line.iter().filter(|tile| tile.energized_mask > 0).count()
    }).sum()
}

fn clear_map(tile_map: &mut Vec<Vec<Tile>>) {
    tile_map.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|tile| tile.energized_mask = 0);
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut tile_map: Vec<Vec<Tile>> = input.iter().filter(|line| line.len() > 0).map(|line| {
        line.chars().map(|c| {
            match c {
                '/' => Tile::new(mirror_1),
                '\\' => Tile::new(mirror_2),
                '|' => Tile::new(split_v),
                '-' => Tile::new(split_h),
                _ => Tile::new(passthrough),
            }
        }).collect::<Vec<Tile>>()
    }).collect();
    let width = tile_map[0].len();
    let height = tile_map.len();
    let res = match aoc::puzzle_part() {
        PuzzlePart::PartOne => calculate_energized((0, 0, 0b0100), &mut tile_map),
        PuzzlePart::PartTwo => {
            let mut best = 0;
            for x in 0..width {
                best = best.max(calculate_energized((x as i32, 0, 0b0010), &mut tile_map));
                clear_map(&mut tile_map);
                best = best.max(calculate_energized((x as i32, height as i32 - 1, 0b1000), &mut tile_map));
                clear_map(&mut tile_map);
            }
            for y in 0..height {
                best = best.max(calculate_energized((0, y as i32, 0b0100), &mut tile_map));
                clear_map(&mut tile_map);
                best = best.max(calculate_energized((width as i32 - 1, y as i32, 0b0001), &mut tile_map));
                clear_map(&mut tile_map);
            }
            best
        }
    };
    println!("{}", res);
    Ok(())
}