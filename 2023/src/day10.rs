use std::collections::{HashMap, HashSet};

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_y = map.len();
    let max_x = map[0].len();

    let find_start = || {
        for y in 0..max_y {
            for x in 0..max_x {
                if map[y][x] == 'S' {
                    return (y, x);
                }
            }
        }
        unreachable!()
    };
    let start = find_start();
    let mut visited = HashMap::new();
    visited.insert(start, 0);

    let mut queue = vec![(start, 0)];
    while let Some((current, steps)) = queue.pop() {
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if is_reachable(current, *dir, &map) {
                let next_pos = dir.apply(current);
                if let Some(old_steps) = visited.get_mut(&next_pos) {
                    if *old_steps <= steps + 1 {
                        continue;
                    } else {
                        *old_steps = steps + 1;
                        queue.push((next_pos, steps + 1));
                    }
                } else {
                    visited.insert(next_pos, steps + 1);
                    queue.push((next_pos, steps + 1));
                }
            }
        }
    }
    *visited.values().max().unwrap()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_y = map.len();
    let max_x = map[0].len();

    let find_start = || {
        for y in 0..max_y {
            for x in 0..max_x {
                if map[y][x] == 'S' {
                    return (y, x);
                }
            }
        }
        unreachable!()
    };
    let start = find_start();
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut queue = vec![start];
    while let Some(current) = queue.pop() {
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if is_reachable(current, *dir, &map) {
                let next_pos = dir.apply(current);
                if !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    queue.push(next_pos);
                }
            }
        }
    }

    let mut inside_points = 0;
    for y in 0..max_y {
        let mut parity = 0;
        for x in 0..max_x {
            if ['|', 'J', 'L', 'S'].contains(&map[y][x]) && visited.contains(&(y, x)) {
                parity += 1;
            }
            if !visited.contains(&(y, x)) && (parity % 2 == 1) {
                inside_points += 1;
            }
        }
    }
    inside_points
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn apply(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        }
    }
}

fn is_reachable((from_y, from_x): (usize, usize), direction: Direction, map: &[Vec<char>]) -> bool {
    let max_y = map.len();
    let max_x = map[0].len();
    let (to_y, to_x) = match direction {
        Direction::Up if from_y > 0 => (from_y - 1, from_x),
        Direction::Down if from_y < max_y => (from_y + 1, from_x),
        Direction::Left if from_x > 0 => (from_y, from_x - 1),
        Direction::Right if from_x < max_x => (from_y, from_x + 1),
        _ => return false,
    };
    if to_y >= max_y || to_x >= max_x {
        return false;
    }

    let to_char = map[to_y][to_x];
    let from_char = map[from_y][from_x];
    let m = |to_char, from_char, direction| match (to_char, from_char, direction) {
        ('|', '|', Direction::Up | Direction::Down)
        | ('-', '-', Direction::Left | Direction::Right)
        | ('|', 'L' | 'J', Direction::Down)
        | ('|', 'F' | '7', Direction::Up)
        | ('-', 'L' | 'F', Direction::Left)
        | ('-', 'J' | '7', Direction::Right)
        | ('L', 'F' | '7', Direction::Up)
        | ('L', 'J' | '7', Direction::Right)
        | ('J', 'F' | 'L', Direction::Left)
        | ('J', '7' | 'F', Direction::Up)
        | ('7', 'F' | 'L', Direction::Left)
        | ('7', 'L' | 'J', Direction::Down)
        | ('F', 'J' | '7', Direction::Right)
        | ('F', 'J' | 'L', Direction::Down)
        | ('S', '|' | 'F' | '7', Direction::Up)
        | ('S', '|' | 'J' | 'L', Direction::Down)
        | ('S', '-' | 'F' | 'L', Direction::Left)
        | ('S', '-' | 'J' | '7', Direction::Right) => true,
        _ => false,
    };
    m(from_char, to_char, direction) || m(to_char, from_char, direction.opposite())
}

#[test]
fn test_part1() {
    let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
    assert_eq!(part1(input), 4);
}
