// Okay, I have an idea for an algorithm:
// 0. All nodes start as (U)ndecided
// 1. Mark an arbitrary node as (P)art of the network
// 2. Mark all (U) nodes that are connected to a (P) node as (A)djacent
// 3. If there are 3 or less nodes between (P) and (A) nodes:
//    You can cut off (P) from the rest of the graph. Done!
// 4. For every (A) node: If it is connected to more than 3 nodes that
//    are either (A) or (P), it must be (P) as well.
// 5. If a new (P) node was found during the previous step, return to
//    step 2.
// 6. Hopefully, this is never reached? Otherwise, not sure what to do.

// Okay, step 6 was reached, but we can just trial-and-error our way
// to victory afterwards. Very fun final puzzle, I enjoyed this one
// a lot.

// Christmas is saved!

use std::{error::Error, collections::{HashMap, HashSet}};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Label {
    Undecided,
    Part,
    Adjacent,
}

fn solve(nodes: &HashMap<String, Label>, edges: &HashMap<String, HashSet<String>>, start: String) -> bool {
    println!("Guess: {} is part of the graph", start);
    let mut nodes = nodes.clone();
    let mut new_parts: Vec<String> = vec![start];
    while let Some(p) = new_parts.pop() {
        nodes.insert(p.to_string(), Label::Part);
        for c in edges.get(&p).unwrap() {
            if let Some(Label::Undecided) = nodes.get(c) {
                nodes.insert(c.to_string(), Label::Adjacent);
            }
        }
        let mut total_p = 0;
        let mut possible: Vec<(String, usize)> = nodes.iter().filter(|(_, v)| v == &&Label::Adjacent).map(|(k, _)| {
            let mut n_a = 0;
            let mut n_p = 0;
            for c in edges.get(k).unwrap() {
                match nodes.get(c).unwrap() {
                    Label::Undecided => (),
                    Label::Part => n_p += 1,
                    Label::Adjacent => n_a += 1,
                }
            }
            total_p += n_p;
            //println!("No connections for {}: {} + {} = {}", k, n_a, n_p, n_a + n_p);
            if n_a + n_p > 3 {
                new_parts.push(k.to_string());
            }
            (k.to_string(), n_a + n_p)
        }).collect();
        if total_p <= 3 {
            println!("Found solution! {:?}", nodes);
            let parts = nodes.values().filter(|n| n == &&Label::Part).count();
            let not_part = nodes.len() - parts;
            println!("Part of the graph: {}; not part: {}; product: {}", parts, not_part, parts * not_part);
            return true;
        }
        if new_parts.len() == 0 {
            possible.sort_by(|(_, a), (_, b)| a.cmp(b));
            println!("{:?}", possible);
            while let Some((n, _)) = possible.pop() {
                if solve(&nodes, &edges, n) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut nodes: HashMap<String, Label> = HashMap::new();
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    let input = aoc::read_input()?;
    input.iter().filter(|line| line.len() > 0).for_each(|line| {
        let mut split = line.split(": ");
        let source = split.next().unwrap();
        nodes.insert(source.to_string(), Label::Undecided);
        split.next().unwrap().split(" ").for_each(|dest| {
            nodes.insert(dest.to_string(), Label::Undecided);
            if let Some(e) = edges.get_mut(source) {
                e.insert(dest.to_string());
            } else {
                let mut set = HashSet::new();
                set.insert(dest.to_string());
                edges.insert(source.to_string(), set);
            }
            if let Some(e) = edges.get_mut(dest) {
                e.insert(source.to_string());
            } else {
                let mut set = HashSet::new();
                set.insert(source.to_string());
                edges.insert(dest.to_string(), set);
            }
        });
    });
    solve(&nodes, &edges, nodes.keys().next().unwrap().to_string());
    //println!("{:?}", nodes);
    //println!("{:?}", edges);
    Ok(())
}