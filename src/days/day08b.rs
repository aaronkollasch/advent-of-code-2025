use std::collections::{BTreeMap, HashMap};

use crate::common::parse;
use itertools::Itertools;
use ordered_float::OrderedFloat;

type Number = isize;

type Pos = (Number, Number, Number);

type Float = OrderedFloat<f32>;

fn distance(pos1: Pos, pos2: Pos) -> Float {
    Float::from(
        (((pos1.0 - pos2.0).pow(2) + (pos1.1 - pos2.1).pow(2) + (pos1.2 - pos2.2).pow(2)) as f32)
            .sqrt(),
    )
}

pub fn get_result(input: &[u8]) -> isize {
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
    #[cfg(debug_assertions)]
    println!("num boxes: {}", boxes.len());

    // TODO: change from (pos1, pos2) to (i, j)?
    let mut closest: BTreeMap<Float, Vec<(Pos, Pos)>> = BTreeMap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let (pos1, pos2) = (boxes[i], boxes[j]);
            let dist = distance(pos1, pos2);
            if let Some(pairs) = closest.get_mut(&dist) {
                pairs.push((pos1, pos2));
            } else {
                closest.insert(dist, vec![(pos1, pos2)]);
            }
        }
    }
    let mut circuit_to_cluster: HashMap<Pos, usize> = HashMap::with_capacity(1000);
    let mut cluster_to_circuits: HashMap<usize, Vec<Pos>> = HashMap::with_capacity(1000);
    let mut next_cluster: usize = 0;
    let mut last_pair: (Pos, Pos) = ((42, 42, 42), (42, 42, 42));
    let _ = closest
        .values()
        .take_while(|pairs| {
            pairs.into_iter().for_each(|&(pos1, pos2)| {
                last_pair = (pos1, pos2);
                match (circuit_to_cluster.get(&pos1), circuit_to_cluster.get(&pos2)) {
                    (Some(&new_cluster), Some(&old_cluster)) if new_cluster != old_cluster => {
                        // move pos2's clusters to pos1
                        let mut pos2_circuits = cluster_to_circuits.remove(&old_cluster).unwrap();
                        pos2_circuits.iter().for_each(|pos| {
                            *circuit_to_cluster.get_mut(&pos).unwrap() = new_cluster
                        });
                        #[cfg(debug_assertions)]
                        println!("merge clusters {} -> {}", old_cluster, new_cluster);
                        cluster_to_circuits
                            .get_mut(&new_cluster)
                            .unwrap()
                            .append(&mut pos2_circuits);
                    }
                    (Some(_), Some(_)) => {}
                    (Some(&cluster), None) => {
                        // add pos2 to pos1's cluster
                        circuit_to_cluster.insert(pos2, cluster);
                        cluster_to_circuits.get_mut(&cluster).unwrap().push(pos2);
                        #[cfg(debug_assertions)]
                        println!("add to cluster {}: {:?}", cluster, pos2);
                    }
                    (None, Some(&cluster)) => {
                        // add pos1 to pos2's cluster
                        circuit_to_cluster.insert(pos1, cluster);
                        cluster_to_circuits.get_mut(&cluster).unwrap().push(pos1);
                        #[cfg(debug_assertions)]
                        println!("add to cluster {}: {:?}", cluster, pos2);
                    }
                    (None, None) => {
                        // create new cluster with both pos1 and pos2
                        circuit_to_cluster.insert(pos1, next_cluster);
                        circuit_to_cluster.insert(pos2, next_cluster);
                        cluster_to_circuits.insert(next_cluster, vec![pos1, pos2]);
                        #[cfg(debug_assertions)]
                        println!("new cluster {} {:?} {:?}", next_cluster, pos1, pos2);
                        next_cluster += 1;
                    }
                }
            });
            #[cfg(debug_assertions)]
            println!(
                "{}/{} connected in {} cluster, continue? {}",
                circuit_to_cluster.len(),
                boxes.len(),
                cluster_to_circuits.len(),
                circuit_to_cluster.len() < boxes.len() || cluster_to_circuits.len() > 1
            );
            circuit_to_cluster.len() < boxes.len() || cluster_to_circuits.len() > 1
        })
        .last();
    last_pair.0.0 * last_pair.1.0
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day08.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day08.example.txt"));
        assert_eq!(result, 25272);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day08.txt"));
        assert_eq!(result, 51294528);
    }
}
