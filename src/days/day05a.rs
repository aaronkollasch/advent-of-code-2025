use crate::common::parse;
use core::range::RangeInclusive;

pub fn get_result(input: &[u8]) -> usize {
    let database = input;
    let split_index = database.windows(2).position(|c| c == *b"\n\n").unwrap();
    let fresh_ranges: Vec<RangeInclusive<usize>> = database[0..split_index]
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let (start, last) = l.split_once(|&x| x == b'-').unwrap();
            let (start, last) = (parse::<usize>(start), parse::<usize>(last));
            RangeInclusive { start, last }
        })
        .collect();
    let ingredient_ids = &database[split_index + 2..];
    ingredient_ids
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| parse::<usize>(l))
        .filter(|id| fresh_ranges.iter().any(|&r| r.contains(id)))
        .count()
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
        assert_eq!(result, 3);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day05.txt"));
        assert_eq!(result, 811);
    }
}
