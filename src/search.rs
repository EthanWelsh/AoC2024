use std::hash::Hash;

pub fn has_path<F, G, I>(start: &I, neighbors: &F, is_goal: &G) -> bool
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
            return true;
        }

        for neighbor in neighbors(&node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor);
            }
        }
    }

    false
}
