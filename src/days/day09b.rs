use core::range::RangeInclusive;
use std::{
    cmp::Ordering,
    fmt::{Display, Error, Formatter},
};

use crate::common::parse;

type Number = isize;
type Pos = (Number, Number);

fn area(pos1: Pos, pos2: Pos) -> usize {
    let dx = pos1.0.abs_diff(pos2.0) + 1;
    let dy = pos1.1.abs_diff(pos2.1) + 1;
    dx * dy
}

#[derive(Debug)]
enum Quadrant {
    LR,
    LL,
    UL,
    UR,
}

impl Quadrant {
    fn flip(&self) -> Quadrant {
        match self {
            Quadrant::LR => Quadrant::UL,
            Quadrant::LL => Quadrant::UR,
            Quadrant::UL => Quadrant::LR,
            Quadrant::UR => Quadrant::LL,
        }
    }

    fn from(dx: Number, dy: Number) -> Option<Quadrant> {
        match (dx.cmp(&0isize), dy.cmp(&0isize)) {
            (Ordering::Equal, _) => None,
            (_, Ordering::Equal) => None,
            (Ordering::Greater, Ordering::Greater) => Some(Quadrant::LR),
            (Ordering::Less, Ordering::Greater) => Some(Quadrant::LL),
            (Ordering::Less, Ordering::Less) => Some(Quadrant::UL),
            (Ordering::Greater, Ordering::Less) => Some(Quadrant::UR),
        }
    }

    fn contains(&self, dx: Number, dy: Number) -> bool {
        match self {
            Quadrant::LR => dx >= 0 && dy >= 0,
            Quadrant::LL => dx <= 0 && dy >= 0,
            Quadrant::UL => dx <= 0 && dy <= 0,
            Quadrant::UR => dx >= 0 && dy <= 0,
        }
    }

    fn contains_exclusive(&self, dx: Number, dy: Number) -> bool {
        match self {
            Quadrant::LR => dx > 0 && dy > 0,
            Quadrant::LL => dx < 0 && dy > 0,
            Quadrant::UL => dx < 0 && dy < 0,
            Quadrant::UR => dx > 0 && dy < 0,
        }
    }
}

enum QuadrantInverted {
    Inner(Quadrant),
    Outer(Quadrant),
}

impl QuadrantInverted {
    fn contains(&self, dx: Number, dy: Number) -> bool {
        match self {
            Self::Inner(quadrant) => quadrant.contains(dx, dy),
            Self::Outer(quadrant) => quadrant.flip().contains_exclusive(dx, dy),
        }
    }
}

#[derive(Debug)]
struct Box {
    x: RangeInclusive<Number>,
    y: RangeInclusive<Number>,
}

impl Box {
    fn from(corner1: Pos, corner2: Pos) -> Self {
        Self {
            x: RangeInclusive {
                start: corner1.0.min(corner2.0),
                last: corner1.0.max(corner2.0),
            },
            y: RangeInclusive {
                start: corner1.1.min(corner2.1),
                last: corner1.1.max(corner2.1),
            },
        }
    }

    fn shrink_1(&self) -> Self {
        Self {
            x: RangeInclusive {
                start: self.x.start + 1,
                last: self.x.last - 1,
            },
            y: RangeInclusive {
                start: self.y.start + 1,
                last: self.y.last - 1,
            },
        }
    }

    fn contains_line(&self, point1: Pos, point2: Pos) -> bool {
        if point1.0 == point2.0 {
            // shared x
            self.x.contains(&point1.0)
                && (self.y.contains(&point1.1)
                    || self.y.contains(&point2.1)
                    || (point1.1.min(point2.1) < self.y.start
                        && point1.1.max(point2.1) > self.y.last))
        } else {
            // shared y
            self.y.contains(&point1.1)
                && (self.x.contains(&point1.0)
                    || self.x.contains(&point2.0)
                    || (point1.0.min(point2.0) < self.x.start
                        && point1.0.max(point2.0) > self.x.last))
        }
    }
}

impl Display for Box {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Box(x: {:?}, y: {:?})", self.x, self.y)
    }
}

pub fn get_result(input: &[u8]) -> usize {
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
    let mut largest_area = 0;
    for i in 0..red_tiles.len() - 1 {
        let current = red_tiles[i];
        let previous = red_tiles
            .get(i.wrapping_sub(1))
            .unwrap_or_else(|| red_tiles.last().unwrap());
        let next = red_tiles
            .get(i + 1)
            .unwrap_or_else(|| red_tiles.first().unwrap());
        let (dx_0, dy_0) = (current.0 - previous.0, current.1 - previous.1);
        let (dx_1, dy_1) = (next.0 - current.0, next.1 - current.1);
        #[cfg(debug_assertions)]
        println!("{}: {:?}-{:?}-{:?}", i, previous, current, next);
        #[cfg(debug_assertions)]
        println!("d's: ({}, {}), ({}, {})", dx_0, dy_0, dx_1, dy_1);

        let allowed_box_quadrants = if dx_0 == 0 && dy_0 < 0 && dx_1 > 0 {
            #[cfg(debug_assertions)]
            println!("90deg lower right");
            QuadrantInverted::Inner(Quadrant::LR)
        } else if dx_0 == 0 && dy_0 > 0 && dx_1 < 0 {
            #[cfg(debug_assertions)]
            println!("90deg upper left");
            QuadrantInverted::Inner(Quadrant::UL)
        } else if dy_0 == 0 && dx_0 > 0 && dy_1 > 0 {
            #[cfg(debug_assertions)]
            println!("90deg lower left");
            QuadrantInverted::Inner(Quadrant::LL)
        } else if dy_0 == 0 && dx_0 < 0 && dy_1 < 0 {
            #[cfg(debug_assertions)]
            println!("90deg upper right");
            QuadrantInverted::Inner(Quadrant::UR)
        } else {
            // 270deg
            #[cfg(debug_assertions)]
            println!("270deg");
            QuadrantInverted::Outer(Quadrant::LR)
        };

        for j in i + 1..red_tiles.len() {
            let (pos1, pos2) = (red_tiles[i], red_tiles[j]);
            let area = area(pos1, pos2);
            let (pos2_dx, pos2_dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
            // skip areas <= largest_area and if dx or dy == 0,
            // or if dx, dy is outside of inner angle
            if area > largest_area
                && let Some(_pos2_quadrant) = Quadrant::from(pos2_dx, pos2_dy)
                && allowed_box_quadrants.contains(pos2_dx, pos2_dy)
            {
                // check all points after i, in the same quadrant as the i->j box
                let conflict_box = Box::from(pos1, pos2).shrink_1();
                // #[cfg(debug_assertions)]
                // println!(
                //     "testing area {} for ({}-{}) {:?}-{:?}, quadrant {:?}, dx {} dy {}, conflict {}",
                //     area, i, j, pos1, pos2, _pos2_quadrant, pos2_dx, pos2_dy, conflict_box
                // );
                let mut has_conflict = false;
                let mut last_pos = pos1;
                let _ = (i + 1..red_tiles.len())
                    .chain(0..i)
                    .take_while(|&k| {
                        let pos3 = red_tiles[k];
                        let conflict_found = conflict_box.contains_line(last_pos, pos3);
                        if conflict_found {
                            // #[cfg(debug_assertions)]
                            // println!(
                            //     "conflict found for {:?}-{:?} with box {}",
                            //     last_pos, pos3, conflict_box
                            // );
                            has_conflict = true;
                            return false;
                        }
                        last_pos = pos3;
                        true
                    })
                    .last();
                if !has_conflict {
                    #[cfg(debug_assertions)]
                    println!(
                        "new largest area {} for ({}-{}) {:?} - {:?}",
                        area, i, j, pos1, pos2
                    );
                    largest_area = area;
                }
            }
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
        assert_eq!(result, 24);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day09.txt"));
        assert_eq!(result, 1474699155);
    }
}
