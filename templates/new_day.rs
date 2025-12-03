// use crate::common::parse;

pub fn get_result() -> usize {
    include_bytes!("../../inputs/DAY.txt")
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .count()
}

pub fn main() {
    print!("{} ", get_result());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_result() {
        let result = get_result();
        assert_eq!(result, 1);
    }
}
