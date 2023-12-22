// Part 1 seems pretty doable, which terrifies me of part 2.

// okay, part 2 seemed a little scary at first, but the input is
// incredibly nice, which means we only have to look at a couple of
// variations:

//     A1B
//    AIYJB
//   AIYXYJA
//   2YXYXY3
//   CKYXYLC
//    CKYLD
//     C4D

// X and Y are completely filled out and we just need to count the
// odd / even tiles within. 1, 2, 3 and 4 start from the center bottom /
// right / left / top respectively and have 130 steps left.
// A, B, C and D start from their respective corners and have 64 steps
// left. I, J, K and L start from their respective corners and have 195
// steps left. for 1-4 and I-L, we need to consider odd tiles, even for A-D.

// 26_501_365 = 202_300 * 131 + 65, which means there are 202_229 - 4 X tiles,
// 202_300^2 Y tiles 202_300 A, B, C and D tiles each and 202_299 I-L tiles each.

// okay, with that out of the way, let's start coding!

// (side node: It's kinda funny that the example map does not obviously exhibit
// the properties that make the actual map so approachable.)

// oh wait, there are unreachable spots, so we do have to count after all.

// took a while to get the math completely right, but got there in the end.


use std::error::Error;
use aoc::{self, PuzzlePart};

#[derive(Clone)]
struct Tile {
    is_even: bool,
    walkable: bool,
    visited: bool,
}

fn get_reachable(map: &mut Vec<Vec<Tile>>, start: (usize, usize), steps: usize) -> usize {
    let start_parity = (start.0 + start.1 + steps) % 2 == 0;
    let width = map[0].len();
    let height = map.len();
    let mut heads = vec![start];
    let mut possible_heads: Vec<(usize, usize)> = Vec::new();
    for _i in 0..steps {
        while let Some((x, y)) = heads.pop() {
            if x > 0 {
                possible_heads.push((x - 1, y));
            }
            if x < width - 1 {
                possible_heads.push((x + 1, y));
            }
            if y > 0 {
                possible_heads.push((x, y - 1));
            }
            if y < height - 1 {
                possible_heads.push((x, y + 1));
            }
        }
        while let Some((x, y)) = possible_heads.pop() {
            let tile = &mut map[y][x];
            if tile.visited || !tile.walkable {
                continue;
            }
            tile.visited = true;
            heads.push((x, y));
        }
    }
    map.iter().map(|line| {
        line.iter().filter(|tile| {
            tile.visited && tile.is_even == start_parity
        }).count()
    }).sum()
}

fn reset_map(map: &mut Vec<Vec<Tile>>) {
    map.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|tile| tile.visited = false);
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut start_parity: bool = false;
    let mut start: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<Tile>> = input.iter().filter(|line| line.len() > 0).enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            let walkable = c != '#';
            let visited = c == 'S';
            let is_even = (x + y) % 2 == 0;
            if c == 'S' {
                start_parity = is_even;
                start = (x, y);
            }
            Tile {
                is_even,
                walkable,
                visited,
            }
        }).collect()
    }).collect();
    let res = match aoc::puzzle_part() {
        PuzzlePart::PartOne => get_reachable(&mut map, start, 64),
        PuzzlePart::PartTwo => {
            let full_squares = 202_300;
            let amount_x = (full_squares - 1) * (full_squares - 1) * get_reachable(&mut map, (65, 65), 999);
            reset_map(&mut map);
            let amount_y = full_squares * full_squares * get_reachable(&mut map, (65, 65), 1000);
            reset_map(&mut map);
            let amount_a = full_squares * get_reachable(&mut map, (130, 130), 64);
            reset_map(&mut map);
            let amount_b = full_squares * get_reachable(&mut map, (0, 130), 64);
            reset_map(&mut map);
            let amount_c = full_squares * get_reachable(&mut map, (130, 0), 64);
            reset_map(&mut map);
            let amount_d = full_squares * get_reachable(&mut map, (0, 0), 64);
            reset_map(&mut map);
            let amount_i = (full_squares - 1) * get_reachable(&mut map, (130, 130), 195);
            reset_map(&mut map);
            let amount_j = (full_squares - 1) * get_reachable(&mut map, (0, 130), 195);
            reset_map(&mut map);
            let amount_k = (full_squares - 1) * get_reachable(&mut map, (130, 0), 195);
            reset_map(&mut map);
            let amount_l = (full_squares - 1) * get_reachable(&mut map, (0, 0), 195);
            reset_map(&mut map);
            let amount_1 = get_reachable(&mut map, (65, 130), 130);
            reset_map(&mut map);
            let amount_2 = get_reachable(&mut map, (130, 65), 130);
            reset_map(&mut map);
            let amount_3 = get_reachable(&mut map, (0, 65), 130);
            reset_map(&mut map);
            let amount_4 = get_reachable(&mut map, (65, 0), 130);
            amount_x + amount_y
                + amount_a + amount_b + amount_c + amount_d
                + amount_i + amount_j + amount_k + amount_l
                + amount_1 + amount_2 + amount_3 + amount_4
        },
    };
    println!("{}", res);
    Ok(())
}
