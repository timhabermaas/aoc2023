use std::fs::read_to_string;

const MAP: [(&'static str, u32); 19] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("0", 0),
];

fn first_digit(s: &str) -> u32 {
    let mut min_byte_index = s.len();
    let mut number: u32 = 0;
    for (pattern, digit) in MAP {
        let pos = s.find(pattern);
        if let Some(pos) = pos {
            if pos <= min_byte_index {
                min_byte_index = pos;
                number = digit;
            }
        }
    }

    return number;
}

fn last_digit(s: &str) -> u32 {
    let mut max_byte_index = 0;
    let mut number: u32 = 0;
    for (pattern, digit) in MAP {
        let pos = s.rfind(pattern);
        if let Some(pos) = pos {
            if pos >= max_byte_index {
                max_byte_index = pos;
                number = digit;
            }
        }
    }

    return number;
}

fn main() {
    let input = read_to_string("inputs/day01.txt").expect("file not found");

    /*
    let numbers: Vec<u32> = input
        .lines()
        .map(|c| {
            println!("{c}");
            let digits = c.chars().filter(|c| c.is_ascii_digit());
            let first = digits.clone().next().unwrap();
            let last = digits.last().unwrap();

            let first: u32 = first.to_digit(10).unwrap();
            let last: u32 = last.to_digit(10).unwrap();
            return first * 10 + last;
        })
        .collect();

    println!("Part 1: {}", numbers.iter().sum::<u32>());*/

    let mut result: Vec<u32> = vec![];

    for line in input.lines() {
        println!("{line}");
        let first = first_digit(line);
        let last = last_digit(line);
        println!("{}, {}", first, last);
        result.push(first * 10 + last);
    }

    println!("Part 2: {}", result.iter().sum::<u32>());
}
