use crate::common::{parse, parse_signed};

enum Direction {
    Left,
    Right,
}

type UNumber = u32;
const UNUM_POINTS: UNumber = 100;

struct Instruction {
    direction: Direction,
    clicks_rem: UNumber,
    clicks_div: UNumber,
}

// TODO: add function that calculates num crossings
// and resulting state given initial state and rotation
// and add tests


pub fn get_result_unsigned(input: &[u8]) -> UNumber {
    let mut dial_state: UNumber = 50;
    input
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let clicks = parse::<UNumber>(&l[1..]);
            let (clicks_div, clicks_rem) = (clicks / UNUM_POINTS, clicks % UNUM_POINTS);
            let inst = match l[0] {
                b'L' => Instruction { direction: Direction::Left, clicks_rem, clicks_div},
                b'R' => Instruction { direction: Direction::Right, clicks_rem, clicks_div},
                _ => unreachable!(),
            };
            let new_dial_state = (dial_state + match inst.direction {
                Direction::Left => UNUM_POINTS - inst.clicks_rem,
                Direction::Right => inst.clicks_rem,
            }).rem_euclid(UNUM_POINTS);
            let num_crossings = inst.clicks_div + match inst.direction {
                Direction::Left if dial_state == 0 => 0,
                Direction::Left if inst.clicks_rem >= dial_state => 1,
                Direction::Left => 0,
                Direction::Right if inst.clicks_rem + dial_state >= UNUM_POINTS => 1,
                Direction::Right => 0,
            };
            #[cfg(debug_assertions)]
            println!(
                "{} + {}=({} {}) -> {} {}",
                dial_state,
                str::from_utf8(l).unwrap(),
                inst.clicks_rem,
                inst.clicks_div,
                new_dial_state,
                num_crossings,
            );
            dial_state = new_dial_state;
            num_crossings
        })
        .sum::<UNumber>()
}

type Number = i32;

const NUM_POINTS: Number = 100;

enum Rotation {
    Left(Number),
    Right(Number),
}

pub fn get_result(input: &[u8]) -> Number {
    let mut dial_state: Number = 50;
    input
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
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
            let num_crossings = match rot {
                Rotation::Left(_) => (dial_state - 1).div_euclid(NUM_POINTS) - (new_dial_state - 1).div_euclid(NUM_POINTS),
                Rotation::Right(_) => new_dial_state.div_euclid(NUM_POINTS),
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
        .sum::<Number>()
}

pub fn get_result_brute(input: &[u8]) -> usize {
    let mut dial_state: Number = 50;
    input
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
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
        .count()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day01.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day01.example.txt"));
        assert_eq!(result, 6);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day01.txt"));
        assert_eq!(result, 5963);
    }

    #[test]
    fn correct_result_unsigned() {
        let result = get_result_unsigned(include_bytes!("../../inputs/day01.txt"));
        assert_eq!(result, 5963);
    }

    #[test]
    fn correct_result_brute() {
        let result = get_result_brute(include_bytes!("../../inputs/day01.txt"));
        assert_eq!(result, 5963);
    }
}
