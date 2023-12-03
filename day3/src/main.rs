use std::error::Error;
use std::cmp;
use std::collections::HashMap;
use aoc::{PuzzlePart, self};

// very unoptimized solution, especially for part 2, but it works, so whatever.

fn create_symbol_map(input: &Vec<String>) -> HashMap<(usize, usize), char> {
    let mut symbol_map: HashMap<(usize, usize), char> = HashMap::new();
    input.iter().enumerate().for_each(|(y, e)| {
        e.chars().enumerate().for_each(|(x, c)| {
            if c != '.' && !c.is_ascii_digit() {
                symbol_map.insert((x, y), c);
            }
        });
    });
    symbol_map
}

#[derive(Debug)]
struct Part {
    value: i32,
    adjacent_symbols: Vec<(usize, usize, char)>,
}

impl Part {

    fn is_valid(&self) -> bool {
        self.adjacent_symbols.len() > 0
    }

    fn is_connected(&self, symbol: &(usize, usize, char)) -> bool {
        self.adjacent_symbols.contains(symbol)
    }

}

fn create_parts(input: &Vec<String>, symbol_map: &HashMap<(usize, usize), char>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    input.iter().enumerate().for_each(|(y, e)| {
        let mut chars = e.chars().enumerate();
        while let Some((x, c)) = chars.next() {
            if c.is_ascii_digit() {
                let min_y = match y {
                    0 => 0,
                    _ => y - 1,
                };
                let max_y = y + 2;
                let min_x = match x {
                    0 => 0,
                    _ => x - 1,
                };
                let mut max_x = x + 2;
                let mut value_string = c.to_string();
                loop {
                    if let Some((x, c)) = chars.next() {
                        if c.is_ascii_digit() {
                            value_string.push(c);
                            max_x = x + 2;
                            continue;
                        }
                    }
                    break;
                }
                let mut adjacent_symbols: Vec<(usize, usize, char)> = Vec::new();
                for ty in min_y..max_y {
                    for tx in min_x..max_x {
                        if let Some(c) = symbol_map.get(&(tx, ty)) {
                            adjacent_symbols.push((tx as usize, ty as usize, *c));
                        }
                    }
                }
                parts.push(Part {
                    value: value_string.parse().unwrap(),
                    adjacent_symbols,
                });
            }
        }
    });
    parts
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let symbol_map = create_symbol_map(&input);
    let parts = create_parts(&input, &symbol_map);
    let res: i32 = match aoc::puzzle_part() {
        PuzzlePart::PartOne => parts.iter().filter_map(|p| {
            match p.is_valid() {
                true => Some(p.value),
                false => None,
            }
        }).sum(),
        PuzzlePart::PartTwo => symbol_map.iter()
            .filter(|(_, sym)| **sym == '*')
            .filter_map(|(k, sym)| {
                let connected: Vec<&Part> = parts.iter().filter(|p| p.is_connected(&(k.0, k.1, *sym))).collect();
                match connected.len() == 2 {
                    true => Some(connected.get(0).unwrap().value * connected.get(1).unwrap().value),
                    false => None,
                }
            })
            .sum(),
    };
    println!("{}", res);
    Ok(())
}
