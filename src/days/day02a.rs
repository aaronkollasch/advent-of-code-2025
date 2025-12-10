use crate::common::parse;

fn sum_first_n(n: usize) -> usize {
    n * (n + 1) / 2
}

fn sum_range(from: usize, to: usize) -> usize {
    sum_first_n(to) - sum_first_n(from - 1)
}

fn sum_invalid(from: usize, to: usize) -> usize {
    let min_len = from.ilog10() + 1;
    let max_len = to.ilog10() + 1;
    #[cfg(debug_assertions)]
    println!("{}-{} => lengths {}-{}", from, to, min_len, max_len);

    (min_len..=max_len)
        .filter(|&total_len| total_len.is_multiple_of(2))
        .map(|total_len| {
            let half_len = total_len / 2;
            let low_half = 10usize.pow(half_len - 1);
            let high_half = 10usize.pow(half_len) - 1;
            let mut min_repeat;
            if total_len > min_len {
                min_repeat = low_half;
            } else {
                min_repeat = from.div_floor(low_half * 10);
                if min_repeat * (10 * low_half + 1) < from {
                    min_repeat += 1;
                }
            }
            let mut max_repeat;
            if total_len < max_len {
                max_repeat = high_half;
            } else {
                max_repeat = to.div_floor(low_half * 10);
                if max_repeat * (10 * low_half + 1) > to {
                    max_repeat -= 1;
                }
            }
            #[cfg(debug_assertions)]
            println!(
                "{}-{} => {}{}-{}{} @ len {}",
                from, to, min_repeat, min_repeat, max_repeat, max_repeat, total_len
            );
            sum_range(min_repeat, max_repeat) * (10 * low_half + 1)
        })
        .sum::<_>()
}

pub fn get_result(input: &[u8]) -> usize {
    input
        .split(|b| *b == b',')
        .map(|r| r.split_once(|&b| b == b'-').unwrap())
        .map(|(from, to)| (parse::<usize>(from), parse::<usize>(to)))
        .map(|(from, to)| sum_invalid(from, to))
        .sum::<usize>()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day02.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day02.example.txt"));
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day02.txt"));
        assert_eq!(result, 34826702005);
    }
}
