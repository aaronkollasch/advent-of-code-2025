// use crate::common::parse;
// use itertools::Itertools;

pub fn get_result(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            l
        })
        .count()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/DAY.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/DAY.example.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/DAY.txt"));
        assert_eq!(result, 1);
    }
}
