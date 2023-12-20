// wait, isn't this a rehash of day 10, basically? Should be doable, then.
// color codes are a little scary, though.

// oh snap, turns out I was right about being woried about the color codes.
// oh well. Let's just see if we can finish in a reasonable amount of time anyway?
// yeah no, we ain't gonna finish that in a reasonable amount of time. Let's try to think about this.

// okay, I have a solution that might work, but it's weird as heck. No idea if it's correct. Let's try.
// oops, doesn't work at all, just coincidence

// oooh, I think we can do this with triangles!
// that's actually a pretty neat solution. Lovely! Great riddle, glad I had to work for it.

use std::error::Error;
use aoc::{self, PuzzlePart};

struct Cmd {
    len: i64,
    dir: (i64, i64),
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let commands: Vec<Cmd> = input.iter().filter(|line| line.len() > 0).map(|line| {
        let mut split = line.split(" ");
        let mut dir = match split.next().unwrap() {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0, 0),
        };
        let mut len: i64 = split.next().unwrap().parse().unwrap();
        if aoc::puzzle_part() == PuzzlePart::PartTwo {
            let code = split.next().unwrap();
            let num = &code[2..7];
            len = i64::from_str_radix(num, 16).unwrap();
            let cmd = &code[7..8];
            dir = match cmd {
                "0" => (1, 0),
                "1" => (0, 1),
                "2" => (-1, 0),
                "3" => (0, -1),
                _ => (0, 0),
            };
        }
        //println!("{:?}, {}", dir, len);
        Cmd { dir, len }
    }).collect();
    let mut relative: (i64, i64) = (0, 0);
    let mut area: i64 = 0;
    let mut circumference: i64 = 0;
    for cmd in &commands {
        circumference += cmd.len;
        relative.0 += cmd.dir.0 * cmd.len;
        relative.1 += cmd.dir.1 * cmd.len;
        let height = cmd.dir.1 * relative.0 - cmd.dir.0 * relative.1;
        area += height * cmd.len;
        //println!("{}, {}", area, circumference);
    }
    let total = (area + circumference) / 2 + 1;
    println!("{}", total);
    Ok(())
}
