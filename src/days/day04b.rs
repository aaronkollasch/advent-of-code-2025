use crate::common::Vec2;

type Number = usize;
type Pos = Vec2<Number>;

const MAX_COLS: usize = 139;
const MAX_ROWS: usize = 139;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Grid {
    vals: [[bool; MAX_COLS]; MAX_ROWS],
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            vals: [[false; MAX_COLS]; MAX_ROWS],
            width,
            height,
        }
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        self.vals = [[false; MAX_COLS]; MAX_ROWS];
    }

    fn get_val(&self, pos: Pos) -> bool {
        self.vals[pos.y][pos.x]
    }

    fn set_val(&mut self, pos: Pos) {
        self.vals[pos.y][pos.x] = true;
    }

    fn unset_val(&mut self, pos: Pos) {
        self.vals[pos.y][pos.x] = false;
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
                    grid.set_val(Vec2 {
                        x: i_col + 1,
                        y: i_row + 1,
                    });
                }
            });
        });

    let mut q: Vec<Pos>;
    if cfg!(debug_assertions) {
        q = Vec::with_capacity(4);
    } else {
        q = Vec::with_capacity(8192);
    }
    let mut num_removed = 0;
    for i_row in 1..MAX_ROWS - 1 {
        for i_col in 1..MAX_COLS - 1 {
            let pos = Vec2 { x: i_col, y: i_row };
            if grid.get_val(pos) {
                let neighbors = [
                    Vec2 {
                        x: pos.x - 1,
                        y: pos.y - 1,
                    },
                    Vec2 {
                        x: pos.x - 1,
                        y: pos.y,
                    },
                    Vec2 {
                        x: pos.x - 1,
                        y: pos.y + 1,
                    },
                    Vec2 {
                        x: pos.x + 1,
                        y: pos.y - 1,
                    },
                    Vec2 {
                        x: pos.x + 1,
                        y: pos.y,
                    },
                    Vec2 {
                        x: pos.x + 1,
                        y: pos.y + 1,
                    },
                    Vec2 {
                        x: pos.x,
                        y: pos.y - 1,
                    },
                    Vec2 {
                        x: pos.x,
                        y: pos.y + 1,
                    },
                ];
                let mut num_neighbors = 0;
                for neighbor in neighbors {
                    if grid.get_val(neighbor) {
                        num_neighbors += 1;
                    }
                }
                if num_neighbors < 4 {
                    q.extend(neighbors.into_iter().filter(|&p| grid.get_val(p)));
                    num_removed += 1;
                    grid.unset_val(pos);
                }
            }
        }
    }
    while let Some(pos) = q.pop() {
        if !grid.get_val(pos) {
            continue;
        }
        let mut num_neighbors = 0u8;
        let neighbors = [
            Vec2 {
                x: pos.x - 1,
                y: pos.y - 1,
            },
            Vec2 {
                x: pos.x - 1,
                y: pos.y,
            },
            Vec2 {
                x: pos.x - 1,
                y: pos.y + 1,
            },
            Vec2 {
                x: pos.x + 1,
                y: pos.y - 1,
            },
            Vec2 {
                x: pos.x + 1,
                y: pos.y,
            },
            Vec2 {
                x: pos.x + 1,
                y: pos.y + 1,
            },
            Vec2 {
                x: pos.x,
                y: pos.y - 1,
            },
            Vec2 {
                x: pos.x,
                y: pos.y + 1,
            },
        ];
        for neighbor in neighbors {
            if grid.get_val(neighbor) {
                num_neighbors += 1;
            }
        }
        if num_neighbors < 4 {
            num_removed += 1;
            grid.unset_val(pos);
            q.extend(neighbors.into_iter().filter(|&p| grid.get_val(p)));
        }
    }
    #[cfg(debug_assertions)]
    println!("deque size: {}", q.capacity());
    num_removed
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
        assert_eq!(result, 43);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day04.txt"));
        assert_eq!(result, 8727);
    }
}
