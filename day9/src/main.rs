// okay, *today's* puzzle was definitely the one with the smallest change from
// part 1 to part 2. Like, seriously? What was that?

use std::error::Error;
use aoc::{self, PuzzlePart};

fn predict(input: &Vec<i32>) -> i32 {
    match input.iter().find(|e| **e != 0) {
        Some(_) => (predict(&input.iter()
            .enumerate()
            .filter_map(|(i, e)| match i == 0 {
                true => None,
                false => Some(*e - input[i - 1]),
            })
            .collect()) + match aoc::puzzle_part() {
                PuzzlePart::PartOne => *input.last().unwrap(),
                PuzzlePart::PartTwo => -input.get(0).unwrap(),
            }) * match aoc::puzzle_part() {
                PuzzlePart::PartOne => 1,
                PuzzlePart::PartTwo => -1,
            },
        None => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let seqs: Vec<Vec<i32>> = input.iter().filter_map(|line| {
        let seq: Vec<i32> = line.split(" ").filter_map(|n| match n.parse() {
            Ok(r) => Some(r),
            Err(_) => None,
        }).collect();
        match seq.len() {
            0 => None,
            _ => Some(seq),
        }
    }).collect();
    let res: i32 = seqs.iter().map(predict).sum();
    println!("{}", res);
    Ok(())
}
