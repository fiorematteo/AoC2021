use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[aoc(day10, part1, hash)]
fn part1(input: &str) -> usize {
    solve(input, false)
}

#[aoc(day10, part2, hash)]
fn part2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, count_duplicates: bool) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(100))
                .collect()
        })
        .collect();
    let in_bounds =
        |p: Pair| p.y >= 0 && p.y < grid.len() as i32 && p.x >= 0 && p.x < grid[0].len() as i32;
    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &digit) in row.iter().enumerate() {
            if digit != 0 {
                continue;
            }
            let mut visited = HashSet::new();
            let mut queue: Vec<(Pair, u32)> = vec![((y as _, x as _).into(), 0)];
            while let Some((coords, value)) = queue.pop() {
                if !count_duplicates {
                    if visited.contains(&coords) {
                        continue;
                    }
                    visited.insert(coords);
                }
                if value == 9 {
                    total += 1;
                    continue;
                }
                for dir in [
                    Pair { x: 0, y: -1 },
                    Pair { x: 0, y: 1 },
                    Pair { x: -1, y: 0 },
                    Pair { x: 1, y: 0 },
                ] {
                    let next = coords + dir;
                    if in_bounds(next) && grid[next.y as usize][next.x as usize] == value + 1 {
                        queue.push((next, value + 1));
                    }
                }
            }
        }
    }
    total
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i32,
    y: i32,
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

impl From<(i32, i32)> for Pair {
    fn from(value: (i32, i32)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

#[test]
fn test_part1() {
    let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    assert_eq!(part1(input), 3);
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part1(input), 36);
}

#[test]
fn test_part2() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part2(input), 81);
}
