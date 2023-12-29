use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Copy, Clone)]
enum Rounding {
    Ceil,
    Floor,
}

impl Rounding {
    fn apply(&self, number: f64) -> i64 {
        match self {
            Rounding::Ceil => number.ceil() as i64,
            Rounding::Floor => number.floor() as i64,
        }
    }

    fn flip(&self) -> Rounding {
        match self {
            Rounding::Ceil => Rounding::Floor,
            Rounding::Floor => Rounding::Ceil,
        }
    }
}

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

    fn points(&self) -> Vec<(i64, i64)> {
        let mut inner_points = Vec::with_capacity(self.instructions.len() + 1);
        let mut current_pos: (f64, f64) = (0.5, 0.5);
        inner_points.push(current_pos);

        // In order to calclulate the area of the polygon we need to know the outline. When
        // starting at the center of a block and moving from center to center we might miss some
        // area. In order to avoid that we start at (0.5, 0.5) which can be interpreted as "center
        // of block". By later rounding either up or down for each corner we find the correct
        // outline. The outline also depends on whether we walk clockwise or counter-clockwise.

        for first in self.instructions.iter().take(self.instructions.len() - 1) {
            current_pos = first.move_from(current_pos);

            inner_points.push(current_pos);
        }

        fn rounding_of_corner(
            from: Direction,
            to: Direction,
            rotation: Rotation,
        ) -> (Rounding, Rounding) {
            use Direction::*;
            use Rounding::*;
            let result = match (from, to) {
                (L, U) => (Floor, Ceil),
                (L, D) => (Ceil, Ceil),
                (R, U) => (Floor, Floor),
                (R, D) => (Ceil, Floor),
                (U, L) => (Floor, Ceil),
                (U, R) => (Floor, Floor),
                (D, L) => (Ceil, Ceil),
                (D, R) => (Ceil, Floor),
                _ => panic!("impossible combination {:?} {:?}", from, to),
            };
            if rotation == Rotation::CCW {
                (result.0.flip(), result.1.flip())
            } else {
                result
            }
        }

        let rotation = self.path_direction();
        // Get from the middle of the blocks to its outline.
        let outer_points = inner_points
            .iter()
            .zip(self.corners())
            .map(|(point, corner)| {
                let (rounding_x, rounding_y) = rounding_of_corner(corner.0, corner.1, rotation);
                (rounding_x.apply(point.0), rounding_y.apply(point.1))
            })
            .collect();

        outer_points
    }

    fn enclosed_area(&self) -> i64 {
        // Using https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula_2 to calculate
        // the area of the rectilinear polygon.
        let area: i64 = self
            .points()
            .iter()
            .tuple_windows()
            .map(|(old_point, new_point)| (old_point.1 + new_point.1) * (old_point.0 - new_point.0))
            .sum();

        area.abs() / 2
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
