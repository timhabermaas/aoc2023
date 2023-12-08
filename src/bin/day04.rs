use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Card {
    index: usize,
    winning: Vec<u32>,
    own: Vec<u32>,
}

impl Card {
    fn matches_count(&self) -> u32 {
        let mut matches = 0;

        for w in self.winning.iter() {
            for o in self.own.iter() {
                if o == w {
                    matches += 1;
                    break;
                }
            }
        }

        matches
    }

    fn points(&self) -> u32 {
        let matches_count = self.matches_count();

        if matches_count == 0 {
            0
        } else {
            2_u32.pow(matches_count - 1)
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    cards: Vec<Card>,
}

fn parse_input(input: &str) -> Puzzle {
    let mut cards = vec![];

    for (index, line) in input.lines().enumerate() {
        let (_, numbers) = line.split_once(":").unwrap();

        let (winning, own) = numbers.split_once("|").unwrap();
        cards.push(Card {
            index,
            winning: parse_numbers(winning),
            own: parse_numbers(own),
        });
    }

    Puzzle { cards }
}

fn part_2(game: &Puzzle) -> u32 {
    let mut queue: VecDeque<&Card> = VecDeque::new();
    let mut matches: HashMap<usize, u32> = HashMap::new();

    for card in game.cards.iter() {
        queue.push_back(card);
    }

    let mut result = 0;

    while let Some(card) = queue.pop_front() {
        result += 1;
        let matches = *matches
            .entry(card.index)
            .or_insert_with(|| card.matches_count());

        let rest = game
            .cards
            .get(card.index + 1..=card.index + matches as usize);

        if let Some(rest) = rest {
            for r in rest {
                queue.push_back(r);
            }
        }
    }

    result
}

fn main() {
    let input = read_to_string("inputs/day04.txt").expect("file not found");

    let game = parse_input(&input);

    println!(
        "Part 1: {}",
        game.cards.iter().map(|c| c.points()).sum::<u32>()
    );

    println!("Part 2: {}", part_2(&game));
}
