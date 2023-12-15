use itertools::iproduct;
use std::{collections::HashSet, fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn is_opposite(&self, dir: Dir) -> bool {
        match (self, dir) {
            (Dir::N, Dir::S) => true,
            (Dir::S, Dir::N) => true,
            (Dir::E, Dir::W) => true,
            (Dir::W, Dir::E) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Start,
    Connection(Dir, Dir),
}

type Pos = (i32, i32);

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dir::*;
        Ok(match s {
            "|" => Self::Connection(N, S),
            "-" => Self::Connection(E, W),
            "L" => Self::Connection(N, E),
            "J" => Self::Connection(N, W),
            "7" => Self::Connection(S, W),
            "F" => Self::Connection(S, E),
            "." => Self::Ground,
            "S" => Self::Start,
            _ => panic!("Unknown symbol: '{}'", s),
        })
    }
}

impl Tile {
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    map: Vec<Vec<Tile>>,
    start: Pos,
    width: i32,
    height: i32,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = vec![];
        let mut start = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut l = vec![];
            for (x, c) in line.chars().enumerate() {
                let tile: Tile = c.to_string().parse().unwrap();
                l.push(tile);
                if tile.is_start() {
                    start = (x as i32, y as i32);
                }
            }

            map.push(l);
        }

        Ok(Puzzle {
            width: map[0].len() as i32,
            height: map.len() as i32,
            map,
            start,
        })
    }
}

fn walk(pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::N => (pos.0, pos.1 - 1),
        Dir::S => (pos.0, pos.1 + 1),
        Dir::E => (pos.0 + 1, pos.1),
        Dir::W => (pos.0 - 1, pos.1),
    }
}

/// Given the start tile finds one of the two possible directions one could walk.
fn find_first_direction(puzzle: &Puzzle) -> Dir {
    use Dir::*;
    let neighbours = [
        (puzzle.start.0 + 1, puzzle.start.1, E),
        (puzzle.start.0 - 1, puzzle.start.1, W),
        (puzzle.start.0, puzzle.start.1 + 1, S),
        (puzzle.start.0, puzzle.start.1 - 1, N),
    ];
    for neighbour in neighbours {
        // By clamping we might end up with the position of the start tile. This isn't an issue
        // since we only match on connecting tiles.
        match puzzle.map[neighbour.1.clamp(0, puzzle.height) as usize]
            [neighbour.0.clamp(0, puzzle.width) as usize]
        {
            Tile::Connection(a, b) => {
                if neighbour.2.is_opposite(a) || neighbour.2.is_opposite(b) {
                    return neighbour.2;
                }
            }
            _ => continue,
        }
    }
    panic!("There should be at least one starting direction for the start tile!");
}

fn cycle(puzzle: &Puzzle) -> Vec<Pos> {
    let mut current_dir = find_first_direction(puzzle);
    let mut current_pos = walk(puzzle.start, current_dir);

    let mut result = vec![current_pos];

    loop {
        let tile = puzzle.map[current_pos.1 as usize][current_pos.0 as usize];

        match tile {
            Tile::Start => return result,
            Tile::Ground => panic!("shouldn't land on ground!"),
            Tile::Connection(a, b) => {
                if current_dir.is_opposite(a) {
                    current_dir = b;
                } else if current_dir.is_opposite(b) {
                    current_dir = a;
                } else {
                    panic!(
                        "couldn't find opposite for pos {:?}, dir {:?} and connection {:?}, {:?}!",
                        current_pos, current_dir, a, b
                    );
                }
            }
        }

        current_pos = walk(current_pos, current_dir);

        result.push(current_pos);
    }
}

fn part_1(puzzle: &Puzzle) -> i32 {
    return cycle(puzzle).len() as i32 / 2;
}

/// Given a position `pos` and the `main_loop` determines whether the point is enclosed by the main
/// loop.
/// This is accomplished by casting a ray into an arbitrary direction and counting the number of
/// intersections with the main loop. If the number of intersections is odd, the point `pos` is
/// inside, otherwise it's outside.
/// NOTE: This function doesn't work when hitting the start tile S. This is due to the fact that
/// the directions of the start tile are unknown at this point. The proper solution would be to
/// replace `Tile::Start` with `Tile::Connection` during or right after parsing to avoid that
/// special case.
fn is_inside(pos: Pos, main_loop: &HashSet<Pos>, puzzle: &Puzzle) -> bool {
    // By definition the loop itself is not considered "inside" the enclosed space.
    if main_loop.contains(&pos) {
        return false;
    }

    // We walk diagonally in order to avoid edge cases regarding walks along edges. Using `zip` is
    // fine here since this naturally limits the diagonal line to either width or height whichever
    // is smaller.
    let ray: HashSet<Pos> = (pos.0..puzzle.width).zip(pos.1..puzzle.height).collect();

    ray.intersection(&main_loop)
        // When walking from top left to bottom right we don't want to count L and 7 pieces as
        // hits since we're leaving the enclosed area immediately. If we don't exclude them they
        // would count as one intersection instead of zero.
        .filter(|(x, y)| match puzzle.map[*y as usize][*x as usize] {
            Tile::Connection(Dir::W, Dir::S) => false,
            Tile::Connection(Dir::S, Dir::W) => false,
            Tile::Connection(Dir::N, Dir::E) => false,
            Tile::Connection(Dir::E, Dir::N) => false,
            _ => true,
        })
        .count()
        % 2
        == 1
}

fn part_2(puzzle: &Puzzle) -> i32 {
    let main_loop: HashSet<Pos> = cycle(puzzle).into_iter().collect();

    // We can go through all tiles and for each position not part of the main loop we can figure
    // out whether it's inside the enclosed area or not, see `is_inside`.

    let inside_counts = iproduct!(0..puzzle.width, 0..puzzle.height)
        .filter(|(x, y)| is_inside((*x as i32, *y as i32), &main_loop, puzzle))
        .count() as i32;

    inside_counts
}

fn main() {
    let input = read_to_string("inputs/day10.txt").expect("file not found");

    let puzzle = input.parse().unwrap();

    println!("Part 1: {}", part_1(&puzzle));
    println!("Part 2: {}", part_2(&puzzle));
}
