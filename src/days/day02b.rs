use crate::common::parse;

fn sum_first_n(n: usize) -> usize {
    n * (n + 1) / 2
}

fn sum_range(from: usize, to: usize) -> usize {
    sum_first_n(to) - sum_first_n(from - 1)
}

fn sum_range_len_rep(
    from: usize,
    to: usize,
    min_len: u32,
    max_len: u32,
    total_len: u32,
    rep_len: u32,
) -> usize {
    let rep_count = total_len / rep_len;
    let low_rep = 10usize.pow(rep_len - 1);
    let high_rep = 10usize * low_rep - 1;
    #[cfg(debug_assertions)]
    println!("testing repeats of {}-{}", low_rep, high_rep);
    let mut min_repeat;
    if total_len > min_len {
        min_repeat = low_rep;
        #[cfg(debug_assertions)]
        println!("{} = min", min_repeat);
    } else {
        min_repeat = from.div_floor((low_rep * 10).pow(rep_count - 1));
        #[cfg(debug_assertions)]
        println!(
            "{} = {} / {}",
            min_repeat,
            from,
            (low_rep * 10).pow(rep_count - 1)
        );
        let repeat_start =
            (0..rep_count - 1).fold(min_repeat, |acc, _| acc * 10 * low_rep + min_repeat);
        if repeat_start < from {
            #[cfg(debug_assertions)]
            println!(
                "{} (or {}) < from {}",
                repeat_start,
                min_repeat.to_string().repeat(rep_count as usize),
                from
            );
            min_repeat += 1;
        };
    }
    let mut max_repeat;
    if total_len < max_len {
        max_repeat = high_rep;
        #[cfg(debug_assertions)]
        println!("{} = max", max_repeat);
    } else {
        max_repeat = to.div_floor((low_rep * 10).pow(rep_count - 1));
        #[cfg(debug_assertions)]
        println!(
            "{} = {} / {}",
            max_repeat,
            to,
            (low_rep * 10).pow(rep_count - 1)
        );
        let repeat_end =
            (0..rep_count - 1).fold(max_repeat, |acc, _| acc * 10 * low_rep + max_repeat);
        if repeat_end > to {
            #[cfg(debug_assertions)]
            println!(
                "{} (or {}) > to {}",
                repeat_end,
                max_repeat.to_string().repeat(rep_count as usize),
                to
            );
            max_repeat -= 1
        };
    }
    #[cfg(debug_assertions)]
    println!(
        "{}-{} => {}-{} @ len {}, rep {}",
        from,
        to,
        min_repeat.to_string().repeat(rep_count as usize),
        max_repeat.to_string().repeat(rep_count as usize),
        total_len,
        rep_len
    );
    if max_repeat < min_repeat {
        return 0;
    }
    let ret = sum_range(min_repeat, max_repeat);
    (0..rep_count - 1).fold(ret, |acc, _| acc * 10 * low_rep + ret)
}

fn sum_len(from: usize, to: usize, min_len: u32, max_len: u32, total_len: u32) -> usize {
    let mut total = 0;
    let mut counts_arr: [usize; 7] = [0; 7];
    (1..=total_len / 2)
        .filter(|&rep| total_len.is_multiple_of(rep))
        .for_each(|rep| {
            let mut total_rep = sum_range_len_rep(from, to, min_len, max_len, total_len, rep);
            if total_rep > 0 {
                (1..rep).filter(|&l| rep.is_multiple_of(l)).for_each(|l| {
                    #[cfg(debug_assertions)]
                    println!("subtracting {}", counts_arr[l as usize]);
                    total_rep -= counts_arr[l as usize];
                });
            }
            #[cfg(debug_assertions)]
            println!("adding {}", total_rep);
            counts_arr[rep as usize] = total_rep;
            total += total_rep;
        });
    #[cfg(debug_assertions)]
    println!("SUM_LEN {}-{}, l={}, total={}", from, to, total_len, total);
    total
}

fn sum_invalid(from: usize, to: usize) -> usize {
    let min_len = from.ilog10() + 1;
    let max_len = to.ilog10() + 1;
    #[cfg(debug_assertions)]
    println!("{}-{} => lengths {}-{}", from, to, min_len, max_len);

    (min_len..=max_len)
        .map(|total_len| sum_len(from, to, min_len, max_len, total_len))
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
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day02.txt"));
        assert_eq!(result, 43287141963);
    }
}
