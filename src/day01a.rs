use crate::common::parse;
use std::cmp::max;

pub fn main() {
    let mut elf_total = 0u32;
    let mut max_elf = 0u32;
    include_bytes!("../inputs/day01.txt")
        .split(|b| *b == b'\n')
        .for_each(|l| {
            if l.is_empty() {
                max_elf = max(max_elf, elf_total);
                elf_total = 0;
            } else {
                elf_total += parse::<u32>(l);
            }
        });
    print!("{} ", max_elf);
}
