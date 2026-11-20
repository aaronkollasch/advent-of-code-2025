use std::fmt;
use std::iter::zip;

use crate::common::parse;
use hashbrown::HashSet;
use itertools::Itertools;
use num::Integer;

type Present = [[u8; 3]; 3];

fn rotate_ninety(present: &mut Present) {
    for i in 0..3 {
        for j in i + 1..3 {
            (present[i][j], present[j][i]) = (present[j][i], present[i][j])
        }
    }
    for row_i in present.iter_mut().take(3) {
        for j in 0..3 / 2 {
            (row_i[j], row_i[3 - 1 - j]) = (row_i[3 - 1 - j], row_i[j])
        }
    }
}

fn flip(present: &mut Present) {
    for row_i in present.iter_mut().take(3) {
        for j in 0..3 {
            (row_i[j], row_i[3 - 1 - j]) = (row_i[3 - 1 - j], row_i[j])
        }
    }
}

const MAX_HEIGHT: usize = 64;
const MAX_WIDTH: usize = 64;
const MAP_SIZE: usize = MAX_WIDTH * MAX_HEIGHT;
const MAP_FILL: u8 = b'#';
const MAP_EMPTY: u8 = b' ';
const NUM_PRESENTS: usize = 6;

#[derive(Default, Debug)]
struct Region {
    width: u8,
    length: u8,
    num_presents: [u8; NUM_PRESENTS],
}

struct Map {
    width: u8,
    length: u8,
    contents: [u8; MAP_SIZE],
    edge_positions: HashSet<(u8, u8)>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Map {}x{} {{", self.width, self.length);
        let _ = writeln!(f, "{}", "|".repeat(self.width as usize + 2));
        for row in 0..self.length {
            let _ = write!(f, "{}", b'|' as char);
            for col in 0..self.width {
                let val = match (
                    self.get_val((col, row)),
                    self.edge_positions.contains(&(col, row)),
                ) {
                    (MAP_FILL, false) => MAP_FILL,
                    (MAP_EMPTY, true) => b'-',
                    (MAP_EMPTY, false) => MAP_EMPTY,
                    _ => b'?',
                };
                let _ = write!(f, "{}", val as char);
            }
            let _ = write!(f, "{}", b'|' as char);
            let _ = writeln!(f);
        }
        let _ = writeln!(f, "{}", "|".repeat(self.width as usize + 2));
        let _ = writeln!(f, "}}");
        Ok(())
    }
}

impl Map {
    pub fn new(width: u8, length: u8) -> Self {
        let mut edge_positions = HashSet::new();
        (0..8).for_each(|i| {
            edge_positions.insert((i, 0));
        });
        (0..3).for_each(|i| {
            edge_positions.insert((0, i));
        });
        Self {
            width,
            length,
            contents: [MAP_EMPTY; MAP_SIZE],
            edge_positions,
        }
    }

    #[inline]
    pub fn get_val(&self, pos: (u8, u8)) -> u8 {
        self.contents[pos.0 as usize * MAX_WIDTH + pos.1 as usize]
    }

    #[inline]
    pub fn set_val(&mut self, pos: (u8, u8), val: u8) {
        self.contents[pos.0 as usize * MAX_WIDTH + pos.1 as usize] = val;
    }

    fn check_present(&self, present: Present, x: u8, y: u8) -> u8 {
        let mut edge_positions_covered_count = 0;
        for i in 0..3 {
            for j in 0..3 {
                if present[i as usize][j as usize] == MAP_FILL {
                    let (x_i, y_i) = (x + i, y + j);
                    if self.get_val((x_i, y_i)) == MAP_FILL {
                        return 0;
                    }
                    if self.edge_positions.contains(&(x_i, y_i)) {
                        edge_positions_covered_count += 1;
                    }
                }
            }
        }
        edge_positions_covered_count
    }

    fn place_present(&mut self, present: Present, x: u8, y: u8) {
        for i in 0..3 {
            for j in 0..3 {
                if present[i as usize][j as usize] == MAP_FILL {
                    let (x_i, y_i) = (x + i, y + j);
                    self.set_val((x_i, y_i), MAP_FILL);
                    self.edge_positions.remove(&(x_i, y_i));
                }
            }
        }
    }

    fn add_edge_positions(&mut self, present: Present, x: u8, y: u8) {
        for (new_x, new_y) in [
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x, y + 1),
        ] {
            if new_x + 3 > MAX_WIDTH as u8 || new_y + 3 > MAX_HEIGHT as u8 {
                continue;
            }
            for i in 0..3 {
                for j in 0..3 {
                    let (x_i, y_i) = (new_x + i, new_y + j);
                    if present[i as usize][j as usize] == MAP_FILL
                        && self.get_val((x_i, y_i)) != MAP_FILL
                    {
                        self.edge_positions.insert((x_i, y_i));
                    }
                }
            }
        }
    }

    pub fn pick_place_present(&mut self, presents: &[Present]) -> Option<usize> {
        // step 1: for each edge position, check all presents + rotations, with a corner at that position;
        //     keep the one that covers the most edge positions.
        let mut best_position = (0, 0);
        let mut best_present: Present = Default::default();
        let mut best_present_i: Option<usize> = None;
        let mut best_present_edge_coverage: u8 = 0;
        self.edge_positions.iter().for_each(|&(x, y)| {
            if x + 2 >= self.width || y + 2 >= self.length {
                return;
            }
            let mut present: Present = Default::default();
            presents
                .iter()
                .enumerate()
                .for_each(|(i_present, new_present)| {
                    present = *new_present;
                    for i_transform in 0..8 {
                        let edge_coverage = self.check_present(present, x, y);
                        if edge_coverage > best_present_edge_coverage {
                            best_position = (x, y);
                            best_present = present;
                            best_present_i = Some(i_present);
                            best_present_edge_coverage = edge_coverage;
                        }
                        rotate_ninety(&mut present);
                        if i_transform == 3 {
                            flip(&mut present);
                        }
                    }
                });
        });
        // step 2: place the present at that position, remove any covered edge positions
        self.place_present(best_present, best_position.0, best_position.1);
        // step 3: add all unset neighboring positions to edge_positions
        self.add_edge_positions(best_present, best_position.0, best_position.1);
        best_present_i
    }
}

pub fn get_result_full(input: &[u8]) -> usize {
    let mut presents = [Present::default(); NUM_PRESENTS];
    let mut regions: Vec<Region> = Vec::with_capacity(128);
    let lines = input.split(|&b| b == b'\n').filter(|&l| !l.is_empty());

    lines.enumerate().for_each(|(i_line, line)| {
        if i_line < 24 {
            let (i_shape, i_line) = i_line.div_rem(&4);
            if i_line == 0 {
                return;
            }
            line.iter().enumerate().for_each(|(j, &c)| {
                presents[i_shape][i_line - 1][j] = match c {
                    MAP_FILL => MAP_FILL,
                    _ => MAP_EMPTY,
                };
            });
        } else {
            let mut region: Region = Default::default();
            let mut words = line.split(|&c| c == b' ');
            let (width, length) = words
                .next()
                .unwrap()
                .split(|&c| c == b'x' || c == b':')
                .take(2)
                .map(parse::<u8>)
                .collect_tuple()
                .unwrap();
            region.width = width;
            region.length = length;
            words.map(parse::<u8>).enumerate().for_each(|(i, c)| {
                region.num_presents[i] = c;
            });
            regions.push(region);
        }
    });

    let mut present_sizes: [u8; NUM_PRESENTS] = Default::default();
    (0..NUM_PRESENTS).for_each(|i| {
        present_sizes[i] = presents[i]
            .iter()
            .flatten()
            .map(|&c| if c == MAP_FILL { 1 } else { 0 })
            .sum()
    });

    regions
        .into_iter()
        .map(|region| {
            let num_squares = (region.width / 3) as usize * (region.length / 3) as usize;
            if region
                .num_presents
                .iter()
                .map(|&p| p as usize)
                .sum::<usize>()
                <= num_squares
            {
                return 1;
            }
            let area = region.width as usize * region.length as usize;
            let num_filled: usize = zip(region.num_presents, present_sizes)
                .map(|(n, s)| n as usize * s as usize)
                .sum();
            if num_filled >= area {
                return 0;
            }
            println!("borderline region: {:?}", region);
            let mut num_presents = region
                .num_presents
                .into_iter()
                .enumerate()
                .filter_map(|(i, n)| {
                    if region.num_presents[i] > 0 {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let mut presents = presents
                .into_iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    if region.num_presents[i] > 0 {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Present>>();
            let mut map = Map::new(region.width, region.length);
            while !num_presents.is_empty() {
                if let Some(i_present) = map.pick_place_present(&presents) {
                    #[cfg(debug_assertions)]
                    println!("{:?}", map);
                    num_presents[i_present] -= 1;
                    if num_presents[i_present] == 0 {
                        presents.remove(i_present);
                        num_presents.remove(i_present);
                    }
                } else {
                    return 0;
                }
            }
            1
        })
        .sum()
}

pub fn get_result(input: &[u8]) -> usize {
    let mut num_presents: [u8; NUM_PRESENTS] = Default::default();
    input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .skip(24)
        .map(|line| {
            let mut words = line.split(|&c| c == b' ');
            let (width, length) = words
                .next()
                .unwrap()
                .split(|&c| c == b'x' || c == b':')
                .take(2)
                .map(parse::<usize>)
                .collect_tuple()
                .unwrap();
            words.map(parse::<u8>).enumerate().for_each(|(i, c)| {
                num_presents[i] = c;
            });
            let num_presents = num_presents.into_iter().map(|p| p as usize).sum::<usize>();
            let num_squares = (width / 3) * (length / 3);
            (num_presents <= num_squares) as usize
        })
        .sum()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day12.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result_full(include_bytes!("../../inputs/day12.example.txt"));
        assert_eq!(result, 2);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day12.txt"));
        assert_eq!(result, 521);
    }
}
