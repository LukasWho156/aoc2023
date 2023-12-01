use std::fs;
use std::env;
use std::error::Error;

pub fn read_input() -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string("./input.txt")?;
    Ok(file.split("\n").map(|e| e.to_string()).collect())
}

pub enum PuzzlePart {
    PartOne,
    PartTwo,
}

pub fn puzzle_part() -> PuzzlePart {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("part2")) {
        PuzzlePart::PartTwo
    } else {
        PuzzlePart::PartOne
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!();
    }
}
