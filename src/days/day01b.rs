use crate::common::parse;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn get_result() -> u32 {
    let num_top = 3;
    let mut top_elves: BinaryHeap<Reverse<u32>> = BinaryHeap::with_capacity(num_top + 1);
    for _ in 0..num_top {
        top_elves.push(Reverse::<u32>(0));
    }

    let mut elf_total = 0u32;
    include_bytes!("../../inputs/day01.txt")
        .split(|b| *b == b'\n')
        .for_each(|l| {
            if l.is_empty() {
                top_elves.push(Reverse::<u32>(elf_total));
                top_elves.pop();
                elf_total = 0;
            } else {
                elf_total += parse::<u32>(l);
            }
        });
    return top_elves.iter().map(|e| e.0).sum::<u32>();
}

pub fn main() {
    print!("{} ", get_result());
}
