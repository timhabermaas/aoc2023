use std::{collections::HashMap, fs::read_to_string, str::FromStr};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    directions: Vec<Direction>,
    mapping: HashMap<String, (String, String)>,
}

fn parse_mapping(input: &str) -> (String, (String, String)) {
    let (from, to) = input.split_once(" = ").unwrap();

    let to = &to[1..to.len() - 1];
    let (to_left, to_right) = to.split_once(", ").unwrap();

    (from.to_owned(), (to_left.to_owned(), to_right.to_owned()))
}

fn parse_puzzle(input: &str) -> Puzzle {
    let (directions, mappings) = input.split_once("\n\n").unwrap();

    let directions = directions
        .chars()
        .map(|c| c.to_string().parse::<Direction>().unwrap())
        .collect();

    let mut mapping = HashMap::new();
    for line in mappings.lines() {
        let (left, right) = parse_mapping(line);
        mapping.insert(left, right);
    }

    Puzzle {
        directions,
        mapping,
    }
}

fn advance(puzzle: &Puzzle, current_pos: &str, direction: Direction) -> String {
    choose(puzzle.mapping.get(current_pos).unwrap(), direction).to_string()
}

fn choose<A>((a, b): &(A, A), dir: Direction) -> A
where
    A: Clone,
{
    match dir {
        Direction::Left => a.clone(),
        Direction::Right => b.clone(),
    }
}

fn step_count(start_node: &str, puzzle: &Puzzle, is_end_node: fn(&str) -> bool) -> u64 {
    puzzle
        .directions
        .iter()
        .cycle()
        .scan(start_node.to_string(), |state, dir| {
            *state = advance(puzzle, state, *dir);
            if is_end_node(state) {
                None
            } else {
                Some("foo")
            }
        })
        .count() as u64
        + 1
}

fn part_1(puzzle: &Puzzle) -> u64 {
    step_count("AAA", puzzle, |node| node == "ZZZ")
}

fn part_2(puzzle: &Puzzle) -> u64 {
    // The problem "execute all steps in parallel and stop if all parallel paths reach an end" can
    // be reformulated in terms of the least common multiple of each step count.
    puzzle
        .mapping
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|start_node| step_count(start_node, puzzle, |node| node.ends_with('Z')))
        .reduce(least_common_multiple)
        .expect("no start nodes found")
}

/// See https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }

    a
}

/// See https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
fn least_common_multiple(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 {
        return 0;
    }
    a * (b / greatest_common_divisor(a, b))
}

fn main() {
    let input = read_to_string("inputs/day08.txt").expect("file not found");

    let puzzle = parse_puzzle(&input);

    println!("Part 1: {}", part_1(&puzzle));
    println!("Part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(greatest_common_divisor(48, 18), 6);
        assert_eq!(greatest_common_divisor(1, 0), 1);
        assert_eq!(greatest_common_divisor(0, 1), 1);
        assert_eq!(greatest_common_divisor(0, 0), 0);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(least_common_multiple(21, 6), 42);
        assert_eq!(least_common_multiple(1, 0), 0);
        assert_eq!(least_common_multiple(0, 1), 0);
        assert_eq!(least_common_multiple(0, 0), 0);
    }
}
