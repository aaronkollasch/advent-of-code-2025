type Number = usize;

pub fn get_result(input: &[u8]) -> Number {
    input
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let mut accum = 0;
            let mut highest_val = 0u8;
            let mut highest_pos = 0;
            for digits_place in (0..12).rev() {
                let mut new_highest_pos = highest_pos;
                let _ = l[highest_pos..l.len() - digits_place]
                    .iter()
                    // .take_while(|_| highest_val < 9)
                    .map(|c| c - b'0')
                    .enumerate()
                    .try_for_each(|(i, c)| {
                        if c > highest_val {
                            highest_val = c;
                            new_highest_pos = i;
                        }
                        if c == 9 {
                            return Err(());
                        }
                        Ok(())
                    });
                accum += highest_val as Number * (10 as Number).pow(digits_place as u32);
                #[cfg(debug_assertions)]
                println!(
                    "{} {} {} {} {}",
                    digits_place, highest_pos, new_highest_pos, highest_val, accum
                );
                highest_val = 0;
                highest_pos += new_highest_pos + 1;
            }
            #[cfg(debug_assertions)]
            println!("{} {}", std::str::from_utf8(l).unwrap(), accum);
            accum
        })
        .sum()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day03.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day03.example.txt"));
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day03.txt"));
        assert_eq!(result, 172740584266849);
    }
}
