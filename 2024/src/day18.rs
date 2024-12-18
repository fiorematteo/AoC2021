use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, AddAssign, Sub};

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    part1_impl(input, 70, 1024)
}

pub fn part1_impl(input: &str, size: i64, sim_count: usize) -> usize {
    let memory: HashMap<_, _> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (x, y) = l.split_once(',').unwrap();
            (
                Pair {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                i,
            )
        })
        .collect();

    solve(size, &memory, sim_count).unwrap()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    part2_impl(input, 1024, 70)
}

fn part2_impl(input: &str, safe_point: usize, size: i64) -> String {
    let memory: HashMap<_, _> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (x, y) = l.split_once(',').unwrap();
            (
                Pair {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                i,
            )
        })
        .collect();

    let line_count = input.lines().count();
    let mut left = safe_point;
    let mut right = line_count;
    while left != right {
        let middle = (right + left) / 2;
        if solve(size, &memory, middle).is_some() {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    let last_byte = input.lines().nth(left).unwrap();
    last_byte.to_owned()
}

fn solve(size: i64, memory: &HashMap<Pair, usize>, iteration: usize) -> Option<usize> {
    let mut visited = HashSet::new();
    visited.insert(Pair { x: 0, y: 0 });
    let mut queue = VecDeque::new();
    queue.push_back((Pair { x: 0, y: 0 }, 0));
    while let Some((pos, cost)) = queue.pop_front() {
        if pos == (Pair { x: size, y: size }) {
            return Some(cost);
        }
        let new_dirs = [
            Pair { x: 0, y: 1 },
            Pair { x: 0, y: -1 },
            Pair { x: 1, y: 0 },
            Pair { x: -1, y: 0 },
        ];
        for new_dir in new_dirs {
            let new_pos = pos + new_dir;
            let new_cost = cost + 1;
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x > size || new_pos.y > size {
                continue;
            }
            if memory
                .get(&new_pos)
                .map(|&byte| byte <= iteration)
                .unwrap_or(false)
            {
                continue;
            }
            if visited.contains(&new_pos) {
                continue;
            }
            visited.insert(new_pos);
            queue.push_back((new_pos, new_cost));
        }
    }
    None
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
fn part1_test() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(part1_impl(input, 6, 12), 22);
}

#[test]
fn part2_test() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    assert_eq!(part2_impl(input, 12, 6), "6,1");
}
