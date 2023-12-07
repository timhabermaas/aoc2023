use std::{collections::HashMap, fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => Result::Ok(Color::Red),
            "green" => Result::Ok(Color::Green),
            "blue" => Result::Ok(Color::Blue),
            _ => panic!("expected red, green or blue, but got '{}'", input),
        }
    }
}

#[derive(Debug)]
struct Draw(Color, u32);

impl FromStr for Draw {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (count, color) = input.trim().split_once(' ').unwrap();

        let count = count.parse().unwrap();
        let color = color.parse().unwrap();

        Result::Ok(Draw(color, count))
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Vec<Draw>>,
}
impl Game {
    fn is_possible(&self, bag: &[Draw]) -> bool {
        for draw in self.draws.iter() {
            for color in draw {
                for bag_color in bag {
                    if bag_color.0 == color.0 && color.1 > bag_color.1 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn min_bag(&self) -> Vec<Draw> {
        let reds: u32 = self
            .draws
            .iter()
            .filter_map(|b| b.iter().find(|d| d.0 == Color::Red))
            .map(|d| d.1)
            .max()
            .unwrap_or(0);

        let greens: u32 = self
            .draws
            .iter()
            .filter_map(|b| b.iter().find(|d| d.0 == Color::Green))
            .map(|d| d.1)
            .max()
            .unwrap_or(0);

        let blues: u32 = self
            .draws
            .iter()
            .filter_map(|b| b.iter().find(|d| d.0 == Color::Blue))
            .map(|d| d.1)
            .max()
            .unwrap_or(0);

        vec![
            Draw(Color::Red, reds),
            Draw(Color::Blue, blues),
            Draw(Color::Green, greens),
        ]
    }
}

fn parse_line(line: &str) -> Game {
    let (game, draws_s) = line.split_once(':').unwrap();
    let (_game, id) = game.split_once(' ').unwrap();

    let mut draws = vec![];
    for draw in draws_s.split(';') {
        let hand = draw.split(',').map(|c| c.parse().unwrap()).collect();
        draws.push(hand);
    }

    Game {
        id: id.parse().unwrap(),
        draws,
    }
}

fn main() {
    let input = read_to_string("inputs/day02.txt").expect("file not found");

    let games = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let bag = vec![
        Draw(Color::Red, 12),
        Draw(Color::Green, 13),
        Draw(Color::Blue, 14),
    ];

    //let mut id_sum: u32 = 0;
    let id_sum: u32 = games
        .iter()
        .filter(|game| game.is_possible(&bag))
        .map(|game| game.id)
        .sum();

    println!("Part 1: {}", id_sum);

    let mut sum_power: u32 = 0;
    for game in games {
        let power: u32 = game.min_bag().iter().map(|d| d.1).product();
        sum_power += power;
    }

    println!("Part 2: {}", sum_power);
}
