use crate::common::parse;

const NUM_POINTS: u8 = 100;

enum Rotation {
    Left(u8),
    Right(u8),
}

pub fn get_result() -> usize {
    let mut dial_state = 50u8;
    return include_bytes!("../../inputs/day01.txt")
        .split(|b| *b == b'\n')
        .map(|l| match l[0] {
            b'L' => Rotation::Left((parse::<usize>(&l[1..]) % NUM_POINTS as usize) as u8),
            b'R' => Rotation::Right((parse::<usize>(&l[1..]) % NUM_POINTS as usize) as u8),
            _ => unreachable!(),
        })
        .map(|rot| {
            dial_state = (dial_state
                + match rot {
                    Rotation::Left(click) => NUM_POINTS - click,
                    Rotation::Right(click) => click,
                })
            .rem_euclid(NUM_POINTS);
            #[cfg(debug_assertions)]
            println!("{}", dial_state);
            match dial_state {
                0 => 1,
                _ => 0,
            }
        })
        .sum::<usize>();
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
        assert_eq!(result, 1043);
    }
}
