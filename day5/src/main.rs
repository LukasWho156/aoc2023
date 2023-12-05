use std::error::Error;
use std::str::FromStr;
use aoc::{self, ParseLineError, PuzzlePart};

struct Map {
    steps: Vec<(i64, i64)>,
}

impl Map {

    fn convert(&self, input: i64) -> i64 {
        let step = self.steps.iter().rev().find(|i| i.0 <= input).unwrap();
        input + step.1
    }

    fn new(ranges: &mut Vec<MapRange>) -> Map {
        ranges.sort_by(|a, b| a.from.cmp(&b.from));
        let mut min = 0;
        let mut steps: Vec<(i64, i64)> = Vec::new();
        for r in ranges {
            if r.from > min {
                steps.push((min, 0));
            }
            steps.push((r.from, r.offset));
            min = r.to;
        }
        steps.push((min, 0));
        Map { steps }
    }

    fn convert_range(&self, input: (i64, i64)) -> Vec<(i64, i64)> {
        let mut it = self.steps.iter();
        let mut res: Vec<(i64, i64)> = Vec::new();
        let mut prev_min = input.0;
        let mut prev_offset = 0;
        while let Some(step) = it.next() {
            if step.0 <= input.0 {
                prev_offset = step.1;
                continue;
            }
            if step.0 > input.1 {
                res.push((prev_min + prev_offset, input.1 + prev_offset));
                break;
            }
            res.push((prev_min + prev_offset, step.0 + prev_offset));
            prev_min = step.0;
            prev_offset = step.1;
        }
        res
    }

}

#[derive(Debug)]
struct MapRange {
    from: i64,
    to: i64,
    offset: i64,
}

impl FromStr for MapRange {

    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<i64> = s.split(" ")
            .filter_map(|e| match e.parse() {
                Ok(i) => Some(i),
                Err(e) => {
                    println!("{}", e);
                    None
                }
            })
            .collect();
        if split.len() != 3 {
            return Err(ParseLineError::new("MapRange", s));
        }
        Ok(MapRange {
            from: split[1],
            to: split[1] + split[2],
            offset: split[0] - split[1],
        })
    }

}

fn parse_map(it: &mut dyn Iterator<Item = &String>) -> Map {
    let mut ranges: Vec<MapRange> = Vec::new();
    while let Some(line) = it.next() {
        if let Ok(range) = line.parse() {
            ranges.push(range);
        } else {
            break;
        }
    }
    //println!("{:?}", ranges);
    Map::new(&mut ranges)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut it = input.iter();
    let mut maps: Vec<Map> = Vec::with_capacity(7);
    while let Some(line) = it.next() {
        if line.contains("map") {
            maps.push(parse_map(&mut it))
        }
    }
    let res: i64 = match aoc::puzzle_part() {
        PuzzlePart::PartOne => {
            input[0].split(" ").filter_map(|e| match e.parse() {
                Ok(i) => Some(i),
                Err(e) => None,
            }).map(|s: i64| {
                let mut o = s;
                for map in &maps {
                    o = map.convert(o);
                }
                o
            }).min().unwrap()
        },
        // okay, turns out part 2 isn't that bad if you take some time setting up proper modeling.
        // still, definitely the nastiest part 2 so far
        PuzzlePart::PartTwo => {
            let list: Vec<&str> = input[0].split(" ").collect();
            let mut ranges: Vec<(i64, i64)> = Vec::with_capacity(10);
            for i in 1..list.len() / 2 {
                let bottom = list[2 * i - 1].parse::<i64>().unwrap();
                let top = bottom + list[2 * i].parse::<i64>().unwrap() - 1;
                ranges.push((bottom, top));
            }
            for m in maps {
                println!("{:?}", ranges);
                let ranges_copy = ranges.clone();
                ranges.clear();
                for r in ranges_copy {
                    for nr in m.convert_range(r) {
                        ranges.push(nr);
                    }
                }
            }
            ranges.iter().map(|r| r.0).min().unwrap()
        }
    };
    println!("{}", res);
    Ok(())
}
