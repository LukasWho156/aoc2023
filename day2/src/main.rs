use std::str::FromStr;
use std::cmp;
use std::error::Error;
use aoc::{self, PuzzlePart, ParseLineError};

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>
}

impl FromStr for Game {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(": ").collect();
        if split.len() < 2 {
            return Err(ParseLineError::new("Game", s));
        }
        let def: Vec<&str> = split[0].split(" ").collect();
        if def.len() < 2 || def[0] != "Game" {
            return Err(ParseLineError::new("Game", s));
        }
        let id = def[1].parse();
        if id.is_err() {
            return Err(ParseLineError::new("Game", s));
        }
        let id = id.unwrap();
        let sets: Vec<Set> = split[1].split("; ")
            .filter_map(|e| {
                match e.parse() {
                    Ok(r) => Some(r),
                    Err(e) => {
                        println!("{}", e);
                        None
                    }
                }
            })
            .collect();
        Ok(Game { id, sets })
    }
}

impl Game {

    fn is_valid(&self) -> bool {
        for set in &self.sets {
            if !set.is_valid() {
                return false;
            }
        }
        true
    }

    fn get_minimal_set(&self) -> Set {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in &self.sets {
            red = cmp::max(red, set.red);
            green = cmp::max(green, set.green);
            blue = cmp::max(blue, set.blue);
        }
        Set { red, green, blue }
    }

}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

impl FromStr for Set {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s.split(", ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for entry in cubes {
            let split: Vec<&str> = entry.split(" ").collect();
            if split.len() < 2 {
                return Err(ParseLineError::new("Set", s));
            }
            let amount = split[0].parse();
            if amount.is_err() {
                return Err(ParseLineError::new("Set", s));
            }
            let amount = amount.unwrap();
            match split[1] {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => return Err(ParseLineError::new("Set", s)),
            }
        }
        Ok(Set { red, green, blue })
    }
}

impl Set {

    fn is_valid(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }

}

fn main() -> Result<(), Box<dyn Error>>  {
    let input = aoc::read_input()?;
    let input = input.iter()
        .filter_map(|s| {
            match s.parse() {
                Ok(r) => Some(r),
                Err(e) => {
                    println!("{}", e);
                    None
                },
            }
        });
    let res: i32 = match aoc::puzzle_part() {
        PuzzlePart::PartOne => input.filter_map(|g: Game| {
            match g.is_valid() {
                true => Some(g.id),
                false => None,
            }
        }).sum(),
        PuzzlePart::PartTwo => input.map(|g: Game| {
            g.get_minimal_set().power()
        }).sum(),
    };
    println!("{:?}", res);
    Ok(())
}
