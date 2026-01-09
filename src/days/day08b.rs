use crate::common::parse;
use itertools::Itertools;

type Number = i64;
type Circuit = usize;

type Pos = (Number, Number, Number);

fn distance(pos1: Pos, pos2: Pos) -> Number {
    (pos1.0 - pos2.0).pow(2) + (pos1.1 - pos2.1).pow(2) + (pos1.2 - pos2.2).pow(2)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct DistancePair {
    dist: Number,
    box1: Circuit,
    box2: Circuit,
}

pub fn get_result(input: &[u8]) -> Number {
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

    let mut closest = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);
    for i in 0..boxes.len() as Circuit {
        for j in i + 1..boxes.len() as Circuit {
            let (pos1, pos2) = (boxes[i], boxes[j]);
            let dist = distance(pos1, pos2);
            if dist > 300_000_000 {
                continue;
            }
            closest.push(DistancePair {
                dist,
                box1: i,
                box2: j,
            });
        }
    }
    #[cfg(debug_assertions)]
    println!(
        "{} closest (capacity {})",
        closest.len(),
        closest.capacity()
    );
    if closest.len() > 7000 {
        closest.select_nth_unstable(7000);
        closest.truncate(7000);
    }
    closest.sort_unstable();
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
    let mut next_cluster: Cluster = 0;
    let mut num_connected: usize = 0;
    let mut num_clusters: usize = 0;
    let mut last_pair: (usize, usize) = (42, 42);
    #[cfg(debug_assertions)]
    let mut i_pair = 0usize;
    #[cfg(debug_assertions)]
    let _closest_len = closest.len();
    let _ = closest
        .into_iter()
        .take_while(|pair| {
            let (box1, box2) = (pair.box1, pair.box2);
            last_pair = (box1, box2);
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
                    num_connected += 2;
                    num_clusters += 1;
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
                    num_connected += 1;
                }
                (NULL_CLUSTER, cluster) => {
                    // add box1 to box2's cluster
                    #[cfg(debug_assertions)]
                    println!("add to cluster {}: {:?}", cluster, box2);
                    circuit_to_cluster[box1] = cluster;
                    let last_circuit_pointer = cluster_ends[cluster as usize];
                    circuit_to_next_circuit[last_circuit_pointer as usize] = box1 as Circuit;
                    cluster_ends[cluster as usize] = box1 as Circuit;
                    num_connected += 1;
                }
                (cluster1, cluster2) if cluster1 != cluster2 => {
                    // move box2's clusters to box1
                    let (old_cluster, new_cluster) =
                        (cluster1.max(cluster2), cluster1.min(cluster2));
                    #[cfg(debug_assertions)]
                    println!("merge clusters {} -> {}", old_cluster, new_cluster);
                    let mut cluster_pointer = cluster_starts[old_cluster as usize];
                    while cluster_pointer != NULL_CIRCUIT {
                        circuit_to_cluster[cluster_pointer as usize] = new_cluster;
                        cluster_pointer = circuit_to_next_circuit[cluster_pointer as usize];
                    }
                    circuit_to_next_circuit[cluster_ends[new_cluster as usize] as usize] =
                        cluster_starts[old_cluster as usize];
                    cluster_ends[new_cluster as usize] = cluster_ends[old_cluster as usize];
                    num_clusters -= 1;
                }
                _ => {}
            }
            let to_continue = num_connected < boxes.len() || num_clusters > 1;
            #[cfg(debug_assertions)]
            {
                println!(
                    "step {}/{}: {}/{} connected in {} cluster, continue? {}",
                    i_pair,
                    _closest_len,
                    num_connected,
                    boxes.len(),
                    num_clusters,
                    to_continue,
                );
                i_pair += 1;
            }
            to_continue
        })
        .last();
    boxes[last_pair.0].0 * boxes[last_pair.1].0
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
