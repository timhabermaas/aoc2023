use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

type Point = (usize, usize);

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn parse(input: &str) -> Vec<Point> {
    let mut result = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => {
                    result.push((x, y));
                }
                _ => panic!("unknown character {}", c),
            }
        }
    }

    result
}

fn distances(galaxies: &[Point]) -> usize {
    (0..galaxies.len())
        .combinations(2)
        .map(|coord| {
            let from = coord[0];
            let to = coord[1];

            manhattan_distance(&galaxies[from], &galaxies[to])
        })
        .sum()
}

fn extend(galaxies: &mut [Point], offset: usize) {
    let max_x = galaxies.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = galaxies.iter().map(|(_, y)| *y).max().unwrap();

    let xs = galaxies.iter().map(|(x, _)| *x).collect::<HashSet<_>>();
    let ys = galaxies.iter().map(|(_, y)| *y).collect::<HashSet<_>>();

    let all_columns = (0..=max_x).collect::<HashSet<_>>();
    let all_rows = (0..=max_y).collect::<HashSet<_>>();

    let mut empty_columns = all_columns.difference(&xs).collect::<Vec<_>>();
    empty_columns.sort();
    let mut empty_rows = all_rows.difference(&ys).collect::<Vec<_>>();
    empty_rows.sort();

    for empty_row in empty_rows.iter().rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 > **empty_row {
                galaxy.1 += offset;
            }
        }
    }

    for empty_column in empty_columns.iter().rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 > **empty_column {
                galaxy.0 += offset;
            }
        }
    }
}

fn part_1(mut galaxies: &mut [Point]) -> usize {
    extend(&mut galaxies, 1);
    distances(galaxies)
}

fn part_2(mut galaxies: &mut [Point]) -> usize {
    extend(&mut galaxies, 1000000 - 1);
    distances(galaxies)
}

fn main() {
    let input = read_to_string("inputs/day11.txt").expect("file not found");

    let mut galaxies = parse(&input);
    println!("Part 1: {}", part_1(&mut galaxies));

    let mut galaxies = parse(&input);
    println!("Part 2: {}", part_2(&mut galaxies));
}
