// Alright, part 1 seems like a weirdly simple counting puzzle? Let's
// see if I'm mistaken. Also, don't feel like implementing fancy
// structs right now, so let's hope it doesn't bite me in part 2.

// Oooh, part 2 looks interesting. Multiple approaches:
//   * brute-force, obviously. Probably wouldn't take ages, but kinda boring?
//   * some clever math, including cycle detection and chinese remainder theorem stuff?
//     sounds fun, but do I have the time?
// After some consideration, you know what they say about premature optimization,
// so let's try to brute-force.
// Okay, ran the brute-force algorithm for a couple of minutes, let's actually take some
// time to think about it instead.
// huh. Turns out all the cycles are fully cyclic, so we don't even need CRT, we just
// need least common multiple.

use std::error::Error;
use std::collections::HashMap;
use aoc::{self, PuzzlePart, ParseLineError};


fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let directions: Vec<char> = input[0].chars().filter(|c| *c == 'R' || *c == 'L').collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    input.iter().filter(|e| e.contains("=")).for_each(|e| {
        let mut split_1 = e.split(" = (");
        let node = split_1.next().unwrap();
        let mut split_2 = split_1.next().unwrap().split(", ");
        let left = split_2.next().unwrap();
        let right = split_2.next().unwrap().replace(")", "").replace("\r", "");
        map.insert(String::from(node), (String::from(left), String::from(right)));
    });
    if aoc::puzzle_part() == PuzzlePart::PartOne {
        let mut steps = 0;
        let mut cur_pos = "AAA";
        loop {
            for dir in &directions {
                let node = map.get(&String::from(cur_pos)).unwrap();
                cur_pos = match dir {
                    'L' => node.0.as_str(),
                    'R' => node.1.as_str(),
                    _ => panic!("Unknown direction"),
                };
                steps += 1;
                if cur_pos == "ZZZ" {
                    println!("Done! Took {} steps", steps);
                    return Ok(());
                }
            }
        }
    } else {
        let cycles: Vec<i64> = map.iter().filter_map(|(k, v)| {
            match k.chars().nth(2) == Some('A') {
                true => {
                    println!("Starting point: {}", k);
                    Some(k.as_str())
                },
                false => None,
            }
        }).map(|pos| {
            let mut cur_pos = pos;
            let mut steps = 0;
            let mut first_occurence = 0;
            let mut cycle_length = 0;
            loop {
                for dir in &directions {
                    let node = map.get(&String::from(cur_pos)).unwrap();
                    cur_pos = match dir {
                        'L' => node.0.as_str(),
                        'R' => node.1.as_str(),
                        _ => panic!("Unknown direction"),
                    };
                    steps += 1;
                    if cur_pos.chars().nth(2) == Some('Z') {
                        return steps;
                    }
                }
            }
        }).collect();
        println!("cycle lengths: {:?}", cycles);
        let divisors: Vec<Vec<i64>> = cycles.iter().map(|cycle| {
            let mut f = 2;
            let mut remainder = *cycle;
            let mut factors: Vec<i64> = Vec::new();
            while f * f < remainder {
                while remainder % f == 0 {
                    remainder /= f;
                    factors.push(f);
                }
                f += 1;
            }
            if remainder > 1 {
                factors.push(remainder);
            }
            factors
        }).collect();
        println!("{:?}", divisors);
    }
    Ok(())
}
