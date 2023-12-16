use itertools::Itertools;
use std::{fs::read_to_string, ops::RangeInclusive};

#[derive(Debug)]
struct Grid {
    numbers: Vec<(u32, RangeInclusive<usize>, usize)>,
    symbols: Vec<(char, usize, usize)>,
}

impl Grid {
    fn part_numbers(&self) -> Vec<u32> {
        let mut result = vec![];
        for (number, x_range, y) in self.numbers.iter() {
            let rect_x = *x_range.start() as i32 - 1..=*x_range.end() as i32 + 1;
            let rect_y = *y as i32 - 1..=*y as i32 + 1;

            for (_symbol, x, y) in self.symbols.iter() {
                if rect_x.contains(&(*x as i32)) && rect_y.contains(&(*y as i32)) {
                    result.push(*number);
                }
            }
        }

        result
    }

    fn gear_ratios(&self) -> Vec<u32> {
        let mut result = vec![];
        for (symbol, x, y) in self.symbols.iter() {
            if *symbol == '*' {
                let mut adjacent = vec![];
                for (number, x_range, y_pos) in self.numbers.iter() {
                    let rect_x = *x_range.start() as i32 - 1..=*x_range.end() as i32 + 1;
                    let rect_y = *y_pos as i32 - 1..=*y_pos as i32 + 1;
                    if rect_x.contains(&(*x as i32)) && rect_y.contains(&(*y as i32)) {
                        adjacent.push(*number);
                    }
                }

                if adjacent.len() == 2 {
                    result.push(*adjacent.first().unwrap() * *adjacent.last().unwrap());
                }
            }
        }

        result
    }
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid {
        numbers: vec![],
        symbols: vec![],
    };

    for (y, line) in input.lines().enumerate() {
        for (key, group) in &line.chars().enumerate().group_by(|(_x, c)| match c {
            digit if digit.is_ascii_digit() => 1,
            '.' => 2,
            _ => 3,
        }) {
            match key {
                1 => {
                    let group = group.collect::<Vec<_>>();
                    let number: u32 = group
                        .iter()
                        .map(|(_x, d)| d.to_digit(10).unwrap())
                        .rev()
                        .enumerate()
                        .map(|(i, d)| d * 10_u32.pow(i as u32))
                        .sum();

                    let range = group.first().unwrap().0..=group.last().unwrap().0;
                    grid.numbers.push((number, range, y));
                }
                2 => continue,
                3 => {
                    for (x, symbol) in group {
                        grid.symbols.push((symbol, x, y));
                    }
                }
                _ => panic!("can't happen"),
            }
        }
    }

    grid
}

fn main() {
    let input = read_to_string("inputs/day03.txt").expect("file not found");

    let grid = parse_input(&input);

    println!("Part 1: {}", grid.part_numbers().iter().sum::<u32>());

    println!("Part 2: {}", grid.gear_ratios().iter().sum::<u32>());
}
