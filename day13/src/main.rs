use std::error::Error;
use aoc::{self, PuzzlePart};

// This one went well!

#[derive(Debug)]
struct MirrorField {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl From<Box<&[String]>> for MirrorField {

    fn from(value: Box<&[String]>) -> Self {
        let rows: Vec<usize> = value.iter()
            .map(|row| row.chars()
                .enumerate()
                .map(|(i, c)| {
                    let v = match c {
                        '#' => 1,
                        _ => 0,
                    };
                    return v << i;
                })
                .sum()
            ).collect();
        let cols: Vec<usize> = (0..value[0].len()).map(|i| {
            rows.iter().enumerate().map(|(j, row)| {
                let v = (row & (1 << i)) >> i;
                v << j
            }).sum()
        }).collect();
        MirrorField { rows, cols }
    }

}

fn find_mirror(input: &Vec<usize>) -> Option<usize> {
    for i in 1..input.len() {
        let mut ok = true;
        let mut diff = 1;
        loop {
            let left = input[i - diff];
            let right = input[i + diff - 1];
            if left != right {
                ok = false;
                break;
            }
            diff += 1;
            if diff > i {
                break;
            }
            if diff > input.len() - i {
                break;
            }
        }
        if ok {
            return Some(i);
        }
    }
    None
}

fn find_mirror_smudgy(input: &Vec<usize>) -> Option<usize> {
    for i in 1..input.len() {
        //println!("Line {}", i);
        let mut ok = true;
        let mut diff = 1;
        let mut used_smudge = false;
        loop {
            let left = input[i - diff];
            let right = input[i + diff - 1];
            if left != right {
                let line_diff = left ^ right;
                //println!("{:#b}", line_diff);
                if (line_diff & (line_diff - 1)) == 0 {
                    if !used_smudge {
                        used_smudge = true
                    } else {
                        ok = false;
                        break;
                    }
                } else {
                    ok = false;
                    break;
                }
            }
            diff += 1;
            if diff > i {
                break;
            }
            if diff > input.len() - i {
                break;
            }
        }
        if ok && used_smudge {
            return Some(i);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = aoc::read_input()?;
    let fields: Vec<MirrorField> = input.split(|line| line == "")
        .filter(|field| field.len() > 0)
        .map(|field| MirrorField::from(Box::new(field)))
        .collect();
    let col_sum: usize = fields.iter()
        .filter_map(|f| match aoc::puzzle_part() {
            PuzzlePart::PartOne => find_mirror(&f.cols),
            PuzzlePart::PartTwo => find_mirror_smudgy(&f.cols),
        })
        .sum();
    let row_sum: usize = fields.iter()
        .filter_map(|f| match aoc::puzzle_part() {
            PuzzlePart::PartOne => find_mirror(&f.rows),
            PuzzlePart::PartTwo => find_mirror_smudgy(&f.rows),
        })
        .sum();
    println!("{}", col_sum + 100 * row_sum);
    Ok(())
}
