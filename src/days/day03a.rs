// use crate::common::parse;

pub fn get_result() -> usize {
    include_bytes!("../../inputs/day03.txt")
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let mut highest_val = 0u8;
            let mut highest_pos = 0usize;
            let _ = l.iter()
                .take(l.len() - 1)
                .map(|c| c - b'0')
                .enumerate()
                .try_for_each(|(i, c)| {
                    if c > highest_val {
                        highest_val = c;
                        highest_pos = i;
                    }
                    if c == 9 {
                        return Err(())
                    }
                    Ok(())
                });
            highest_val as usize * 10
                + l[highest_pos + 1..].iter().map(|c| c - b'0').max().unwrap() as usize
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
        assert_eq!(result, 17408);
    }
}
