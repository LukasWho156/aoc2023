use std::{error::Error, collections::HashMap};
use aoc::{self, PuzzlePart};

// welp, that was a simple part 1. let's see how part 2 treats us today.
// even part 2 was pretty easy. neat!

fn h_a_s_h(text: &str) -> u8 {
    let mut res: u8 = 0;
    text.as_bytes().iter().for_each(|b| {
        res = res.wrapping_add(*b);
        res = res.wrapping_mul(17);
    });
    res
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let res: usize = match aoc::puzzle_part() {
        PuzzlePart::PartOne => {
            input[0].split(",")
                .map(|cmd| h_a_s_h(cmd) as usize)
                .sum()
        },
        PuzzlePart::PartTwo => {
            let mut map: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
            for i in 0..=255 {
                map.insert(i, Vec::new());
            }
            input[0].split(",")
                .for_each(|cmd| {
                    let (label, op) = if cmd.contains("-") {
                        (cmd.split("-").next().unwrap(), None)
                    } else {
                        let mut split = cmd.split("=");
                        (split.next().unwrap(), Some(split.next().unwrap().parse::<u8>().unwrap()))
                    };
                    let hash = h_a_s_h(label);
                    let vec = map.get_mut(&hash).unwrap();
                    let label = String::from(label);
                    match op {
                        Some(v) => {
                            match vec.iter_mut().find(|b| b.0 == label) {
                                Some(b) => b.1 = v,
                                None => vec.push((label, v)),
                            };
                        },
                        None => {
                            match vec.iter().enumerate().find(|(_, b)| b.0 == label) {
                                Some((i, _)) => {
                                    vec.remove(i);
                                },
                                None => (),
                            };
                        }
                    }
                    //println!("{:?}", vec);
                });
            (0..=255).map(|hash| {
                let b = map.get(&hash).unwrap();
                b.iter().enumerate().map(|(i, v)| {
                    let r = (hash as usize + 1) * (i + 1) * v.1 as usize;
                    //println!("{}", r);
                    r
                }).sum::<usize>()
            }).sum()
        },
    };
    println!("{}", res);
    Ok(())
}

