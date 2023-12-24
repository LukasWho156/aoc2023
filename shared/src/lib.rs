pub mod astar;
pub mod pipes;

use std::fs;
use std::env;
use std::fmt;
use std::error::Error;

pub fn read_input() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = if args.contains(&String::from("test")) {
        "./test_input.txt"
    } else {
        "./input.txt"
    };
    let file = fs::read_to_string(file)?;
    let file = file.replace("\r", "");
    Ok(file.split("\n").map(|e| e.to_string()).collect())
}

// could've probably made this method a lot earlier than day23,
// but better late than never, right?
pub fn read_map<T: From<char>>() -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    let input = read_input()?;
    Ok(input.iter().filter(|line| line.len() > 0).map(|line| {
        line.chars().map(|c| T::from(c)).collect()
    }).collect())
}

#[derive(Eq, PartialEq)]
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

#[derive(Debug)]
pub struct ParseLineError {
    line: String,
    target: String,
}

impl ParseLineError {
    pub fn new(target: &str, line: &str) -> ParseLineError {
        ParseLineError {
            line: line.to_string(),
            target: target.to_string()
        }
    }
}

impl Error for ParseLineError {}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The following line could not be parsed as {}: \n {}", self.target, self.line)
    }
}