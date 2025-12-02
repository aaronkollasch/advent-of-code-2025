use crate::common::parse_signed;

type Number = i32;

const NUM_POINTS: Number = 100;

// TODO: change to struct with direction + num clicks
enum Rotation {
    Left(Number),
    Right(Number),
}

// TODO: add function that calculates num crossings
// and resulting state given initial state and rotation
// and add tests

pub fn get_result() -> Number {
    let mut dial_state: Number = 50;
    return include_bytes!("../../inputs/day01.txt")
        .split(|b| *b == b'\n')
        .filter(|&l| l.len() > 0)
        .map(|l| {
            let rot = match l[0] {
                b'L' => Rotation::Left(parse_signed::<Number>(&l[1..])),
                b'R' => Rotation::Right(parse_signed::<Number>(&l[1..])),
                _ => unreachable!(),
            };
            let new_dial_state = dial_state
                + match rot {
                    Rotation::Left(clicks) => -clicks,
                    Rotation::Right(clicks) => clicks,
                };
            let num_crossings = match new_dial_state > 0 {
                true => new_dial_state.div_floor(NUM_POINTS),
                false if dial_state == 0 => (-new_dial_state).div_floor(NUM_POINTS),
                false => (-new_dial_state).div_floor(NUM_POINTS) + 1,
            };
            dial_state = new_dial_state.rem_euclid(NUM_POINTS);
            #[cfg(debug_assertions)]
            println!(
                "{} {} {}",
                str::from_utf8(l).unwrap(),
                dial_state,
                num_crossings
            );
            num_crossings
        })
        .sum::<Number>();
}

pub fn get_result_brute() -> usize {
    let mut dial_state: Number = 50;
    return include_bytes!("../../inputs/day01.txt")
        .split(|b| *b == b'\n')
        .filter(|&l| l.len() > 0)
        .map(|l| match l[0] {
            b'L' => Rotation::Left(parse_signed::<Number>(&l[1..])),
            b'R' => Rotation::Right(parse_signed::<Number>(&l[1..])),
            _ => unreachable!(),
        })
        .flat_map(|rot| {
            let clicks = match rot {
                Rotation::Left(clicks) => -clicks,
                Rotation::Right(clicks) => clicks,
            };
            let num_steps = clicks.abs();
            let states = std::iter::chain(
                std::iter::repeat_n(
                    match rot {
                        Rotation::Left(_) => Some(-1),
                        Rotation::Right(_) => Some(1),
                    },
                    num_steps as usize,
                ),
                std::iter::once(None),
            )
            .scan(dial_state, |state, m| match m {
                Some(m) => {
                    *state += m;
                    *state = state.rem_euclid(NUM_POINTS);
                    Some(*state)
                }
                None => None,
            });
            dial_state = (dial_state + clicks).rem_euclid(NUM_POINTS);
            states
        })
        .filter(|&s| s == 0)
        .count();
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
        assert_eq!(result, 5963);
    }

    #[test]
    fn correct_result_brute() {
        let result = get_result_brute();
        assert_eq!(result, 5963);
    }
}
