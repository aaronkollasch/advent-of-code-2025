use itertools::Itertools;
use crate::common::parse_iter;

type Number = usize;

pub fn get_result() -> usize {
    let mut lines: Vec<&[u8]> = include_bytes!("../../inputs/day06.txt")
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .collect();
    let operations = lines.pop().unwrap();
    operations.iter()
        .enumerate()
        .map(|(i, &op)| (i, op))
        .filter(|&(_, op)| op != b' ')
        .chain([(operations.len() + 1, b' ')].into_iter())
        .tuple_windows::<(_, _)>()
        .map(|((i, op), (next_i, _))| {
            let first_column = i;
            let end_column = next_i - 1;
            #[cfg(debug_assertions)]
            println!("{} @ {}..{}", std::str::from_utf8(&[op]).unwrap(), i, next_i);
            let values = (first_column..end_column)
                .map(|c| {
                    parse_iter::<Number, _>(lines.iter().map(|&l| l[c]).filter(|&b| b != b' '))
                });
            let result = match op {
                b'*' => values.product1::<Number>().unwrap(),
                b'+' => values.sum1().unwrap(),
                _ => unreachable!("unrecognized operation!"),
            };
            #[cfg(debug_assertions)]
            println!("{} -> {}", std::str::from_utf8(&[op]).unwrap(), result);
            result
        })
        .sum()
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
        assert_eq!(result, 10227753257799);
    }
}
