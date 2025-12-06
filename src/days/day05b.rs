use crate::common::parse;
use core::range::RangeInclusive;
use std::cmp::max;

type Number = usize;

pub fn get_result(input: &[u8]) -> Number {
    let database = input;
    let split_index = database.windows(2).position(|c| c == *b"\n\n").unwrap();
    let mut fresh_ranges: Vec<RangeInclusive<Number>> = database[0..split_index]
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let (start, last) = l.split_once(|&x| x == b'-').unwrap();
            let (start, last) = (parse::<Number>(start), parse::<Number>(last));
            RangeInclusive { start, last }
        })
        .collect();
    fresh_ranges.sort_by_key(|r| (r.start, r.last));
    let mut num_fresh: Number = 0;
    let mut range_start: Number = fresh_ranges.first().unwrap().start;
    let mut range_end: Number = fresh_ranges.first().unwrap().last;
    #[cfg(debug_assertions)]
    println!("{:?} -> {} {}", fresh_ranges.first().unwrap(), range_start, range_end);
    fresh_ranges.iter().skip(1).for_each(|&r| {
        #[cfg(debug_assertions)]
        print!("{:?} {} {} -> ", r, range_start, range_end);
        if r.start > range_end + 1 {
            num_fresh += range_end - range_start + 1;
            range_start = r.start;
            range_end = r.last;
        } else {
            range_end = max(range_end, r.last);
        }
        #[cfg(debug_assertions)]
        println!("{} {}", range_start, range_end);
    });
    num_fresh += range_end - range_start + 1;
    num_fresh
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day05.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day05.example.txt"));
        assert_eq!(result, 14);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day05.txt"));
        assert_eq!(result, 338189277144473);
    }
}
