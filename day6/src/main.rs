// Okay, this seems like a cute little math puzzle that shouldn't take too much programming
// effort (at least the first part). The distance the boat can go is d(t) = t * (T - t), where
// t is the time the button is held and T is the total time of the race. After subtracting
// the current record, we just have to find the zero points (which should be simple for
// a quadratic formula) and count how many integers are between them.
//
//     t * (T - t) - R = 0
//     t² - T * t + R = 0
//     t_1/2 = (T +- sqrt(T² - 4 * R)) / 2

use std::error::Error;
use aoc::{self, PuzzlePart};

struct Race {
    total_time: i64,
    record: i64,
}

impl Race {
    fn break_even_points(&self) -> (f64, f64) {
        let disc = ((self.total_time * self.total_time - 4 * self.record) as f64).sqrt();
        let lower = (self.total_time as f64 - disc) * 0.5;
        let upper = (self.total_time as f64 + disc) * 0.5;
        (lower, upper)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let b = input[0].replace("\r", "").replace("Time:", "");
    let times = b.split(" ").filter(|e| *e != "");
    let times: Vec<String> = match aoc::puzzle_part() {
        PuzzlePart::PartOne => times.map(|s| String::from(s)).collect(),
        PuzzlePart::PartTwo => {
            let concat = times.fold(String::from(""), |acc, x| acc + x);
            vec![concat]
        },
    };
    let b = input[1].replace("\r", "").replace("Distance:", "");
    let records = b.split(" ").filter(|e| *e != "");
    let records: Vec<String> = match aoc::puzzle_part() {
        PuzzlePart::PartOne => records.map(|s| String::from(s)).collect(),
        PuzzlePart::PartTwo => {
            let concat = records.fold(String::from(""), |acc, x| acc + x);
            vec![concat]
        },
    };
    println!("{:?}, {:?}", times, records);
    let mut races: Vec<Race> = Vec::with_capacity(times.len());
    for i in 0..times.len() {
        races.push(Race {
            total_time: times[i].parse().unwrap(),  // quick and dirty unwraps. Getting lazy, but meh. We know what the input looks like.
            record: records[i].parse().unwrap(),
        });
    }
    let res: i64 = races.iter().map(|r| {
        let bep = r.break_even_points();
        println!("break even points: {:?}", bep);
        // okay, one mean little trick: We are required to beat the record, not tie it, so we have to consider edge cases.
        let lower = bep.0;
        let upper = bep.1;
        let floor = if upper.floor() == upper {
            upper as i64 - 1
        } else {
            upper.floor() as i64
        };
        let ceil = if lower.ceil() == lower {
            lower as i64 + 1
        } else {
            lower.ceil() as i64
        };
        println!("floor and ceil: {}, {}", floor, ceil);
        floor - ceil + 1
    }).product();
    println!("{}", res);
    Ok(())
}
