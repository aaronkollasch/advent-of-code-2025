use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
};

// use crate::common::parse;
use itertools::Itertools;
use nohash::NoHashHasher;

const YOU: u32 = u32::from_le_bytes([b'y', b'o', b'u', b' ']);
const OUT: u32 = u32::from_le_bytes([b'o', b'u', b't', b' ']);

#[derive(Debug)]
struct Node {
    outputs: Vec<u32>,
    path_count: usize,
    packet_count: usize,
}

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

pub fn get_result(input: &[u8]) -> usize {
    #[cfg(debug_assertions)]
    println!("{} {}", YOU, OUT);
    let mut forward_map: HashMap<u32, Node, BuildHasherDefault<NoHashHasher<u32>>> =
        HashMap::with_capacity_and_hasher(1024, BuildHasherDefault::default());
    let mut reverse_map: HashMap<u32, Node, BuildHasherDefault<NoHashHasher<u32>>> =
        HashMap::with_capacity_and_hasher(1024, BuildHasherDefault::default());
    input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .for_each(|l| {
            let mut current_node = [b' '; 4];
            current_node[0..3].copy_from_slice(&l[0..3]);
            let current_node = u32::from_le_bytes(current_node);
            let chunks = l[5..].as_chunks::<4>();
            let mut output_nodes = chunks
                .0
                .into_iter()
                .map(|&c| u32::from_le_bytes(c))
                .collect_vec();
            output_nodes.push({
                let mut last_node = [b' '; 4];
                last_node[0..3].copy_from_slice(chunks.1);
                u32::from_le_bytes(last_node)
            });
            #[cfg(debug_assertions)]
            println!("{:?}: {:?}", current_node, output_nodes);
            output_nodes.iter().for_each(|n| {
                if let Some(node) = reverse_map.get_mut(n) {
                    node.outputs.push(current_node);
                } else {
                    reverse_map.insert(
                        *n,
                        Node {
                            outputs: vec![current_node],
                            path_count: 0,
                            packet_count: 0,
                        },
                    );
                }
            });
            forward_map.insert(
                current_node,
                Node {
                    outputs: output_nodes,
                    path_count: 0,
                    packet_count: 0,
                },
            );
        });

    reverse_map.get_mut(&OUT).unwrap().path_count = 1;
    reverse_map.get_mut(&OUT).unwrap().packet_count = 1;
    #[cfg(debug_assertions)]
    println!("YOU (fwd): {:?}", forward_map[&YOU]);
    #[cfg(debug_assertions)]
    println!("OUT (rev): {:?}", reverse_map[&OUT]);

    let mut current_nodes: HashSet<u32, BuildHasherDefault<NoHashHasher<u32>>> =
        HashSet::with_capacity_and_hasher(32, BuildHasherDefault::default());
    let mut next_nodes: HashSet<u32, BuildHasherDefault<NoHashHasher<u32>>> =
        HashSet::with_capacity_and_hasher(32, BuildHasherDefault::default());
    current_nodes.insert(OUT);

    let mut packet_counts: HashMap<u32, usize, BuildHasherDefault<NoHashHasher<u32>>> =
        HashMap::with_capacity_and_hasher(1024, BuildHasherDefault::default());
    while !current_nodes.is_empty() {
        // step 1: collect packet counts from current nodes
        // step 2: clear packet counts for current notdes
        current_nodes.iter().for_each(|n| {
            let node = reverse_map.get_mut(n).unwrap();
            packet_counts.insert(*n, node.packet_count);
            node.packet_count = 0;
        });
        // step 3: propagate total output paths to next nodes
        current_nodes.iter().for_each(|n| {
            let outputs = reverse_map[n].outputs.clone();
            outputs.iter().for_each(|o| {
                if let Some(o_node) = reverse_map.get_mut(o) {
                    o_node.packet_count += packet_counts[n];
                    o_node.path_count += packet_counts[n];
                    next_nodes.insert(*o);
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
    reverse_map[&YOU].path_count as usize
}

pub fn main() {
    print!(
        "{} ",
        get_result(include_bytes!("../../inputs/day11.txt"))
    );
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
