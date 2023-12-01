use std::error::Error;
use aoc::{self, PuzzlePart};

fn replace_written_digits(input: &String) -> String {
    // How mean! things like twone can exist. Hacky solution: keep the original words in both places.
    input.as_str()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .to_string()
}

fn parse(input: &String) -> Option<i32> {
    let binding = &replace_written_digits(input);
    let input = match aoc::puzzle_part() {
        PuzzlePart::PartOne => input,
        PuzzlePart::PartTwo => binding,
    };
    let digits: Vec<u8> = input.as_bytes().iter()
        .filter(|e| e >= &&0x30 && e <= &&0x39)
        .map(|e| *e - 0x30)
        .collect();
    if digits.len() < 1 {
        return None
    }
    Some((digits[0] * 10 + digits[digits.len() - 1]) as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let res: i32 = input.iter()
        .filter_map(parse)
        .sum();
    println!("{:?}", res);
    Ok(())
}
