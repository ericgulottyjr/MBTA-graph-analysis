use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

//Eq not implemented properly for f32, does not work with rest of code
#[derive(Eq, PartialEq)]
struct DistanceVertex(f32, usize);

impl Ord for DistanceVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.partial_cmp(&self.0).unwrap()
    }
}

impl PartialOrd for DistanceVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(adj_matrix: &Vec<Vec<f32>>, start: usize, end: usize) -> Vec<usize> {
    let mut distances: HashMap<usize, f32> = (0..adj_matrix.len()).map(|i| (i, f32::INFINITY)).collect();
    distances.insert(start, 0.0);

    let mut heap = BinaryHeap::new();
    heap.push(DistanceVertex(0.0, start));

    let mut previous: HashMap<usize, Option<usize>> = (0..adj_matrix.len()).map(|i| (i, None)).collect();

    while let Some(DistanceVertex(distance, u)) = heap.pop() {
        if u == end {
            break;
        }

        if distance > distances[&u] {
            continue;
        }

        for (v, weight) in adj_matrix[u].iter().enumerate() {
            let distance = distance + weight;
            if distance < distances[&v] {
                distances.insert(v, distance);
                previous.insert(v, Some(u));
                heap.push(DistanceVertex(distance, v));
            }
        }
    }

    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        current = previous[&current].unwrap();
    }
    path.push(start);
    path.reverse();
    path
}
