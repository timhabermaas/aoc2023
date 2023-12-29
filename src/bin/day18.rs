use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Copy, Clone)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn vec(&self) -> (i32, i32) {
        match self {
            Direction::L => (-1, 0),
            Direction::R => (1, 0),
            Direction::U => (0, -1),
            Direction::D => (0, 1),
        }
    }

    fn rotation(&self, next: Self) -> Rotation {
        use Rotation::*;
        match (self, next) {
            (Direction::R, Direction::D) => CW,
            (Direction::L, Direction::U) => CW,
            (Direction::L, Direction::D) => CCW,
            (Direction::R, Direction::U) => CCW,
            (Direction::U, Direction::L) => CCW,
            (Direction::U, Direction::R) => CW,
            (Direction::D, Direction::L) => CW,
            (Direction::D, Direction::R) => CCW,
            _ => panic!("impossible combination: {:?} {:?}", self, next),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Self::L,
            "R" => Self::R,
            "U" => Self::U,
            "D" => Self::D,
            _ => panic!("unknown character {}", s),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Color((u8, u8, u8));

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut chars = input.chars();
        chars.next().unwrap();

        fn decode_hex(s: &str) -> u8 {
            hex::decode(s).unwrap()[0]
        }
        let r = decode_hex(&format!(
            "{}{}",
            chars.next().unwrap(),
            chars.next().unwrap()
        ));
        let g = decode_hex(&format!(
            "{}{}",
            chars.next().unwrap(),
            chars.next().unwrap()
        ));
        let b = decode_hex(&format!(
            "{}{}",
            chars.next().unwrap(),
            chars.next().unwrap()
        ));

        Ok(Color((r, g, b)))
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    steps: u32,
    paint: Color,
}

impl Instruction {
    fn move_from(&self, start: (f64, f64)) -> (f64, f64) {
        let (x, y) = self.direction.vec();

        let steps = self.steps as i32;
        (start.0 + (x * steps) as f64, start.1 + (y * steps) as f64)
    }

    fn reinterpret(&self) -> Instruction {
        let steps = ((self.paint.0 .0 as u32) << 12)
            + ((self.paint.0 .1 as u32) << 4)
            + (self.paint.0 .2 as u32 >> 4);

        let dir_num = self.paint.0 .2 & 0b1111;

        let direction = match dir_num {
            0 => Direction::R,
            1 => Direction::D,
            2 => Direction::L,
            3 => Direction::U,
            _ => panic!("expected only 0,1,2 or 3"),
        };

        Self {
            direction,
            steps,
            paint: self.paint,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(" ");

        let direction = parts.next().unwrap().parse().unwrap();
        let steps = parts.next().unwrap().parse().unwrap();
        let rest = parts.next().unwrap();
        let paint = rest[1..rest.len() - 1].parse().unwrap();

        Ok(Instruction {
            direction,
            paint,
            steps,
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rotation {
    CW,
    CCW,
}

impl Puzzle {
    fn reinterpret(&self) -> Puzzle {
        let mut result = Puzzle {
            instructions: vec![],
        };
        for instruction in self.instructions.iter() {
            result.instructions.push(instruction.reinterpret());
        }
        result
    }

    fn corners(&self) -> impl Iterator<Item = (Direction, Direction)> {
        let last_instruction = [self.instructions[self.instructions.len() - 1].clone()];
        last_instruction
            .into_iter()
            .chain(self.instructions.clone().into_iter())
            .map(|i| i.direction)
            .tuple_windows()
    }

    fn path_direction(&self) -> Rotation {
        let ccws = self
            .corners()
            .map(|w| w.0.rotation(w.1))
            .filter(|r| *r == Rotation::CCW)
            .count();
        let cws = self
            .corners()
            .map(|w| w.0.rotation(w.1))
            .filter(|r| *r == Rotation::CW)
            .count();
        if ccws == cws + 4 {
            Rotation::CCW
        } else if cws == ccws + 4 {
            Rotation::CW
        } else {
            panic!("Loop isn't closed!");
        }
    }

    fn inner_points(&self) -> Vec<(f64, f64)> {
        let mut inner_points = Vec::with_capacity(self.instructions.len() + 1);
        let mut current_pos: (f64, f64) = (0.5, 0.5);
        inner_points.push(current_pos);

        // In order to calclulate the area of the polygon we need to know the outline. When
        // starting at the center of a block and moving from center to center we might miss some
        // area. In order to avoid that we start at (0.5, 0.5) which can be interpreted as "center
        // of block". By later adding the missed quarter blocks we can regain the missed are.

        for instruction in self.instructions.iter().take(self.instructions.len() - 1) {
            current_pos = instruction.move_from(current_pos);

            inner_points.push(current_pos);
        }

        inner_points
    }

    fn enclosed_area(&self) -> i64 {
        let inner_area: f64 = self
            .inner_points()
            .iter()
            .tuple_windows()
            .map(|(old_point, new_point)| (old_point.1 + new_point.1) * (old_point.0 - new_point.0))
            .sum();
        let inner_area = (inner_area.abs().round() as i64) / 2;

        let mut straight_area: i64 = 0;
        for instruction in self.instructions.iter() {
            // We only care about the straights betwen corners, so remove the last step into the
            // next corner.
            straight_area += instruction.steps as i64 - 1;
        }

        // For each corner we're either missing a quarter or three quarter of the area, depending
        // on whether the enclosed area is inside or outside.
        let mut corner_area: f64 = 0.0;
        for (from, to) in self.corners() {
            // If the entire loop is CW, for each CW turn we miss 3/4 and for each CCW turn we miss
            // 1/4 of area.
            let mut outer_area = match from.rotation(to) {
                Rotation::CW => 0.75,
                Rotation::CCW => 0.25,
            };
            // In case the loop is CCW we swap 3/4 and 1/4.
            if self.path_direction() == Rotation::CCW {
                outer_area = 1.0 - outer_area;
            }
            corner_area += outer_area;
        }
        let corner_area = corner_area.round() as i64;

        inner_area + (straight_area / 2) + corner_area
    }
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];
        for line in input.lines() {
            instructions.push(line.parse().unwrap());
        }

        Ok(Puzzle { instructions })
    }
}

fn main() {
    let input = read_to_string("inputs/day18.txt").expect("file not found");

    let puzzle: Puzzle = input.parse().unwrap();
    println!("Part 1: {}", puzzle.enclosed_area());

    let puzzle = puzzle.reinterpret();
    println!("Part 2: {}", puzzle.enclosed_area());
}
