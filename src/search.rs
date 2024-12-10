use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

pub fn has_path<F, G, I>(start: &I, neighbors: &F, is_goal: &G) -> bool
where
    F: Fn(&I) -> Vec<I>,
    G: Fn(&I) -> bool,
    I: Clone + PartialEq + Eq + Hash,
{
    search_for_goal(start, neighbors, is_goal).is_some()
}

pub fn search_for_goal<F, G, I>(start: &I, neighbors: &F, is_goal: &G) -> Option<I>
where
    F: Fn(&I) -> Vec<I>,
    G: Fn(&I) -> bool,
    I: Clone + PartialEq + Eq + Hash,
{
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(node) = queue.pop_front() {
        if is_goal(&node) {
            return Some(node);
        }

        for neighbor in neighbors(&node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor);
            }
        }
    }

    None
}

pub fn astar<F, G, H, I>(start: &I, neighbors: &F, heuristic: &H, is_goal: &G) -> Option<I>
where
    F: Fn(&I) -> Vec<I>,
    H: Fn(&I) -> u64,
    G: Fn(&I) -> bool,
    I: Clone + PartialEq + Eq + Hash + Ord,
{
    let mut visited = std::collections::HashSet::new();
    let mut queue = BinaryHeap::new();

    queue.push((Reverse(heuristic(start)), start.clone()));
    visited.insert(start.clone());

    while let Some((Reverse(h), node)) = queue.pop() {
        if is_goal(&node) {
            return Some(node);
        }

        for neighbor in neighbors(&node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                let priority = h + 1 + heuristic(&neighbor);
                queue.push((Reverse(priority), neighbor));
            }
        }
    }

    None
}
