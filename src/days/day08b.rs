use crate::common::parse;
use hashbrown::HashMap;
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct DistancePair {
    dist: Float,
    box1: usize,
    box2: usize,
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

    let mut closest = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let (pos1, pos2) = (boxes[i], boxes[j]);
            let dist = distance(pos1, pos2);
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

    let mut circuit_to_cluster: HashMap<usize, usize> = HashMap::with_capacity(1000);
    let mut cluster_to_circuits: HashMap<usize, Vec<usize>> = HashMap::with_capacity(1000);
    let mut next_cluster: usize = 0;
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
            match (circuit_to_cluster.get(&box1), circuit_to_cluster.get(&box2)) {
                (Some(&new_cluster), Some(&old_cluster)) if new_cluster != old_cluster => {
                    // move box2's clusters to box1
                    let mut box2_circuits = cluster_to_circuits.remove(&old_cluster).unwrap();
                    box2_circuits
                        .iter()
                        .for_each(|pos| *circuit_to_cluster.get_mut(pos).unwrap() = new_cluster);
                    #[cfg(debug_assertions)]
                    println!("merge clusters {} -> {}", old_cluster, new_cluster);
                    cluster_to_circuits
                        .get_mut(&new_cluster)
                        .unwrap()
                        .append(&mut box2_circuits);
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
            {
                println!(
                    "step {}/{}: {}/{} connected in {} cluster, continue? {}",
                    i_pair,
                    _closest_len,
                    circuit_to_cluster.len(),
                    boxes.len(),
                    cluster_to_circuits.len(),
                    circuit_to_cluster.len() < boxes.len() || cluster_to_circuits.len() > 1
                );
                i_pair += 1;
            }
            circuit_to_cluster.len() < boxes.len() || cluster_to_circuits.len() > 1
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
