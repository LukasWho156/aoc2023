use std::error::Error;
use std::str::FromStr;
use aoc::{self, ParseLineError, PuzzlePart};

fn extract_numbers(s: &str) -> Vec<i32> {
    s.split(" ").filter_map(|e| {
        match e.parse() {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }).collect()
}

struct Card {
    winning_numbers: Vec<i32>,
    owned_numbers: Vec<i32>,
}

impl Card {

    fn no_winning(&self) -> usize {
        self.winning_numbers.iter().filter(|i| {
            self.owned_numbers.contains(i)
        }).count()
    }

    fn value(&self) -> i32 {
        let no_winning = self.no_winning();
        match no_winning {
            0 => 0,
            x => 1 << (x - 1),
        }
    }

}

impl FromStr for Card {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split("|").collect();
        if split.len() != 2 {
            return Err(ParseLineError::new("Card", s));
        }
        Ok(Card {
            winning_numbers: extract_numbers(split[0]),
            owned_numbers: extract_numbers(split[1]),
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let cards = input.iter().filter_map(|s| {
        match s.parse() {
            Ok(c) => Some(c),
            Err(e) => {
                println!("{}", e);
                None
            },
        }
    });
    let res: i32 = match aoc::puzzle_part() {
        PuzzlePart::PartOne => {
            cards.map(|c: Card| {
                c.value()
            }).sum()
        },
        PuzzlePart::PartTwo => {
            // there might be some smarter algorithm to do this if you look at the math, but this takes about a second to run, so why bother?
            let mut copies: Vec<i32> = vec![1; cards.clone().count()]; // clone isn't ideal here I think, but not quite sure how to avoid it easily. Still runs quick enough.
            cards.enumerate().for_each(|(i, c)| {
                let max_offset = c.no_winning() + 1;
                for offset in 1..max_offset {
                    copies[i + offset] += copies[i];
                }
            });
            copies.iter().sum()
        },
    };  
    println!("{}", res);
    Ok(())
}
