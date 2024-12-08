use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[aoc_generator(day6)]
fn generator(input: &str) -> (Vec<Vec<char>>, Pair) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut guard: Pair = (0, 0).into();
    for (y, row) in grid.iter().enumerate() {
        for (x, &el) in row.iter().enumerate() {
            if el == '^' {
                guard = (y as _, x as _).into();
            }
        }
    }
    (grid, guard)
}

#[aoc(day6, part1)]
fn part1((grid, guard): &(Vec<Vec<char>>, Pair)) -> u32 {
    let mut guard = *guard;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut total = 0;
    let mut direction = (-1, 0).into();
    loop {
        if !visited[guard.y as usize][guard.x as usize] {
            visited[guard.y as usize][guard.x as usize] = true;
            total += 1;
        }
        if !step(grid, &mut guard, &mut direction) {
            break;
        }
    }
    total
}

#[aoc(day6, part2)]
fn part2((grid, guard): &(Vec<Vec<char>>, Pair)) -> u32 {
    let mut guard = *guard;
    let mut grid = grid.clone();
    let mut total = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut direction = (-1, 0).into();
    loop {
        let has_visited = &mut visited[guard.y as usize][guard.x as usize];
        if !*has_visited {
            *has_visited = true;
        }

        let obstacle = guard + direction;
        if in_bounds(obstacle, &grid)
            && grid[obstacle.y as usize][obstacle.x as usize] == '.'
            && !visited[obstacle.y as usize][obstacle.x as usize]
        {
            grid[obstacle.y as usize][obstacle.x as usize] = '#';
            if is_looping(&grid, guard, direction) {
                total += 1;
            }
            grid[obstacle.y as usize][obstacle.x as usize] = '.';
        }

        if !step(&grid, &mut guard, &mut direction) {
            break;
        }
    }
    total
}

fn is_looping(grid: &[Vec<char>], mut guard: Pair, mut direction: Pair) -> bool {
    let mut visited = vec![vec![(false, vec![]); grid[0].len()]; grid.len()];
    loop {
        let (has_visited, visited_directions) = &mut visited[guard.y as usize][guard.x as usize];
        if *has_visited {
            if visited_directions.contains(&direction) {
                return true;
            }
        } else {
            *has_visited = true;
        }
        visited_directions.push(direction);
        if !step(grid, &mut guard, &mut direction) {
            break;
        }
    }
    false
}

fn step(grid: &[Vec<char>], guard: &mut Pair, direction: &mut Pair) -> bool {
    let next_position = *guard + *direction;
    if !in_bounds(next_position, grid) {
        return false;
    }
    if grid[next_position.y as usize][next_position.x as usize] == '#' {
        // rotate
        *direction = match *direction {
            Pair { y: 1, x: 0 } => (0, -1),
            Pair { y: -1, x: 0 } => (0, 1),
            Pair { y: 0, x: 1 } => (1, 0),
            Pair { y: 0, x: -1 } => (-1, 0),
            _ => unreachable!(),
        }
        .into();
    } else {
        *guard = next_position;
    }
    true
}

fn in_bounds(Pair { x, y }: Pair, grid: &[Vec<char>]) -> bool {
    y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32
}

#[test]
fn test_part1() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part1(&generator(input)), 41);
}

#[test]
fn test_part2() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part2(&generator(input)), 6);
}

#[test]
fn test_part2_line() {
    let input = "......
.#^..#
....#.";
    assert_eq!(part2(&generator(input)), 1);
}
