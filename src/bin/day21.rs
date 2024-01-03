use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

type Coordinate = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Rock,
    GardenPlot,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: Coordinate,
}

impl Grid {
    fn get(&self, (x, y): Coordinate) -> Option<Cell> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }

        let index = y as usize * self.width + x as usize;

        Some(self.cells[index])
    }

    fn neighbours(&self, (x, y): Coordinate) -> HashSet<Coordinate> {
        vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
            .into_iter()
            .filter(|coord| self.get(*coord) == Some(Cell::GardenPlot))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: Coordinate,
    cost: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

fn dijkstra(grid: &Grid, start: Coordinate) -> HashMap<Coordinate, u32> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
    to_visit.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { cost, pos }) = to_visit.pop() {
        if !visited.insert(pos) {
            continue;
        }

        let neighbours = grid.neighbours(pos);
        for neighbour in neighbours {
            let new_distance = cost + 1;
            let is_shorter = distances
                .get(&neighbour)
                .map_or(true, |&current| new_distance < current);

            if is_shorter {
                distances.insert(neighbour, new_distance);
                to_visit.push(Node {
                    pos: neighbour,
                    cost: new_distance,
                });
            }
        }
    }

    distances
}

fn parse(input: &str) -> Grid {
    let mut cells = vec![];
    let mut width = 0;
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        width = line.chars().count();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
            }
            cells.push(match c {
                '.' | 'S' => Cell::GardenPlot,
                '#' => Cell::Rock,
                _ => panic!("unknown cell {}", c),
            });
        }
    }
    let height = input.lines().count();
    Grid {
        cells,
        width,
        height,
        start,
    }
}

fn part_1(grid: &Grid) -> u32 {
    let distances = dijkstra(grid, grid.start);

    let reachable = distances
        .values()
        .filter(|d| **d <= 64)
        .filter(|d| *d % 2 == 0)
        .count() as u32;

    reachable
}

fn main() {
    let input = read_to_string("inputs/day21.txt").expect("file not found");

    let grid = parse(&input);

    println!("Part 1: {}", part_1(&grid));
}
