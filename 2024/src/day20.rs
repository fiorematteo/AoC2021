use std::{
    collections::{BTreeSet, VecDeque},
    ops::{Add, AddAssign, Sub},
};

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 2)
}
#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 20)
}

pub fn solve(input: &str, limit: i64) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = Pair { x: 0, y: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let (y, x) = (y as i64, x as i64);
            if c == 'S' {
                start = Pair { x, y };
            }
        }
    }

    let mut distance = vec![vec![u64::MAX; grid[0].len()]; grid.len()];
    distance[start.y as usize][start.x as usize] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        for dir in [
            Pair { x: 0, y: 1 },
            Pair { x: 0, y: -1 },
            Pair { x: 1, y: 0 },
            Pair { x: -1, y: 0 },
        ] {
            let next = pos + dir;
            if grid[next.y as usize][next.x as usize] == '#' {
                continue;
            }
            let next_distance = distance[pos.y as usize][pos.x as usize] + 1;
            if distance[next.y as usize][next.x as usize] != u64::MAX {
                continue;
            }
            distance[next.y as usize][next.x as usize] = next_distance;
            queue.push_back(next);
        }
    }

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                continue;
            }
            total += explore_cheats(
                Pair {
                    x: x as i64,
                    y: y as i64,
                },
                limit,
                &grid,
                &distance,
            );
        }
    }
    total
}

fn explore_cheats(from: Pair, limit: i64, grid: &[Vec<char>], distance: &[Vec<u64>]) -> usize {
    let mut cheats: BTreeSet<Pair> = BTreeSet::new();
    for step in (-limit..=limit).flat_map(|y| (-limit..=limit).map(move |x| Pair { x, y })) {
        let step_count = step.x.abs() + step.y.abs();
        if step_count > limit {
            continue;
        }
        let pos = from + step;
        if pos.y < 0 || pos.y >= grid.len() as i64 || pos.x < 0 || pos.x >= grid[0].len() as i64 {
            continue;
        }
        if grid[pos.y as usize][pos.x as usize] != '#' {
            let gain = distance[pos.y as usize][pos.x as usize] as i64
                - distance[from.y as usize][from.x as usize] as i64
                - step_count;
            if gain >= 100 && !cheats.contains(&pos) {
                cheats.insert(pos);
            }
        }
    }
    cheats.len()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pair {
    x: i64,
    y: i64,
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Pair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pair {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i64, i64)> for Pair {
    fn from(value: (i64, i64)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

#[test]
fn test_part1() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    part1(input);
}

#[test]
fn test_part2() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    part2(input);
}
