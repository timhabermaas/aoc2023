use std::fs::read_to_string;

/*
 * The following holds for D = distance, R = record (distance), x = acceleration time, y = driving
 * time and T = duration of race:
 *
 *   T = x + y
 *   D = x * y = x * (T - x) = T*x - x^2
 *
 * To find out how long the record holder had to press the acceleration button one has to solve
 *
 *   R = D  <=>  R = x*T - x^2  <=>  x^2 - T*x + R = 0
 *
 * There are two solutions to this quadratic equation x_1 and x_2 with x_1 > x_2. These two points
 * mark the start and end of the acceleration-button-press range within which the record can be
 * broken. We now need to find all natural numbers within (x_1, x_2). Therefore we round x_1 down
 * and x_2 up and get the number of natural numbers using x_2 - x_ 1 - 1.
 */

struct Race {
    duration: i64,
    record: i64,
}

fn accelerations_for_record(Race { duration, record }: &Race) -> (f64, f64) {
    let duration = *duration as f64;
    let record = *record as f64;

    // Solving the quadratic equation x^2 - T*x + R = 0.
    let x_1 = (duration - (duration.powf(2.0) - 4.0 * (1.0) * (record)).sqrt()) / 2.0;
    let x_2 = (duration + (duration.powf(2.0) - 4.0 * (1.0) * (record)).sqrt()) / 2.0;

    (x_1, x_2)
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_1_and_2(races: &[Race]) -> i64 {
    races
        .iter()
        .map(accelerations_for_record)
        .map(|(acc_short, acc_long)| acc_long.ceil() as i64 - acc_short.floor() as i64 - 1)
        .product()
}

/// Takes a list of races and removes all spaces between the numbers:
///
///   Time:   12 32 100
///   Record: 1342 2132 231
///
/// becomes
///
///   Time:   1232100
///   Record: 13422132231
fn unkern(races: &[Race]) -> Race {
    let (durations, records): (Vec<String>, Vec<String>) = races
        .iter()
        .map(|Race { duration, record }| (duration.to_string(), record.to_string()))
        .unzip();

    let duration: String = durations.into_iter().collect();
    let record: String = records.into_iter().collect();

    Race {
        duration: duration.parse().unwrap(),
        record: record.parse().unwrap(),
    }
}

fn main() {
    let input = read_to_string("inputs/day06.txt").expect("file not found");

    let mut lines = input.lines();

    let (_, time) = lines.next().unwrap().split_once(": ").unwrap();
    let (_, record_distance) = lines.next().unwrap().split_once(": ").unwrap();

    let durations = parse_numbers(time);
    let record_distances = parse_numbers(record_distance);

    let races = durations
        .iter()
        .zip(record_distances.iter())
        .map(|(duration, record)| Race {
            duration: *duration,
            record: *record,
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1_and_2(&races));

    println!("Part 2: {}", part_1_and_2(&[unkern(&races)]));
}
