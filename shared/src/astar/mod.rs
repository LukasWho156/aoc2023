use std::{hash::Hash, collections::{HashMap, BinaryHeap}};

pub trait AStarGraph {
    type Node: Hash + Clone + Eq;
    fn get_neighbours(&self, current: &Self::Node) -> Vec<(Self::Node, usize)>;
    fn get_heuristic(&self, target: &Self::Node) -> usize;
    fn is_goal(&self, target: &Self::Node) -> bool;
}

#[derive(Eq, Debug)]
pub struct AStarState<N: Hash + Eq> {
    pub node: N,
    pub estimated_cost: usize,
}

impl<N: Hash + Eq> PartialEq for AStarState<N> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost)
    }
}

impl<N: Hash + Eq> Ord for AStarState<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl<N: Hash + Eq> PartialOrd for AStarState<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn a_star<G: AStarGraph>(graph: &G, start: G::Node) -> Option<usize> {
    let mut visited: HashMap<G::Node, usize> = HashMap::new();
    let mut expanded: HashMap<G::Node, ()> = HashMap::new();
    visited.insert(start.clone(), 0);
    let mut open_list: BinaryHeap<AStarState<G::Node>> = BinaryHeap::new();
    open_list.push(AStarState { node: start, estimated_cost: 0 });
    while let Some(next) = open_list.pop() {
        if expanded.get(&next.node).is_some() {
            continue;
        }
        expanded.insert(next.node.clone(), ());
        let current_cost = *visited.get(&next.node).unwrap();
        if graph.is_goal(&next.node) {
            return Some(current_cost);
        }
        graph.get_neighbours(&next.node).into_iter().for_each(|(node, cost)| {
            let next_cost = current_cost + cost;
            let compare = visited.get(&node);
            if let Some(c) = compare {
                if c < &next_cost {
                    return;
                }
            }
            visited.insert(node.clone(), next_cost);
            let estimated = next_cost + graph.get_heuristic(&node);
            //println!("cost: {}, {}", next_cost, estimated);
            open_list.push(AStarState { node, estimated_cost: estimated });
        });
        //thread::sleep(Duration::from_secs(1));
    }
    None
}