use crate::common::parse;

pub fn id_is_invalid(id: usize) -> bool {
    let digits_s = id.to_string();
    let digits = digits_s.as_bytes();
    let l = digits.len();
    for block_size in 1..=6 {
        if l == block_size || !l.is_multiple_of(block_size) {
            continue;
        }
        let mut chunks = digits.chunks(block_size);
        let first = chunks.next().unwrap();
        if chunks.all(|c| c == first) {
            return true;
        }
    }
    false
}

pub fn get_result(input: &[u8]) -> usize {
    input
        .split(|b| *b == b',')
        .flat_map(|r| {
            let (from, to) = r.split_once(|&x| x == b'-').unwrap();
            let (from, to) = (parse::<usize>(from), parse::<usize>(to));
            #[cfg(debug_assertions)]
            println!("{} -> {} = {}", from, to, to - from + 1);
            from..=to
        })
        .map(|id| {
            if id_is_invalid(id) {
                #[cfg(debug_assertions)]
                println!("{} is invalid", id);
                id
            } else {
                0
            }
        })
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
