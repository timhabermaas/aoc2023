use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};

fn hash(input: &str) -> u8 {
    let mut running: u8 = 0;

    for c in input.chars() {
        running = running.wrapping_add(c as u8);
        running = running.wrapping_mul(17);
    }

    running
}

struct Puzzle {
    operations: Vec<String>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let operations = input.trim_end().split(',').map(|s| s.to_string()).collect();
        Ok(Puzzle { operations })
    }
}

#[derive(Debug)]
enum Operator {
    Minus,
    Set(u8),
}

#[derive(Debug)]
struct Operation {
    label: String,
    operator: Operator,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (op_pos, operator) = input
            .chars()
            .find_position(|c| *c == '=' || *c == '-')
            .unwrap();

        let label = input[0..op_pos].to_string();
        let op = Operation {
            label,
            operator: match operator {
                '=' => {
                    let focal = input[op_pos + 1..].parse().unwrap();
                    Operator::Set(focal)
                }
                '-' => Operator::Minus,
                _ => panic!("unknown operator '{}'", operator),
            },
        };
        Ok(op)
    }
}

struct Boxes(Vec<Vec<(String, u8)>>);

impl Boxes {
    fn new() -> Self {
        Self(vec![vec![]; 256])
    }

    fn apply(&mut self, operation: &Operation) {
        let box_index = hash(&operation.label) as usize;

        match operation.operator {
            Operator::Minus => {
                self.0[box_index].retain(|f| f.0 != operation.label);
            }
            Operator::Set(focal) => {
                let pos = self.0[box_index]
                    .iter()
                    .find_position(|f| f.0 == operation.label);
                match pos {
                    Some((pos, (_, _))) => {
                        self.0[box_index][pos] = (operation.label.clone(), focal);
                    }
                    None => self.0[box_index].push((operation.label.clone(), focal)),
                }
            }
        }
    }

    fn focusing_power(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(move |(box_index, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_index, (_, f))| {
                        (box_index as u32 + 1) * (lens_index as u32 + 1) * *f as u32
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    puzzle.operations.iter().map(|op| hash(op) as u32).sum()
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let operations = puzzle
        .operations
        .iter()
        .map(|operation| operation.parse().unwrap());

    let mut boxes = Boxes::new();

    for operation in operations {
        boxes.apply(&operation);
    }

    boxes.focusing_power()
}

fn main() {
    let input = read_to_string("inputs/day15.txt").expect("file not found");

    let puzzle = input.parse().unwrap();

    println!("Part 1: {}", part_1(&puzzle));
    println!("Part 2: {}", part_2(&puzzle));
}
