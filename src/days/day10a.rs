use crate::common::parse_u8;
use itertools::Itertools;

type State = u16;

fn factorize_target(target: State, buttons: Vec<State>) -> usize {
    for comb_depth in 1..=buttons.len() {
        if let Some(_m) = buttons.iter().combinations(comb_depth).find(|comb| {
            let mut comb_result = target;
            comb.iter().for_each(|&b| comb_result ^= b);
            #[cfg(debug_assertions)]
            if comb_result == 0 {
                println!("{} {}: {:?} = {}", target, comb_depth, comb, comb_result);
            }
            comb_result == 0
        }) {
            return comb_depth;
        }
    }
    usize::MAX
}

pub fn get_result(input: &[u8]) -> usize {
    input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let mut iter = l.split(|&c| c == b' ');
            let target: State = {
                iter.next()
                    .unwrap()
                    .iter()
                    .skip(1)
                    .rev()
                    .skip(1)
                    .fold(0, |acc, &c| (acc << 1) + { if c == b'#' { 1 } else { 0 } })
            };
            let mut buttons = iter.collect::<Vec<_>>();
            let _joltage = buttons.pop().unwrap();
            let buttons: Vec<State> = {
                buttons
                    .iter()
                    .map(|&b| {
                        b[1..b.len() - 1]
                            .split(|&c| c == b',')
                            .fold(0, |acc, num| acc | (1 << parse_u8(num)))
                    })
                    .collect()
            };
            #[cfg(debug_assertions)]
            println!("{:b} ({}), {:?}", target, target, buttons);

            factorize_target(target, buttons)
        })
        .sum()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day10.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day10.example.txt"));
        assert_eq!(result, 7);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day10.txt"));
        assert_eq!(result, 491);
    }
}
