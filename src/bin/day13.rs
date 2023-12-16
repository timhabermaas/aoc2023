use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}

impl Mirror {
    fn points(&self) -> usize {
        match self {
            Self::Vertical(index) => index + 1,
            Self::Horizontal(index) => (index + 1) * 100,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    columns: Vec<u32>,
    rows: Vec<u32>,
}

impl Block {
    fn mirror(&self) -> Mirror {
        let col = find_mirror(&self.columns).map(|i| Mirror::Vertical(i));
        let row = find_mirror(&self.rows).map(|i| Mirror::Horizontal(i));
        col.or(row).unwrap()
    }
}

impl FromStr for Block {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("unknown character '{}'", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let transposed = transpose(&grid);

        Ok(Block {
            rows: grid.iter().map(|b| bitvector_to_u32(b)).collect(),
            columns: transposed.iter().map(|b| bitvector_to_u32(b)).collect(),
        })
    }
}

fn bitvector_to_u32(v: &[bool]) -> u32 {
    v.iter()
        .fold(0, |acc, d| (acc << 1) + if *d { 1 } else { 0 })
}

fn transpose<T>(grid: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..grid[0].len())
        .map(|index| {
            grid.iter()
                .map(|inner| inner[index].clone())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse(input: &str) -> Vec<Block> {
    let mut result = vec![];

    for block in input.split("\n\n") {
        result.push(block.parse().unwrap());
    }
    result
}

/// Finds the index at which the array can be mirrored. The index I should be understood as "the
/// array can be mirrored using a mirror between I and I+1".
fn find_mirror(v: &[u32]) -> Option<usize> {
    (0..v.len() - 1).find(|i| is_mirror(v, *i))
}

/// Checks if the given array can be mirrored at `index` and `index + 1`.
fn is_mirror(v: &[u32], index: usize) -> bool {
    if index + 1 >= v.len() {
        panic!("Index out of bounds. Index: {}, Len: {}", index, v.len());
    }
    let min_len = (v.len() - (index + 1)).min(index + 1);
    let left = v[index - (min_len - 1)..=index].iter().rev();
    let right = v[index + 1..index + 1 + min_len].iter();
    left.eq(right)
}

fn is_power_of_two(n: u32) -> bool {
    n & (n - 1) == 0 && n != 0
}

/// Checks if the given array could be mirrored using a mirror between `index` and `index + 1` if
/// and only if there's a single bit flip in one of the mirrored numbers.
fn is_almost_mirror(v: &[u32], index: usize) -> bool {
    if index + 1 >= v.len() {
        panic!("Index out of bounds. Index: {}, Len: {}", index, v.len());
    }
    let min_len = (v.len() - (index + 1)).min(index + 1);
    let left = v[index - (min_len - 1)..=index].iter().rev();
    let right = v[index + 1..index + 1 + min_len].iter();
    let diff_sum: u32 = left
        .zip(right)
        .map(|(l, r)| l.abs_diff(*r))
        // Contains 1 for every diff which is a power of two and 2 for every difference which isn't
        // a power of two. 0 if there's no difference. By summing it up we know that there's
        // exactly one diff with a power of two if the sum is 1.
        .map(|diff| {
            if diff == 0 {
                0
            } else if is_power_of_two(diff) {
                1
            } else {
                2
            }
        })
        .sum();

    diff_sum == 1
}

fn part_1(blocks: &[Block]) -> usize {
    blocks.iter().map(|b| b.mirror().points()).sum()
}

fn part_2(blocks: &[Block]) -> usize {
    // If there's just one smudge preventing rows/columns from being a perfect mirror from each
    // other, than the difference between these mirrored rows/columns is exactly one element and
    // the difference is of the form 2^n since we encoded the grid as binary in both dimensions.

    let mut sum = 0;
    for block in blocks {
        let mut mirror: Option<Mirror> = None;

        for i in 0..block.rows.len() - 1 {
            if is_almost_mirror(&block.rows, i) {
                mirror = Some(Mirror::Horizontal(i));
                break;
            }
        }
        for i in 0..block.columns.len() - 1 {
            if is_almost_mirror(&block.columns, i) {
                mirror = Some(Mirror::Vertical(i));
                break;
            }
        }

        sum += mirror.unwrap().points();
    }
    sum
}

fn main() {
    let input = read_to_string("inputs/day13.txt").expect("file not found");

    let blocks = parse(&input);

    println!("Part 1: {}", part_1(&blocks));
    println!("Part 2: {}", part_2(&blocks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mirror() {
        let data = vec![1, 1, 3, 5, 3];
        assert_eq!(is_mirror(&data, 0), true);
        assert_eq!(is_mirror(&data, 1), false);

        let data = vec![1, 3, 3, 1, 3];
        assert_eq!(is_mirror(&data, 1), true);
        assert_eq!(is_mirror(&data, 2), false);

        let data = vec![1, 2, 4, 3, 3];
        assert_eq!(is_mirror(&data, 3), true);
        assert_eq!(is_mirror(&data, 2), false);
    }
}
