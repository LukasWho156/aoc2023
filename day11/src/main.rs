use aoc::{self, PuzzlePart};
use std::error::Error;

// seriously? Part 2 is basically "can you use 64 bit integers instead of 32 bit integers"?
// honestly, pretty boring.

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut stars: Vec<(usize, usize)> = Vec::new();
    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                stars.push((x, y));
            }
        });
    });
    let expansion = match aoc::puzzle_part() {
        PuzzlePart::PartOne => 1,
        PuzzlePart::PartTwo => 999999,
    };
    let empty_columns: Vec<usize> = (0..input.len())
        .into_iter()
        .filter(|col| stars.iter().find(|s| s.1 == *col).is_none())
        .collect();
    let empty_rows: Vec<usize> = (0..input[0].len())
        .into_iter()
        .filter(|row| stars.iter().find(|s| s.0 == *row).is_none())
        .collect();
    empty_columns.iter().rev().for_each(|col| {
        stars.iter_mut()
            .filter(|s| s.1 > *col)
            .for_each(|s| s.1 += expansion);
    });
    empty_rows.iter().rev().for_each(|row| {
        stars.iter_mut()
            .filter(|s| s.0 > *row)
            .for_each(|s| s.0 += expansion);
    });
    println!("Stars: {:?}", stars);
    let mut sum: i64 = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            sum += (stars[j].0 as i64 - stars[i].0 as i64).abs() + (stars[j].1 as i64 - stars[i].1 as i64).abs();
        }
    }
    println!("{}", sum);
    Ok(())
}
