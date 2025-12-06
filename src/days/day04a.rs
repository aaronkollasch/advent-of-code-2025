use crate::common::Vec2;
use primitive_types::U256;
use u256_literal::u256;

type Number = usize;
type Pos = Vec2<Number>;

#[allow(dead_code)]
const MAX_COLS: usize = 139;
const MAX_ROWS: usize = 139;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Grid {
    vals: [U256; MAX_ROWS],
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            vals: [u256!(0); MAX_ROWS],
            width,
            height,
        }
    }

    fn get_val(&self, pos: Pos) -> bool {
        (self.vals[pos.y] >> pos.x) & u256!(1) > u256!(0)
    }

    fn set_val(&mut self, pos: Pos) {
        self.vals[pos.y] |= u256!(1) << pos.x;
    }
}

pub fn get_result(input: &[u8]) -> usize {
    let mut grid = Grid::new(MAX_COLS, MAX_ROWS);
    input
        .split(|b| *b == b'\n')
        .filter(|&l| !l.is_empty())
        .enumerate()
        .for_each(|(i_row, l)| {
            l.iter().enumerate().for_each(|(i_col, &v)| {
                if v == b'@' {
                    grid.set_val(Vec2 { x: i_col + 1, y: i_row + 1 });
                }
            });
        });

    let mut num_accessible = 0;
    for i_row in 1..MAX_ROWS - 1 {
        for i_col in 1..MAX_COLS - 1 {
            if grid.get_val(Vec2 { x: i_col, y: i_row }) {
                let mut num_neighbors = 0;
                if grid.get_val(Vec2 { x: i_col - 1, y: i_row - 1 }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col - 1, y: i_row }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col - 1, y: i_row + 1 }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col + 1, y: i_row - 1 }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col + 1, y: i_row }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col + 1, y: i_row + 1 }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col, y: i_row - 1 }) {
                    num_neighbors += 1;
                }
                if grid.get_val(Vec2 { x: i_col, y: i_row + 1 }) {
                    num_neighbors += 1;
                }
                if num_neighbors < 4 {
                    num_accessible += 1;
                }
            }
        }
    }
    num_accessible
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day04.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day04.example.txt"));
        assert_eq!(result, 13);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day04.txt"));
        assert_eq!(result, 1424);
    }
}
