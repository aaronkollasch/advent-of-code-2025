use std::collections::BTreeMap;

use crate::common::parse;
use hashbrown::HashMap;
use itertools::Itertools;
use ordered_float::OrderedFloat;

type Number = isize;

type Pos = (Number, Number, Number);

type Float = OrderedFloat<f32>;

fn distance(box1: Pos, box2: Pos) -> Float {
    Float::from(
        (((box1.0 - box2.0).pow(2) + (box1.1 - box2.1).pow(2) + (box1.2 - box2.2).pow(2)) as f32)
            .sqrt(),
    )
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
                last_dist = Float::from(1000000f32);
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

    let mut circuit_to_cluster: HashMap<usize, usize> = HashMap::with_capacity(1000);
    let mut cluster_to_circuits: HashMap<usize, Vec<usize>> = HashMap::with_capacity(1000);
    let mut next_cluster: usize = 0;
    #[cfg(debug_assertions)]
    let mut _cluster_count_hwm = 0;
    closest.values().for_each(|&(box1, box2)| {
        match (circuit_to_cluster.get(&box1), circuit_to_cluster.get(&box2)) {
            (Some(&new_cluster), Some(&old_cluster)) if new_cluster != old_cluster => {
                // move box2's clusters to box1
                let mut pos2_circuits = cluster_to_circuits.remove(&old_cluster).unwrap();
                pos2_circuits
                    .iter()
                    .for_each(|pos| *circuit_to_cluster.get_mut(pos).unwrap() = new_cluster);
                #[cfg(debug_assertions)]
                println!(
                    "merge clusters {} -> {}: {:?} + {:?}",
                    old_cluster, new_cluster, pos2_circuits, cluster_to_circuits[&new_cluster]
                );
                cluster_to_circuits
                    .get_mut(&new_cluster)
                    .unwrap()
                    .append(&mut pos2_circuits);
            }
            (Some(_), Some(_)) => {}
            (Some(&cluster), None) => {
                // add box2 to box1's cluster
                circuit_to_cluster.insert(box2, cluster);
                cluster_to_circuits.get_mut(&cluster).unwrap().push(box2);
                #[cfg(debug_assertions)]
                println!("add to cluster {}: {:?}", cluster, box2);
            }
            (None, Some(&cluster)) => {
                // add box1 to box2's cluster
                circuit_to_cluster.insert(box1, cluster);
                cluster_to_circuits.get_mut(&cluster).unwrap().push(box1);
                #[cfg(debug_assertions)]
                println!("add to cluster {}: {:?}", cluster, box2);
            }
            (None, None) => {
                // create new cluster with both box1 and box2
                circuit_to_cluster.insert(box1, next_cluster);
                circuit_to_cluster.insert(box2, next_cluster);
                cluster_to_circuits.insert(next_cluster, vec![box1, box2]);
                #[cfg(debug_assertions)]
                println!("new cluster {} {:?} {:?}", next_cluster, box1, box2);
                next_cluster += 1;
            }
        }
        #[cfg(debug_assertions)]
        if cluster_to_circuits.len() > _cluster_count_hwm {
            _cluster_count_hwm = cluster_to_circuits.len();
        }
    });
    #[cfg(debug_assertions)]
    println!(
        "cluster to circuits (max size {}): {:?}",
        _cluster_count_hwm,
        cluster_to_circuits
            .iter()
            .sorted_by_key(|&(_, c)| usize::MAX - c.len())
            .take(3)
            .collect::<Vec<_>>(),
    );
    cluster_to_circuits
        .into_values()
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
