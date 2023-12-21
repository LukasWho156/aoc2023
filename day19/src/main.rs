// okay, this is probably completely overkill, but I want to do this with macros.
// I'm even willing to break rule 2 for that.

// ugh, of course part 2 is like that! Probably would have been easier to do this
// without macros after all.

// quick feasability calculation for a naive approach: 2 * 10^2 parts take 10^-4 s,
// so 2 * 10^14 parts will take around 10^8 s. Yup, definitely way too long.
// so we have to work with break points instead. Getting day5 flashbacks.

use std::{error::Error, str::FromStr, time::Instant};

use aoc::{ParseLineError, PuzzlePart};
use macros::{make_workflows, make_ranges, make_splits};

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn value(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.replace("{", "").replace("}", "");
        let split = line.split(",");
        let values: Vec<u64> = split.map(|s| {
            s.split("=").nth(1).unwrap().parse().unwrap()
        }).collect();
        Ok(Part {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        })
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    x: Vec<(u64, u64)>,
    m: Vec<(u64, u64)>,
    a: Vec<(u64, u64)>,
    s: Vec<(u64, u64)>,
}

impl PartRange {

    fn default() -> PartRange {
        PartRange {
            x: vec![(1, 4000)],
            m: vec![(1, 4000)],
            a: vec![(1, 4000)],
            s: vec![(1, 4000)],
        }
    }

}

make_splits!();

fn fn_true(_: &Part) -> bool {
    true
}

fn fn_false(_: &Part) -> bool {
    false
}

fn range_fn_true(range: &mut PartRange) -> u64 {
    range.x.iter().map(|(min, max)| max - min + 1).sum::<u64>()
    * range.m.iter().map(|(min, max)| max - min + 1).sum::<u64>()
    * range.a.iter().map(|(min, max)| max - min + 1).sum::<u64>()
    * range.s.iter().map(|(min, max)| max - min + 1).sum::<u64>()
}

fn range_fn_false(range: &mut PartRange) -> u64 {
    0
}

// needs to be edited manually for tests, unfortunately, but don't want to mess with config stuff.
make_workflows!("input.txt");
make_ranges!("input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    if aoc::puzzle_part() == PuzzlePart::PartOne {
        let parts: Vec<Part> = input.iter()
            .filter(|line| line.starts_with("{"))
            .map(|line| line.parse().unwrap())
            .collect();
        println!("{}", parts.len());
        let time = Instant::now();
        let res: u64 = parts.iter()
            .filter(|part| my_in(part))
            .map(|part| part.value())
            .sum();
        println!("{}, solved in {} s", res, time.elapsed().as_secs_f64());
    } else {
        let mut range = PartRange::default();
        //let r2 = split_x_gt(&mut range, 2345);
        let res = range_my_in(&mut range);
        println!("{}", res);
    }
    Ok(())
}
