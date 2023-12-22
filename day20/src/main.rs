// Looks like a fun exercise. Been wanting to learn more about Deques.
// Also, I smell big number shenanigans incoming, so let's do the
// state storing thing from day 14 again.

// oh no, I have a terrible sense of scent. This appears to be more of a
// logic puzzle. This is gonna be tricky.

// probably some sort of binary counter? I'll have to think about the
// gates a bit.

// not really a binary counter (or at least not completely), but multiple
// cycles with prime lengths having to line up. I didn't fully analyze this
// and my solution only works for my input, but it should be easy enough
// to adjust, so I'm happy with it.

use std::collections::VecDeque;
use std::{error::Error, collections::HashMap, str::FromStr};
use std::fmt::Debug;

use aoc::ParseLineError;

const PRECURSORS: [&'static str; 4] = ["vd", "nd", "pc", "tx"];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse {
    from: String,
    to: String,
    pulse_type: PulseType,
}

trait Sender: Debug {
    fn connect(&mut self, sender: String);
    fn process_pulse(&mut self, incoming: &Pulse) -> Option<PulseType>;
    fn get_state(&self) -> usize;
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn new() -> FlipFlop { FlipFlop { state: false }}
}

impl Sender for FlipFlop {

    fn connect(&mut self, _: String) {
        // do nothing
    }

    fn process_pulse(&mut self, incoming: &Pulse) -> Option<PulseType> {
        match incoming.pulse_type {
            PulseType::High => None,
            PulseType::Low => {
                self.state = !self.state;
                match self.state {
                    true => Some(PulseType::High),
                    false => Some(PulseType::Low),
                }
            }
        }
    }

    fn get_state(&self) -> usize {
        self.state as usize
    }

}

#[derive(Debug)]
struct Conjunction {
    no_inputs: usize,
    inputs: HashMap<String, usize>,
    state: usize,
}

impl Conjunction {
    fn new() -> Conjunction { Conjunction { no_inputs: 0, inputs: HashMap::new(), state: 0 }}
}

impl Sender for Conjunction {

    fn connect(&mut self, sender: String) {
        self.inputs.insert(sender, self.no_inputs);
        self.no_inputs += 1;
    }

    fn process_pulse(&mut self, incoming: &Pulse) -> Option<PulseType> {
        //println!("{:?}. {:?}", self.inputs, incoming);
        let i = self.inputs.get(&incoming.from).unwrap();
        match incoming.pulse_type {
            PulseType::High => self.state |= 1 << i,
            PulseType::Low => self.state &= !(1 << i),
        }
        self.state &= (1 << self.no_inputs) - 1;
        //println!("{:#b}, {:#b}", self.state, (1 << self.no_inputs) - 1);
        if (1 << self.no_inputs) - 1 == self.state {
            Some(PulseType::Low)
        } else {
            Some(PulseType::High)
        }
    }

    fn get_state(&self) -> usize {
        self.state
    }

}

#[derive(Debug)]
struct Bypass;

impl Bypass {
    fn new() -> Bypass { Bypass {} }
}

impl Sender for Bypass {
    fn connect(&mut self, _: String) {
        // do nothing
    }

    fn process_pulse(&mut self, incoming: &Pulse) -> Option<PulseType> {
        Some(incoming.pulse_type)
    }

    fn get_state(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Module {
    id: String,
    sender: Box<dyn Sender>,
    receivers: Vec<String>
}

impl FromStr for Module {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(ParseLineError::new("Module", s));
        }
        let mut split = s.split(" -> ");
        let module = split.next().unwrap();
        let mod_type = module.chars().next().unwrap();
        let sender: Box<dyn Sender> = match mod_type {
            '%' => Box::new(FlipFlop::new()),
            '&' => Box::new(Conjunction::new()),
            _ => Box::new(Bypass::new()),
        };
        let id = module.replace("&", "").replace("%", "");
        let receivers = split.next().unwrap().split(", ").map(|t| String::from(t)).collect();
        Ok(Module { id, sender, receivers })
    }
}

impl Module {

    fn process_pulse(&mut self, incoming: &Pulse) -> Vec<Pulse> {
        match self.sender.process_pulse(incoming) {
            Some(t) => self.receivers.iter()
                .map(|r| Pulse { from: self.id.clone(), to: r.clone(), pulse_type: t })
                .collect(),
            None => vec![],
        }
    }

}

fn press_button(module_plan: &mut HashMap<String, Module>, i: usize) -> (usize, usize) {
    let mut pulse_amounts = (0, 0);
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
    pulse_queue.push_back(Pulse {
        from: String::from("button"),
        to: String::from("broadcaster"),
        pulse_type: PulseType::Low
    });
    while let Some(p) = pulse_queue.pop_front() {
        if PRECURSORS.iter().find(|id| &&p.from == id).is_some() && p.pulse_type == PulseType::High {
            println!("High pulse from {} after {} presses!", p.from, i);
        }
        //let str = match p.pulse_type {
        //    PulseType::High => "high",
        //    PulseType::Low => "low",
        //};
        //println!("{} -{}-> {}", p.from, str, p.to);
        if &p.to == "rx" && p.pulse_type == PulseType::Low {
            println!("Hello from rx!");
        }
        match p.pulse_type {
            PulseType::High => pulse_amounts.1 += 1,
            PulseType::Low => pulse_amounts.0 += 1,
        }
        if let Some(r) = module_plan.get_mut(&p.to) {
            let new_pulses = r.process_pulse(&p);
            new_pulses.into_iter().for_each(|np| pulse_queue.push_back(np));
        }
    }
    pulse_amounts
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc::read_input()?;
    let mut module_plan: HashMap<String, Module> = HashMap::new();
    input.iter().for_each(|line| {
        match line.parse::<Module>() {
            Ok(m) => module_plan.insert(m.id.clone(), m),
            Err(_) => None,
        };
    });
    let connections: Vec<(String, Vec<String>)> = module_plan.values().map(|s| {
        (s.id.clone(), s.receivers.clone())
    }).collect();
    println!("{:?}", connections);
    connections.iter().for_each(|(s, recs)| {
        recs.iter().for_each(|r| {
            if let Some(rec) = module_plan.get_mut(r) {
                rec.sender.connect(s.clone());
            }
        });
    });
    let mut button_presses: Vec<(usize, usize)> = Vec::new();
    let mut seen_states: HashMap<Vec<usize>, usize> = HashMap::new();
    let hash: Vec<usize> = module_plan.values().map(|m| m.sender.get_state()).collect();
    //println!("{:?}", hash);
    seen_states.insert(hash, 0);
    let max_cycles = match aoc::puzzle_part() {
        aoc::PuzzlePart::PartOne => 1000,
        aoc::PuzzlePart::PartTwo => 100000,
    };
    for i in 1..max_cycles + 1 {
        let presses = press_button(&mut module_plan, i);
        button_presses.push(presses);
        let hash: Vec<usize> = module_plan.values().map(|m| m.sender.get_state()).collect();
        //println!("{:?}", hash);
        if let Some(old) = seen_states.get(&hash) {
            println!("State after {} iterations is equal to state after {} iterations", i, old);
            let cycle_len = i - old;
            let offset_start = *old;
            let no_cycles = (max_cycles - offset_start) / cycle_len;
            let offset_end = (max_cycles - offset_start) % cycle_len;
            let mut total_pulses = (0, 0);
            for n in 0..offset_start {
                total_pulses.0 += button_presses[n].0;
                total_pulses.1 += button_presses[n].1;
            }
            for n in offset_start..offset_start + cycle_len {
                total_pulses.0 += no_cycles * button_presses[n].0;
                total_pulses.1 += no_cycles * button_presses[n].1;
            }
            for n in offset_start..offset_start + offset_end {
                total_pulses.0 += button_presses[n].0;
                total_pulses.1 += button_presses[n].1;
            }
            println!("Total pulses: {} high, {} low, {} product",
                total_pulses.1,
                total_pulses.0,
                total_pulses.0 * total_pulses.1
            );
            return Ok(());
        }
        seen_states.insert(hash, i);
    }
    println!("No cycle detected.");
    let mut total_pulses = (0, 0);
    for n in 0..max_cycles {
        total_pulses.0 += button_presses[n].0;
        total_pulses.1 += button_presses[n].1;
    }
    println!("Total pulses: {} high, {} low, {} product",
        total_pulses.1,
        total_pulses.0,
        total_pulses.0 * total_pulses.1
    );
    //println!("{:?}", button_presses);
    Ok(())
}
