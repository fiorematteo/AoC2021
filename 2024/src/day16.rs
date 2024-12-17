use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, AddAssign, Sub},
};

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = Pair { x: 0, y: 0 };
    let mut end = Pair { x: 0, y: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let (y, x) = (y as i64, x as i64);
            if c == 'S' {
                start = Pair { x, y };
            } else if c == 'E' {
                end = Pair { x, y };
            }
        }
    }

    let mut distance = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        pos: start,
        dir: Pair { x: 1, y: 0 },
        cost: 0,
    });
    while let Some(State { pos, dir, cost }) = queue.pop() {
        if pos == end {
            return cost;
        }
        if distance
            .get(&(pos, dir))
            .map(|&dist| cost > dist)
            .unwrap_or(false)
        {
            continue;
        }
        distance.insert((pos, dir), cost);

        let new_dirs = match dir {
            Pair { x: 1, y: 0 } | Pair { x: -1, y: 0 } => {
                [Pair { x: 0, y: 1 }, Pair { x: 0, y: -1 }]
            }
            Pair { x: 0, y: 1 } | Pair { x: 0, y: -1 } => {
                [Pair { x: 1, y: 0 }, Pair { x: -1, y: 0 }]
            }
            _ => unreachable!(),
        };
        for new_dir in new_dirs {
            queue.push(State {
                pos,
                dir: new_dir,
                cost: cost + 1000,
            });
        }
        let next = pos + dir;
        if grid[next.y as usize][next.x as usize] != '#' {
            queue.push(State {
                pos: next,
                dir,
                cost: cost + 1,
            });
        }
    }
    unreachable!()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = Pair { x: 0, y: 0 };
    let mut end = Pair { x: 0, y: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let (y, x) = (y as i64, x as i64);
            if c == 'S' {
                start = Pair { x, y };
            } else if c == 'E' {
                end = Pair { x, y };
            }
        }
    }

    let mut prev: HashMap<(Pair, Pair), Vec<(Pair, Pair)>> = HashMap::new();
    let mut distance = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        pos: start,
        dir: Pair { x: 1, y: 0 },
        cost: 0,
    });
    while let Some(State { pos, dir, cost }) = queue.pop() {
        if pos == end {
            break;
        }

        let new_dirs = match dir {
            Pair { x: 1, y: 0 } | Pair { x: -1, y: 0 } => {
                [Pair { x: 0, y: 1 }, Pair { x: 0, y: -1 }]
            }
            Pair { x: 0, y: 1 } | Pair { x: 0, y: -1 } => {
                [Pair { x: 1, y: 0 }, Pair { x: -1, y: 0 }]
            }
            _ => unreachable!(),
        };
        for new_dir in new_dirs {
            if distance
                .get(&(pos, new_dir))
                .map(|&dist| cost + 1000 <= dist)
                .unwrap_or(true)
            {
                queue.push(State {
                    pos,
                    dir: new_dir,
                    cost: cost + 1000,
                });
                distance.insert((pos, new_dir), cost + 1000);
                prev.entry((pos, new_dir))
                    .or_insert(vec![])
                    .push((pos, dir));
            }
        }
        let next = pos + dir;
        if grid[next.y as usize][next.x as usize] != '#' {
            if distance
                .get(&(next, dir))
                .map(|&dist| cost + 1 <= dist)
                .unwrap_or(true)
            {
                queue.push(State {
                    pos: next,
                    dir,
                    cost: cost + 1,
                });
                distance.insert((next, dir), cost + 1);
                prev.entry((next, dir)).or_insert(vec![]).push((pos, dir));
            }
        }
    }

    let mut queue: Vec<_> = [
        Pair { x: 1, y: 0 },
        Pair { x: -1, y: 0 },
        Pair { x: 0, y: 1 },
        Pair { x: 0, y: -1 },
    ]
    .into_iter()
    .map(|dir| (end, dir))
    .collect();
    let mut visited = HashSet::new();
    while let Some(node) = queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for &next in prev.get(&node).unwrap_or(&vec![]) {
            queue.push(next);
        }
    }
    let visited_pos: HashSet<Pair> = visited.into_iter().map(|(pos, _)| pos).collect();
    visited_pos.len()
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, Eq)]
struct State {
    pos: Pair,
    dir: Pair,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).map(|ord| ord.reverse())
    }
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
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part1(input), 7036);
}

#[test]
fn test_part2() {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    assert_eq!(part2(input), 45);
}
