use crate::common::parse;

type Number = usize;
#[allow(unused)]
type Pos = (Number, Number);

#[allow(unused)]
fn area(pos1: Pos, pos2: Pos) -> Number {
    let dx = pos1.0.abs_diff(pos2.0) + 1;
    let dy = pos1.1.abs_diff(pos2.1) + 1;
    dx * dy
}

pub fn get_result(input: &[u8]) -> Number {
    let mut red_tiles = Vec::with_capacity(500);
    red_tiles.extend(
        input
            .split(|&b| b == b'\n')
            .filter(|&l| !l.is_empty())
            .map(|l| {
                let (x, y) = l.split_once(|&b| b == b',').unwrap();
                (parse::<Number>(x), parse::<Number>(y))
            }),
    );
    let center = red_tiles
        .iter()
        .fold((0usize, 0usize), |acc, &pos| (acc.0 + pos.0, acc.1 + pos.1));
    let center = (center.0 / red_tiles.len(), center.1 / red_tiles.len());
    let min_distance = (center.0 + center.1) * 3 / 5;
    #[cfg(debug_assertions)]
    println!("approximate center: {:?}", center);
    let mut largest_area = 0;
    for i in 0..red_tiles.len() - 1 {
        let pos1 = red_tiles[i];
        let (dx, dy) = (pos1.0.abs_diff(center.0), pos1.1.abs_diff(center.1));
        if dx + dy < min_distance {
            continue;
        }
        for j in i + 1..red_tiles.len() {
            let pos2 = red_tiles[j];
            let (dx, dy) = (pos2.0.abs_diff(pos1.0) + 1, pos2.1.abs_diff(pos1.1) + 1);
            if dx + dy < min_distance {
                continue;
            }
            let area = dx * dy;
            #[cfg(debug_assertions)]
            println!("area {} for {:?} - {:?}", area, pos1, pos2);
            largest_area = largest_area.max(area);
        }
    }
    largest_area
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day09.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day09.example.txt"));
        assert_eq!(result, 50);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day09.txt"));
        assert_eq!(result, 4758598740);
    }
}
