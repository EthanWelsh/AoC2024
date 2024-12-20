use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use num_traits::Zero;

pub fn dijkstra<N, FN, IN, FS>(
    start: &N,
    mut successors: FN, // Made successors mutable
    mut is_goal: FS,  // Made is_goal mutable
) -> Option<(N, u64)>
where
    N: Eq + Hash + Clone + Ord,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, u64)>,
    FS: FnMut(&N) -> bool,
{
    let mut distances: HashMap<N, u64> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<u64>, N)> = BinaryHeap::new();

    distances.insert(start.clone(), 0);
    queue.push((Reverse(0), start.clone()));

    while let Some((Reverse(current_dist), current)) = queue.pop() {
        if is_goal(&current) {
            return Some((current, current_dist));
        }

        if current_dist > *distances.get(&current).unwrap_or(&u64::MAX) {
            continue;
        }

        for (neighbor, cost) in successors(&current) { // Call mutable successors here
            let new_dist = current_dist + cost;
            if new_dist < *distances.entry(neighbor.clone()).or_insert(u64::MAX) {
                distances.insert(neighbor.clone(), new_dist);
                queue.push((Reverse(new_dist), neighbor));
            }
        }
    }

    None
}

pub fn dijkstra_all_shortest_paths<N, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut is_goal: FS,
) -> Option<(Vec<Vec<N>>, u64)>
where
    N: Eq + Hash + Clone + Ord,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, u64)>,
    FS: FnMut(&N) -> bool,
{
    let mut distances: HashMap<N, u64> = HashMap::new();
    let mut paths: HashMap<N, Vec<Vec<N>>> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<u64>, N)> = BinaryHeap::new();

    distances.insert(start.clone(), 0);
    paths.insert(start.clone(), vec![vec![start.clone()]]);
    queue.push((Reverse(0), start.clone()));

    while let Some((Reverse(current_dist), current)) = queue.pop() {
        if is_goal(&current) {
            let all_paths = paths.get(&current).unwrap().clone();
            return Some((all_paths, current_dist));
        }

        if current_dist > *distances.get(&current).unwrap_or(&u64::MAX) {
            continue;
        }

        for (neighbor, cost) in successors(&current) {
            let new_dist = current_dist + cost;

            if new_dist < *distances.entry(neighbor.clone()).or_insert(u64::MAX) {
                // Found a shorter path, clear previous paths
                distances.insert(neighbor.clone(), new_dist);
                let mut new_paths = Vec::new();
                for path in paths.get(&current).unwrap() {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    new_paths.push(new_path);
                }
                paths.insert(neighbor.clone(), new_paths);

                queue.push((Reverse(new_dist), neighbor));
            } else if new_dist == *distances.get(&neighbor).unwrap_or(&u64::MAX) {
                // Found an equally short path, add it to existing paths
                let mut existing_paths = paths.get(&neighbor).unwrap().clone();
                for path in paths.get(&current).unwrap() {
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    existing_paths.push(new_path);
                }
                paths.insert(neighbor.clone(), existing_paths);
            }
        }
    }

    None
}
