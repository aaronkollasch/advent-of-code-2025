use std::collections::BTreeMap;

use crate::common::parse;
use itertools::Itertools;

type Number = i64;

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

    const MAX_CLUSTERS: usize = 300;
    type Cluster = u16;
    const NULL_CLUSTER: Cluster = Cluster::MAX;
    type Circuit = usize;
    const NULL_CIRCUIT: Circuit = Circuit::MAX;
    let mut circuit_to_cluster = [NULL_CLUSTER; 1000];
    let mut circuit_to_next_circuit: [Circuit; 1000] = [NULL_CIRCUIT; 1000];
    let mut cluster_starts: [Circuit; MAX_CLUSTERS] = [0; MAX_CLUSTERS];
    let mut cluster_ends: [Circuit; MAX_CLUSTERS] = [0; MAX_CLUSTERS];
    let mut cluster_sizes: [u16; MAX_CLUSTERS] = [0; MAX_CLUSTERS];
    let mut next_cluster: Cluster = 0;
    #[cfg(debug_assertions)]
    let mut max_num_clusters = 0usize;
    closest.values().for_each(|&(box1, box2)| {
        match (circuit_to_cluster[box1], circuit_to_cluster[box2]) {
            (NULL_CLUSTER, NULL_CLUSTER) => {
                // create new cluster with both box1 and box2
                #[cfg(debug_assertions)]
                println!("new cluster {} {:?} {:?}", next_cluster, box1, box2);
                circuit_to_cluster[box1] = next_cluster;
                circuit_to_cluster[box2] = next_cluster;
                circuit_to_next_circuit[box1] = box2 as Circuit;
                cluster_starts[next_cluster as usize] = box1 as Circuit;
                cluster_ends[next_cluster as usize] = box2 as Circuit;
                cluster_sizes[next_cluster as usize] = 2;
                next_cluster += 1;
            }
            (cluster, NULL_CLUSTER) => {
                // add box2 to box1's cluster
                #[cfg(debug_assertions)]
                println!("add to cluster {}: {:?}", cluster, box2);
                circuit_to_cluster[box2] = cluster;
                let last_circuit_pointer = cluster_ends[cluster as usize];
                circuit_to_next_circuit[last_circuit_pointer as usize] = box2 as Circuit;
                cluster_ends[cluster as usize] = box2 as Circuit;
                cluster_sizes[cluster as usize] += 1;
            }
            (NULL_CLUSTER, cluster) => {
                // add box1 to box2's cluster
                #[cfg(debug_assertions)]
                println!("add to cluster {}: {:?}", cluster, box2);
                circuit_to_cluster[box1] = cluster;
                let last_circuit_pointer = cluster_ends[cluster as usize];
                circuit_to_next_circuit[last_circuit_pointer as usize] = box1 as Circuit;
                cluster_ends[cluster as usize] = box1 as Circuit;
                cluster_sizes[cluster as usize] += 1;
            }
            (cluster1, cluster2) if cluster1 != cluster2 => {
                // move box2's clusters to box1
                let (old_cluster, new_cluster) = (cluster1.max(cluster2), cluster1.min(cluster2));
                #[cfg(debug_assertions)]
                println!("merge clusters {} -> {}", old_cluster, new_cluster);
                #[cfg(debug_assertions)]
                if cluster_sizes[old_cluster as usize] > cluster_sizes[new_cluster as usize] {
                    println!(
                        "old cluster was larger: moving {} into {}",
                        cluster_sizes[old_cluster as usize], cluster_sizes[new_cluster as usize]
                    );
                }
                let mut cluster_pointer = cluster_starts[old_cluster as usize];
                while cluster_pointer != NULL_CIRCUIT {
                    circuit_to_cluster[cluster_pointer as usize] = new_cluster;
                    cluster_pointer = circuit_to_next_circuit[cluster_pointer as usize];
                }
                circuit_to_next_circuit[cluster_ends[new_cluster as usize] as usize] =
                    cluster_starts[old_cluster as usize];
                cluster_ends[new_cluster as usize] = cluster_ends[old_cluster as usize];
                cluster_sizes[new_cluster as usize] += cluster_sizes[old_cluster as usize];
                cluster_sizes[old_cluster as usize] = 0;
            }
            _ => {}
        }
        #[cfg(debug_assertions)]
        {
            let num_clusters = cluster_sizes.iter().filter(|&&size| size > 0).count();
            if num_clusters > max_num_clusters {
                max_num_clusters = num_clusters;
            }
        }
    });
    #[cfg(debug_assertions)]
    println!(
        "cluster to circuits (max size {}, max num {}): {:?}",
        next_cluster,
        max_num_clusters,
        cluster_sizes
            .iter()
            .sorted_by_key(|&&size| u16::MAX - size)
            .take(3)
            .collect::<Vec<_>>(),
    );
    cluster_sizes
        .iter()
        .k_largest(3)
        .map(|&s| s as usize)
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
