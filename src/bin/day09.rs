use std::{fs::read_to_string, str::FromStr};

#[derive(Debug)]
struct Puzzle {
    sequences: Vec<Vec<i32>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Puzzle {
            sequences: input
                .lines()
                .map(|line| {
                    line.split(" ")
                        .map(|d| d.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        })
    }
}

#[derive(Copy, Clone)]
enum StartOrEnd {
    Start,
    End,
}

fn next_value(sequence: &[i32], pos: StartOrEnd) -> i32 {
    use StartOrEnd::*;

    let mut current_row = sequence.to_owned();
    let mut rows = vec![current_row.clone()];

    while !current_row.iter().all(|x| *x == 0) {
        current_row = current_row
            .iter()
            .take(current_row.len() - 1)
            .zip(current_row.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        rows.push(current_row.clone());
    }

    let column = rows.iter().rev().map(|x| {
        x[match pos {
            Start => 0,
            End => x.len() - 1,
        }]
    });

    column
        .reduce(|acc, x| match pos {
            Start => x - acc,
            End => acc + x,
        })
        .unwrap()
}

fn solve(puzzle: &Puzzle, pos: StartOrEnd) -> i32 {
    puzzle.sequences.iter().map(|s| next_value(s, pos)).sum()
}

fn main() {
    let input = read_to_string("inputs/day09.txt").expect("file not found");

    let puzzle = input.parse().unwrap();

    println!("Part 1: {}", solve(&puzzle, StartOrEnd::End));
    println!("Part 2: {}", solve(&puzzle, StartOrEnd::Start));
}
