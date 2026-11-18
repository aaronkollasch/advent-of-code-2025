use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
};

use itertools::Itertools;
use nohash::NoHashHasher;

type NodeIndex = u32;

#[derive(Debug)]
struct Node {
    outputs: Vec<NodeIndex>,
    packet_count: usize,
}

type NodeHashSet = HashSet<NodeIndex, BuildHasherDefault<NoHashHasher<NodeIndex>>>;
type NodeHashMap = HashMap<NodeIndex, Node, BuildHasherDefault<NoHashHasher<NodeIndex>>>;

const SVR: NodeIndex = NodeIndex::from_le_bytes(*b"svr ");
const FFT: NodeIndex = NodeIndex::from_le_bytes(*b"fft ");
const DAC: NodeIndex = NodeIndex::from_le_bytes(*b"dac ");
const OUT: NodeIndex = NodeIndex::from_le_bytes(*b"out ");

fn count_paths(start_node: NodeIndex, end_node: NodeIndex, map: &mut NodeHashMap) -> usize {
    map.iter_mut().for_each(|(_k, v)| {
        v.packet_count = 0;
    });
    map.get_mut(&start_node).unwrap().packet_count = 1;

    let mut current_nodes: NodeHashSet =
        HashSet::with_capacity_and_hasher(32, BuildHasherDefault::default());
    let mut next_nodes: NodeHashSet =
        HashSet::with_capacity_and_hasher(32, BuildHasherDefault::default());
    current_nodes.insert(start_node);

    let mut path_count = 0;
    let mut packet_counts: HashMap<NodeIndex, usize, BuildHasherDefault<NoHashHasher<NodeIndex>>> =
        HashMap::with_capacity_and_hasher(1024, BuildHasherDefault::default());
    while !current_nodes.is_empty() {
        // step 1: collect packet counts from current nodes
        // step 2: clear packet counts for current notdes
        current_nodes.iter().for_each(|n| {
            let node = map.get_mut(n).unwrap();
            packet_counts.insert(*n, node.packet_count);
            node.packet_count = 0;
        });
        // step 3: propagate total output paths to next nodes
        // TODO: combine with steps 1 and 2
        current_nodes.iter().for_each(|n| {
            let outputs = map[n].outputs.clone();
            outputs.iter().for_each(|o| {
                if let Some(o_node) = map.get_mut(o) {
                    o_node.packet_count += packet_counts[n];
                    next_nodes.insert(*o);
                }
                if *o == end_node {
                    path_count += packet_counts[n];
                }
            });
        });
        // step 4: clear nodes and packet counts
        (current_nodes, next_nodes) = (next_nodes, current_nodes);
        next_nodes.clear();
        packet_counts.clear();

        // current_nodes.drain().for_each(|n| unsafe {
        //     let node: *mut Node = reverse_map.get_mut(&n).unwrap() as *mut _;
        //     let outputs = &(*node).outputs;
        //     next_nodes.extend(outputs.iter().filter(|o| reverse_map.contains_key(o)));
        //     let path_count = (*node).path_count;
        //     outputs.iter().for_each(|o| {
        //         if let Some(node) = reverse_map.get_mut(o) {
        //             #[cfg(debug_assertions)]
        //             println!("adding {} from node {} to node {}", path_count, n, o);
        //             node.path_count += path_count;
        //         }
        //         if o == &YOU {
        //             you_path_count += path_count;
        //         }
        //     });
        //     (*node).path_count = 0;
        // });
        // (current_nodes, next_nodes) = (next_nodes, current_nodes);
        // next_nodes.clear();
        // packet_counts.clear();
    }
    path_count
}

pub fn get_result(input: &[u8]) -> usize {
    // TODO: remove one of these
    let mut forward_map: NodeHashMap =
        HashMap::with_capacity_and_hasher(1024, BuildHasherDefault::default());
    let mut reverse_map: NodeHashMap =
        HashMap::with_capacity_and_hasher(1024, BuildHasherDefault::default());
    input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .for_each(|l| {
            let mut current_node = [b' '; 4];
            current_node[0..3].copy_from_slice(&l[0..3]);
            let current_node = NodeIndex::from_le_bytes(current_node);
            let chunks = l[5..].as_chunks::<4>();
            let mut output_nodes = chunks
                .0
                .iter()
                .map(|&c| NodeIndex::from_le_bytes(c))
                .collect_vec();
            output_nodes.push({
                let mut last_node = [b' '; 4];
                last_node[0..3].copy_from_slice(chunks.1);
                NodeIndex::from_le_bytes(last_node)
            });
            output_nodes.iter().for_each(|n| {
                if let Some(node) = reverse_map.get_mut(n) {
                    node.outputs.push(current_node);
                } else {
                    reverse_map.insert(
                        *n,
                        Node {
                            outputs: vec![current_node],
                            packet_count: 0,
                        },
                    );
                }
            });
            forward_map.insert(
                current_node,
                Node {
                    outputs: output_nodes,
                    packet_count: 0,
                },
            );
        });

    let svr_fft = count_paths(SVR, FFT, &mut forward_map);
    let svr_dac = count_paths(SVR, DAC, &mut forward_map);
    let fft_dac = count_paths(FFT, DAC, &mut forward_map);
    let dac_fft = count_paths(DAC, FFT, &mut forward_map);
    let dac_out = count_paths(DAC, OUT, &mut forward_map);
    let fft_out = count_paths(FFT, OUT, &mut forward_map);
    // TODO: alternate: calculate svr -> out minus svr -> out excluding fft & dac
    svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day11.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day11.example2.txt"));
        assert_eq!(result, 2);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day11.txt"));
        assert_eq!(result, 315116216513280);
    }
}
