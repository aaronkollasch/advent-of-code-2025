// use crate::common::parse;

pub fn get_result() -> usize {
    let result = include_bytes!("../../inputs/DAY.txt")
        .split(|b| *b == b'\n')
        .count();
    return result;
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
