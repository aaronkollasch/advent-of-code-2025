use crate::common::parse;

pub fn id_is_invalid(id: usize) -> bool {
    let digits = id.to_string();
    let l = digits.len();
    if !l.is_multiple_of(2) {
        return false;
    }
    digits[..l / 2] == digits[l / 2..]
}

pub fn get_result() -> usize {
    include_bytes!("../../inputs/day02.txt")
        .split(|b| *b == b',')
        .flat_map(|r| {
            let (from, to) = r.split_once(|&x| x == b'-').unwrap();
            let (from, to) = (parse::<usize>(from), parse::<usize>(to));
            #[cfg(debug_assertions)]
            println!("{} -> {} = {}", from, to, to - from + 1);
            from..=to
        })
        .map(|id| if id_is_invalid(id) { id } else { 0 })
        .sum::<usize>()
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
        assert_eq!(result, 34826702005);
    }
}
