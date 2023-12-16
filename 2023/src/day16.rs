use std::collections::HashSet;

#[aoc(day16, part1)]
fn part1(input: &str) -> usize {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    solve(&map, ((0, 0), Direction::Right))
}

#[aoc(day16, part2)]
fn part2(input: &str) -> usize {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut best = 0;
    for y in 0..map.len() {
        best = best.max(solve(&map, ((y as _, 0), Direction::Right)));
        best = best.max(solve(
            &map,
            ((y as _, map[0].len() as i64 - 1), Direction::Left),
        ));
    }
    for x in 0..map.len() {
        best = best.max(solve(&map, ((0, x as _), Direction::Down)));
        best = best.max(solve(&map, ((map.len() as i64 - 1, x as _), Direction::Up)));
    }
    best
}

fn solve(map: &Vec<Vec<char>>, start: ((i64, i64), Direction)) -> usize {
    let mut cycle = HashSet::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut queue = vec![start];
    while let Some(node) = queue.pop() {
        if cycle.contains(&node) {
            continue;
        } else {
            cycle.insert(node);
        }
        let ((y, x), ref dir) = node;
        let new_dirs = match (map[y as usize][x as usize], dir) {
            ('.', _)
            | ('|', Direction::Up | Direction::Down)
            | ('-', Direction::Left | Direction::Right) => {
                vec![*dir]
            }
            ('/', Direction::Right) | ('\\', Direction::Left) => {
                vec![Direction::Up]
            }
            ('/', Direction::Left) | ('\\', Direction::Right) => {
                vec![Direction::Down]
            }
            ('/', Direction::Up) | ('\\', Direction::Down) => {
                vec![Direction::Right]
            }
            ('/', Direction::Down) | ('\\', Direction::Up) => {
                vec![Direction::Left]
            }
            ('|', Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            ('-', Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
            _ => unreachable!(),
        };
        for dir in new_dirs {
            let coords = dir.apply((y, x));
            if coords.0 < map.len() as i64
                && coords.1 < map[0].len() as i64
                && coords.0 >= 0
                && coords.1 >= 0
            {
                visited[coords.0 as usize][coords.1 as usize] = true;
                queue.push((coords, dir));
            }
        }
    }
    visited
        .into_iter()
        .flat_map(|l| l.into_iter())
        .filter(|b| *b)
        .count()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, (y, x): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        }
    }
}
