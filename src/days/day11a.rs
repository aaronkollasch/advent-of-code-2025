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

const YOU: NodeIndex = NodeIndex::from_le_bytes(*b"you ");
const OUT: NodeIndex = NodeIndex::from_le_bytes(*b"out ");

/*
 * aaa -> you -> bbb -> ddd -> ggg -> out
 *     |      \      X
 *     |       > ccc -> eee -> out
 *     |      |      \
 *     |      |       > fff -> out
 *     \      /      /
 *      > hhh --------> iii -> out
 *
 *  paths from out -> out: 1
 *  paths from eee -> out: 1
 *  paths from fff -> out: 1
 *  paths from ggg -> out: 1
 *  paths from iii -> out: 1
 *  paths from ddd -> out: 1
 *  paths from ccc -> out: 3 (ddd + eee + fff)
 *  paths from hhh -> out: 1
 *  paths from ccc -> out: 2 (ddd + eee)
 *  paths from you -> out: 5 (bbb + ccc)
 *  paths from aaa -> out: 6 (you + hhh)
 */

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
    while !current_nodes.is_empty() {
        current_nodes.drain().for_each(|n| unsafe {
            let node: *mut Node = map.get_mut(&n).unwrap() as *mut _;
            // step 1: collect packet count from current node
            let packet_count = (*node).packet_count;
            // step 2: clear packet count for current node
            (*node).packet_count = 0;
            // step 3: propagate packet count to output nodes
            (*node).outputs.iter().for_each(|o| {
                if let Some(node) = map.get_mut(o) {
                    node.packet_count += packet_count;
                    next_nodes.insert(*o);
                }
                if o == &end_node {
                    path_count += packet_count;
                }
            });
        });
        (current_nodes, next_nodes) = (next_nodes, current_nodes);
    }
    path_count
}

pub fn get_result(input: &[u8]) -> usize {
    let mut forward_map: NodeHashMap =
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
            forward_map.insert(
                current_node,
                Node {
                    outputs: output_nodes,
                    packet_count: 0,
                },
            );
        });

    count_paths(YOU, OUT, &mut forward_map)
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day11.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day11.example.txt"));
        assert_eq!(result, 5);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day11.txt"));
        assert_eq!(result, 508);
    }
}
