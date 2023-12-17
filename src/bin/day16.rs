use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Foo {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn dot_product(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    fn add(&self, other: &Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn mul(&self, factor: i32) -> Self {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    fn reflect_at(&self, normal: &Self) -> Self {
        let result = self.sub(&normal.mul(2 * self.dot_product(normal)));

        // Poor man's normalization, only works for diagonal mirrors.
        if result.x.abs() > result.y.abs() {
            Self {
                x: result.x.signum(),
                y: 0,
            }
        } else {
            Self {
                x: 0,
                y: result.y.signum(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Splitter(Foo),
    Empty,
    Mirror(Vector),
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Empty,
                '-' => Tile::Splitter(Foo::Horizontal),
                '|' => Tile::Splitter(Foo::Vertical),
                '/' => Tile::Mirror(Vector { x: 1, y: 1 }),
                '\\' => Tile::Mirror(Vector { x: 1, y: -1 }),
                _ => panic!("unknown char '{}'", c),
            };
            row.push(tile);
        }
        result.push(row);
    }
    result
}

fn simulate_beams(grid: &[Vec<Tile>], start_pos: Vector, start_dir: Vector) -> usize {
    let mut beams: Vec<Vector> = vec![start_pos];
    let mut dirs: Vec<Vector> = vec![start_dir];
    let mut visits: HashSet<(Vector, Vector)> = HashSet::new();

    loop {
        let mut new_beams: Vec<Vector> = vec![];
        let mut new_dirs: Vec<Vector> = vec![];
        for (index, beam) in beams.iter_mut().enumerate() {
            visits.insert((beam.clone(), dirs[index].clone()));
            match &grid[beam.y as usize][beam.x as usize] {
                Tile::Empty => {}
                Tile::Splitter(Foo::Vertical) => {
                    if dirs[index].y == 0 {
                        dirs[index].x = 0;
                        dirs[index].y = -1;
                        new_beams.push(beam.clone());
                        new_dirs.push(Vector { x: 0, y: 1 });
                    }
                }
                Tile::Splitter(Foo::Horizontal) => {
                    if dirs[index].x == 0 {
                        dirs[index].x = -1;
                        dirs[index].y = 0;
                        new_beams.push(beam.clone());
                        new_dirs.push(Vector { x: 1, y: 0 });
                    }
                }
                Tile::Mirror(n) => {
                    dirs[index] = dirs[index].reflect_at(n);
                }
            }
        }
        beams.extend(new_beams);
        dirs.extend(new_dirs);

        let mut removed_indices: Vec<usize> = vec![];
        // Movement
        for (index, beam) in beams.iter_mut().enumerate() {
            let dir = &dirs[index];
            *beam = beam.add(dir);

            if beam.x < 0
                || beam.y < 0
                || beam.x >= grid[0].len() as i32
                || beam.y >= grid.len() as i32
                || visits.contains(&(beam.clone(), dir.clone()))
            {
                removed_indices.push(index);
            }
        }

        let mut offset = 0;

        for i in removed_indices {
            beams.remove(i - offset);
            dirs.remove(i - offset);
            offset += 1;
        }

        if beams.is_empty() {
            break;
        }
    }

    visits
        .iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}

fn part_1(grid: &[Vec<Tile>]) -> u32 {
    simulate_beams(grid, Vector { x: 0, y: 0 }, Vector { x: 1, y: 0 }) as u32
}

fn part_2(grid: &[Vec<Tile>]) -> u32 {
    let tops = (0..grid[0].len()).map(|x| (Vector { x: x as i32, y: 0 }, Vector { x: 0, y: 1 }));
    let bottoms = (0..grid[0].len()).map(|x| {
        (
            Vector {
                x: x as i32,
                y: grid.len() as i32 - 1,
            },
            Vector { x: 0, y: -1 },
        )
    });
    let rights = (0..grid.len()).map(|y| {
        (
            Vector {
                x: grid[0].len() as i32 - 1,
                y: y as i32,
            },
            Vector { x: -1, y: 0 },
        )
    });
    let lefts = (0..grid.len()).map(|y| (Vector { x: 0, y: y as i32 }, Vector { x: 1, y: 0 }));

    let all = tops.chain(bottoms).chain(rights).chain(lefts);

    all.map(|(s, d)| simulate_beams(grid, s, d)).max().unwrap() as u32 as u32
}

fn main() {
    let input = read_to_string("inputs/day16.txt").expect("file not found");

    let grid = parse(&input);

    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}
