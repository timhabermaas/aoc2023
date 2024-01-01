use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Card(char);

impl Card {
    fn value(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => self.0.to_string().parse().unwrap(),
        }
    }

    fn is_joker(&self) -> bool {
        self.0 == 'J'
    }

    fn value_with_joker(&self) -> u32 {
        if self.is_joker() {
            1
        } else {
            self.value()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand([Card; 5]);

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn type_value(unique_cards: &[(Card, usize)]) -> u32 {
    let type_value = if unique_cards.len() == 1 {
        7_000_000
    } else if unique_cards[0].1 == 4 {
        6_000_000
    } else if unique_cards[0].1 == 3 && unique_cards[1].1 == 2 {
        5_000_000
    } else if unique_cards[0].1 == 3 {
        4_000_000
    } else if unique_cards[0].1 == 2 && unique_cards[1].1 == 2 {
        3_000_000
    } else if unique_cards[0].1 == 2 {
        2_000_000
    } else {
        1_000_000
    };
    type_value
}

impl Hand {
    fn value(&self) -> u32 {
        let mut unique_cards = self.unique_cards().into_iter().collect::<Vec<_>>();
        unique_cards.sort_by(|a, b| b.1.cmp(&a.1));

        let type_value = type_value(&unique_cards);
        let hand_value = self.pure_hand_value(Card::value);

        type_value + hand_value
    }

    fn unique_cards(&self) -> HashMap<Card, usize> {
        self.0
            .iter()
            .into_group_map_by(|x| *x)
            .into_iter()
            .map(|(k, v)| (*k, v.len()))
            .collect()
    }

    fn value_with_joker(&self) -> u32 {
        let mut unique_cards = self.unique_cards().into_iter().collect::<Vec<_>>();
        unique_cards.sort_by(|a, b| b.1.cmp(&a.1));
        let joker_count = unique_cards
            .iter()
            .find(|(c, _)| c.is_joker())
            .map(|j| j.1)
            .unwrap_or(0);

        // Prevent removing all cards if the hand consists of 5 Js.
        if unique_cards.len() == 1 && unique_cards[0].0.is_joker() {
        } else {
            unique_cards.retain(|(c, _)| !c.is_joker());
            unique_cards[0].1 += joker_count;
        }

        let type_value = type_value(&unique_cards);
        let hand_value = self.pure_hand_value(Card::value_with_joker);

        type_value + hand_value
    }

    fn pure_hand_value<F>(&self, card_value_fn: F) -> u32
    where
        F: Fn(&Card) -> u32,
    {
        self.0
            .iter()
            .rev()
            .map(card_value_fn)
            .enumerate()
            .map(|(i, value)| value * 15_u32.pow(i as u32))
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Bid {
    bid: u32,
    hand: Hand,
}

#[derive(Debug, Clone)]
struct Puzzle {
    bids: Vec<Bid>,
}

fn total_winnings(bids: &[Bid]) -> u32 {
    bids.into_iter()
        .enumerate()
        .map(|(i, bid)| bid.bid * (i as u32 + 1))
        .sum()
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let mut puzzle = puzzle.clone();
    puzzle.bids.sort_by(|a, b| a.hand.cmp(&b.hand));

    total_winnings(&puzzle.bids)
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let mut puzzle = puzzle.clone();
    puzzle
        .bids
        .sort_by(|a, b| a.hand.value_with_joker().cmp(&b.hand.value_with_joker()));

    total_winnings(&puzzle.bids)
}

fn main() {
    let input = read_to_string("inputs/day07.txt").expect("file not found");

    let mut bids = vec![];
    for line in input.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse().unwrap();
        let hand = Hand(
            hand.chars()
                .map(|c| Card(c))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
        bids.push(Bid { bid, hand });
    }
    let puzzle = Puzzle { bids };

    println!("Part 1: {}", part_1(&puzzle));
    println!("Part 2: {}", part_2(&puzzle));
}
