use std::collections::{HashMap, HashSet};

#[aoc_generator(day10)]
fn generator(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let find_start = || {
        for (y, row) in map.iter().enumerate() {
            for (x, &el) in row.iter().enumerate() {
                if el == 'S' {
                    return (y, x);
                }
            }
        }
        unreachable!()
    };
    let start = find_start();
    let mut possible_directions = vec![];
    for dir in &[
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let next_pos = dir.apply(start);
        if char_can_move(map[next_pos.0][next_pos.1]).contains(&dir.opposite()) {
            possible_directions.push(*dir);
        }
    }

    map[start.0][start.1] = ['-', '|', 'L', 'J', 'F', '7', '.']
        .into_iter()
        .find(|&c| char_can_move(c) == possible_directions)
        .unwrap();
    (map, start)
}

#[aoc(day10, part1)]
fn part1((map, start): &(Vec<Vec<char>>, (usize, usize))) -> usize {
    let mut visited = HashMap::new();
    visited.insert(*start, 0);

    let mut queue = vec![(*start, 0)];
    while let Some((current, steps)) = queue.pop() {
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if is_reachable(current, *dir, map) {
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
fn part2((map, start): &(Vec<Vec<char>>, (usize, usize))) -> usize {
    let mut visited = HashSet::new();
    visited.insert(*start);

    let mut queue = vec![*start];
    while let Some(current) = queue.pop() {
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if is_reachable(current, *dir, map) {
                let next_pos = dir.apply(current);
                if !visited.contains(&next_pos) {
                    visited.insert(next_pos);
                    queue.push(next_pos);
                }
            }
        }
    }

    let mut inside_points = 0;
    for (y, row) in map.iter().enumerate() {
        let mut parity = 0;
        for (x, el) in row.iter().enumerate() {
            if ['|', 'J', 'L', 'S'].contains(el) && visited.contains(&(y, x)) {
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

    char_can_move(from_char).iter().any(|&dir| dir == direction)
        && char_can_move(to_char)
            .iter()
            .any(|&dir| dir == direction.opposite())
}

fn char_can_move(c: char) -> Vec<Direction> {
    match c {
        '-' => vec![Direction::Left, Direction::Right],
        '|' => vec![Direction::Up, Direction::Down],
        'L' => vec![Direction::Up, Direction::Right],
        'J' => vec![Direction::Up, Direction::Left],
        'F' => vec![Direction::Down, Direction::Right],
        '7' => vec![Direction::Down, Direction::Left],
        '.' => vec![],
        _ => unreachable!(),
    }
}

#[test]
fn test_part1() {
    let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
    assert_eq!(part1(&generator(input)), 4);
}
