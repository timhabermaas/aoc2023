use std::{collections::HashMap, fs::read_to_string, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    MovingRock,
    FixedRock,
    Empty,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Platform {
    grid: Vec<Vec<Tile>>,
}

impl Platform {
    fn total_load(&self) -> u32 {
        let mut sum = 0;

        for (y, row) in self.grid.iter().enumerate() {
            for tile in row {
                if *tile == Tile::MovingRock {
                    sum += (self.grid.len() - y) as u32;
                }
            }
        }

        sum
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::FixedRock,
                        'O' => Tile::MovingRock,
                        '.' => Tile::Empty,
                        _ => panic!("unknown character '{}'", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self { grid })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

/// Tilting works by iterating through each row (for south/north) or column (for east/west) and
/// keeping track of where a potential moving rock would land (called `target`).
/// If we encounter a `#` the target will be the current position +/- 1.
fn tilt(platform: &mut Platform, dir: Dir) {
    if dir == Dir::North {
        for col in 0..platform.width() {
            let mut target: usize = 0;
            for row in 0..platform.height() {
                if platform.grid[row][col] == Tile::FixedRock {
                    target = row + 1;
                } else if platform.grid[row][col] == Tile::MovingRock {
                    platform.grid[row][col] = Tile::Empty;
                    platform.grid[target][col] = Tile::MovingRock;
                    target += 1;
                }
            }
        }
    } else if dir == Dir::South {
        for col in 0..platform.width() {
            let mut target: usize = platform.height() - 1;
            for row in (0..platform.height()).rev() {
                if platform.grid[row][col] == Tile::FixedRock {
                    target = row.saturating_sub(1);
                } else if platform.grid[row][col] == Tile::MovingRock {
                    platform.grid[row][col] = Tile::Empty;
                    platform.grid[target][col] = Tile::MovingRock;
                    target = target.saturating_sub(1);
                }
            }
        }
    } else if dir == Dir::West {
        for row in 0..platform.height() {
            let mut target = 0;
            for col in 0..platform.width() {
                if platform.grid[row][col] == Tile::FixedRock {
                    target = col + 1;
                } else if platform.grid[row][col] == Tile::MovingRock {
                    platform.grid[row][col] = Tile::Empty;
                    platform.grid[row][target] = Tile::MovingRock;
                    target = target + 1;
                }
            }
        }
    } else if dir == Dir::East {
        for row in 0..platform.height() {
            let mut target = platform.width() - 1;
            for col in (0..platform.width()).rev() {
                if platform.grid[row][col] == Tile::FixedRock {
                    target = col.saturating_sub(1);
                } else if platform.grid[row][col] == Tile::MovingRock {
                    platform.grid[row][col] = Tile::Empty;
                    platform.grid[row][target] = Tile::MovingRock;
                    target = target.saturating_sub(1);
                }
            }
        }
    }
}

fn part_1(mut platform: Platform) -> u32 {
    tilt(&mut platform, Dir::North);

    platform.total_load()
}

fn part_2(mut platform: Platform) -> u32 {
    let mut states: HashMap<Platform, usize> = HashMap::new();

    let mut i = 0;
    let iteration_count = 1000000000;

    states.insert(platform.clone(), 0);

    while i < iteration_count {
        tilt(&mut platform, Dir::North);
        tilt(&mut platform, Dir::West);
        tilt(&mut platform, Dir::South);
        tilt(&mut platform, Dir::East);
        i += 1;

        if let Some(index) = states.get(&platform) {
            println!(
                "State found already in iteration {}, now at i: {}, that was {} iterations ago. The load of the state: {}",
                index,
                i,
                i - index,platform.total_load()
            );
            // If we detect a cycle we move the iteration N times forward by the length of the cycle times
            // such that the iteration will be closest to the total iteration count without going
            // over it.
            let remaining = iteration_count - i;
            let fits_how_often = remaining / (i - index);
            // We don't need to check for i going way over iteration_count since `fits_how_often`
            // is 0 once there's not a full cycle fitting into the remaining iterations, therefore
            // the iteration won't be changed.
            i += fits_how_often * (i - index);
        } else {
            states.insert(platform.clone(), i);
        }
    }
    platform.total_load()
}

fn main() {
    let input = read_to_string("inputs/day14.txt").expect("file not found");

    let platform: Platform = input.parse().unwrap();

    println!("Part 1: {}", part_1(platform.clone()));
    println!("Part 2: {}", part_2(platform.clone()));
}
