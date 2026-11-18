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
    fft_packet_count: usize,
    dac_packet_count: usize,
    fft_dac_packet_count: usize,
}

type NodeHashSet = HashSet<NodeIndex, BuildHasherDefault<NoHashHasher<NodeIndex>>>;
type NodeHashMap = HashMap<NodeIndex, Node, BuildHasherDefault<NoHashHasher<NodeIndex>>>;

const SVR: NodeIndex = NodeIndex::from_le_bytes(*b"svr ");
const FFT: NodeIndex = NodeIndex::from_le_bytes(*b"fft ");
const DAC: NodeIndex = NodeIndex::from_le_bytes(*b"dac ");
const OUT: NodeIndex = NodeIndex::from_le_bytes(*b"out ");

fn count_paths(start_node: NodeIndex, end_node: NodeIndex, map: &mut NodeHashMap) -> usize {
    map.get_mut(&start_node).unwrap().packet_count = 1;

    let mut current_nodes: NodeHashSet =
        HashSet::with_capacity_and_hasher(32, BuildHasherDefault::default());
    let mut next_nodes: NodeHashSet =
        HashSet::with_capacity_and_hasher(32, BuildHasherDefault::default());
    current_nodes.insert(start_node);

    let mut path_count = 0;
    while !current_nodes.is_empty() {
        current_nodes.drain().for_each(|n| unsafe {
            let node: *mut Node = map.get_mut(&n).unwrap() as *mut _;
            // step 1: collect packet count from current node
            let packet_count = (*node).packet_count;
            let mut fft_packet_count = (*node).fft_packet_count;
            let mut dac_packet_count = (*node).dac_packet_count;
            let mut fft_dac_packet_count = (*node).fft_dac_packet_count;
            // step 1.5: update fft and dac packet counts
            if n == FFT {
                fft_packet_count = packet_count;
                fft_dac_packet_count = dac_packet_count;
            } else if n == DAC {
                dac_packet_count = packet_count;
                fft_dac_packet_count = fft_packet_count;
            }
            // step 2: clear packet count for current node
            (*node).packet_count = 0;
            (*node).fft_packet_count = 0;
            (*node).dac_packet_count = 0;
            (*node).fft_dac_packet_count = 0;
            // step 3: propagate packet count to output nodes
            (*node).outputs.iter().for_each(|o| {
                if let Some(node) = map.get_mut(o) {
                    node.packet_count += packet_count;
                    node.fft_packet_count += fft_packet_count;
                    node.dac_packet_count += dac_packet_count;
                    node.fft_dac_packet_count += fft_dac_packet_count;
                    next_nodes.insert(*o);
                }
                if *o == end_node {
                    path_count += fft_dac_packet_count;
                }
            });
        });
        (current_nodes, next_nodes) = (next_nodes, current_nodes);
    }
    path_count
}

pub fn get_result(input: &[u8]) -> usize {
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
                            fft_packet_count: 0,
                            dac_packet_count: 0,
                            fft_dac_packet_count: 0,
                        },
                    );
                }
            });
        });

    count_paths(OUT, SVR, &mut reverse_map)
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
