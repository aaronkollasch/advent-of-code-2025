use std::collections::BTreeMap;

use crate::common::parse;
use itertools::Itertools;

type Number = isize;

type Pos = (Number, Number, Number);

fn distance(box1: Pos, box2: Pos) -> Number {
    (box1.0 - box2.0).pow(2) + (box1.1 - box2.1).pow(2) + (box1.2 - box2.2).pow(2)
}

pub fn get_result(input: &[u8], num_connections: usize) -> usize {
    let mut boxes = Vec::with_capacity(1000);
    boxes.extend(
        input
            .split(|&b| b == b'\n')
            .filter(|&l| !l.is_empty())
            .map(|l| {
                l.splitn(3, |&b| b == b',')
                    .map(parse::<Number>)
                    .collect_tuple::<Pos>()
                    .expect("failed to parse position tuple")
            }),
    );

    let mut closest = BTreeMap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let (box1, box2) = (boxes[i], boxes[j]);
            let dist = distance(box1, box2);
            let last_dist;
            if let Some(entry) = closest.last_entry() {
                last_dist = *entry.key();
            } else {
                last_dist = Number::MAX;
            }
            if dist < last_dist {
                if closest.len() == num_connections {
                    closest.pop_last();
                }
                let _old_value = closest.insert(dist, (i, j));
                #[cfg(debug_assertions)]
                if let Some(old_value) = _old_value {
                    println!(
                        "duplicate distances: {}, old pair: {:?}, new pair {:?}",
                        dist,
                        old_value,
                        (i, j)
                    );
                }
            }
        }
    }
    #[cfg(debug_assertions)]
    println!("closest: {:?}", closest);

    let mut circuit_to_cluster = [usize::MAX; 1000];
    let mut cluster_to_circuits: Vec<Vec<usize>> = Vec::with_capacity(1000);
    closest.values().for_each(|&(box1, box2)| {
        match (circuit_to_cluster[box1], circuit_to_cluster[box2]) {
            (usize::MAX, usize::MAX) => {
                // create new cluster with both box1 and box2
                let next_cluster = cluster_to_circuits.len();
                #[cfg(debug_assertions)]
                println!("new cluster {} {:?} {:?}", next_cluster, box1, box2);
                circuit_to_cluster[box1] = next_cluster;
                circuit_to_cluster[box2] = next_cluster;
                let mut new_circuits = Vec::with_capacity(16);
                new_circuits.push(box1);
                new_circuits.push(box2);
                cluster_to_circuits.push(new_circuits);
            }
            (cluster, usize::MAX) => {
                // add box2 to box1's cluster
                #[cfg(debug_assertions)]
                println!("add to cluster {}: {:?}", cluster, box2);
                circuit_to_cluster[box2] = cluster;
                cluster_to_circuits[cluster].push(box2);
            }
            (usize::MAX, cluster) => {
                // add box1 to box2's cluster
                #[cfg(debug_assertions)]
                println!("add to cluster {}: {:?}", cluster, box2);
                circuit_to_cluster[box1] = cluster;
                cluster_to_circuits[cluster].push(box1);
            }
            (cluster1, cluster2) if cluster1 != cluster2 => {
                // move box2's clusters to box1
                let (old_cluster, new_cluster) = (cluster1.max(cluster2), cluster1.min(cluster2));
                #[cfg(debug_assertions)]
                println!("merge clusters {} -> {}", old_cluster, new_cluster);
                cluster_to_circuits[old_cluster]
                    .iter()
                    .for_each(|&pos| circuit_to_cluster[pos] = new_cluster);
                let (new, old) = cluster_to_circuits.split_at_mut(old_cluster);
                new[new_cluster].append(&mut old[0]);
            }
            _ => {}
        }
    });
    #[cfg(debug_assertions)]
    println!(
        "cluster to circuits (max size {}): {:?}",
        cluster_to_circuits.len(),
        cluster_to_circuits
            .iter()
            .sorted_by_key(|&c| usize::MAX - c.len())
            .take(3)
            .collect::<Vec<_>>(),
    );
    cluster_to_circuits
        .iter()
        .map(|c| c.len())
        .k_largest(3)
        .product::<usize>()
}

pub fn main() {
    print!(
        "{} ",
        get_result(include_bytes!("../../inputs/day08.txt"), 1000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day08.example.txt"), 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day08.txt"), 1000);
        assert_eq!(result, 80446);
    }
}
