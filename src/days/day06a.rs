use itertools::Itertools;
use crate::common::parse_iter;

type Number = usize;

pub fn get_result(input: &[u8]) -> usize {
    let mut lines: Vec<&[u8]> = input
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
            #[cfg(debug_assertions)]
            println!("{} @ {}..{}", std::str::from_utf8(&[op]).unwrap(), i, next_i);
            let values = lines.iter().map(|&l| {
                #[cfg(debug_assertions)]
                println!("{:?}", std::str::from_utf8(&l[i..next_i-1]));
                parse_iter::<Number, _>(l[i..next_i-1].iter().filter(|&&b| b != b' ').map(|&b| b))
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
    print!("{} ", get_result(include_bytes!("../../inputs/day06.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day06.example.txt"));
        assert_eq!(result, 4277556);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day06.txt"));
        assert_eq!(result, 5227286044585);
    }
}
