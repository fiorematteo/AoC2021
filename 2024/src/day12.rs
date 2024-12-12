use std::ops::{Add, Deref, DerefMut, Sub};

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut visited = Set::new(grid.len(), grid[0].len());
    (0..grid.len())
        .flat_map(|y| (0..grid[0].len()).map(move |x| (y as i32, x as i32).into()))
        .map(|pair| simple_cost(pair, &grid, &mut visited))
        .sum()
}

fn in_bounds(p: Pair, grid: &[Vec<char>]) -> bool {
    p.y >= 0 && p.y < grid.len() as i32 && p.x >= 0 && p.x < grid[0].len() as i32
}

fn is_neighbour(a: Pair, b: Pair, grid: &[Vec<char>]) -> bool {
    in_bounds(a, grid) && grid[a.y as usize][a.x as usize] == grid[b.y as usize][b.x as usize]
}

fn simple_cost(from: Pair, grid: &[Vec<char>], visited: &mut Set) -> usize {
    if visited.contains(&from) {
        return 0;
    }
    visited.insert(from);
    let mut queue: Vec<Pair> = vec![from];
    let mut region_area = 1;
    let mut region_perimeter = 0;
    while let Some(coords) = queue.pop() {
        for dir in [
            Pair { x: 0, y: -1 },
            Pair { x: 0, y: 1 },
            Pair { x: -1, y: 0 },
            Pair { x: 1, y: 0 },
        ] {
            let next = coords + dir;
            if is_neighbour(next, coords, grid) {
                if visited.contains(&next) {
                    continue;
                }
                visited.insert(next);
                region_area += 1;
                queue.push(next);
            }
            if !is_neighbour(next, coords, grid) {
                region_perimeter += 1;
            }
        }
    }

    region_perimeter * region_area
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut visited = Set::new(grid.len(), grid[0].len());
    (0..grid.len())
        .flat_map(|y| (0..grid[0].len()).map(move |x| (y as i32, x as i32).into()))
        .map(|pair| bulk_cost(pair, &grid, &mut visited))
        .sum()
}

fn bulk_cost(from: Pair, grid: &[Vec<char>], visited: &mut Set) -> u32 {
    if visited.contains(&from) {
        return 0;
    }
    visited.insert(from);
    let mut queue: Vec<Pair> = vec![from];
    let mut region_area = 1;
    let mut region_sides = 0;
    while let Some(coords) = queue.pop() {
        for dir in [
            Pair { x: 0, y: -1 },
            Pair { x: 0, y: 1 },
            Pair { x: -1, y: 0 },
            Pair { x: 1, y: 0 },
        ] {
            let next = coords + dir;
            if is_neighbour(next, coords, grid) {
                if visited.contains(&next) {
                    continue;
                }
                visited.insert(next);
                region_area += 1;
                queue.push(next);
            }
        }
        region_sides += compute_sides(coords, grid);
    }
    region_sides * region_area
}

fn compute_sides(coords: Pair, grid: &[Vec<char>]) -> u32 {
    let up = is_neighbour(coords + Pair { x: 0, y: -1 }, coords, grid);
    let down = is_neighbour(coords + Pair { x: 0, y: 1 }, coords, grid);
    let left = is_neighbour(coords + Pair { x: -1, y: 0 }, coords, grid);
    let right = is_neighbour(coords + Pair { x: 1, y: 0 }, coords, grid);
    let top_left = is_neighbour(coords + Pair { x: -1, y: -1 }, coords, grid);
    let top_right = is_neighbour(coords + Pair { x: 1, y: -1 }, coords, grid);
    let bottom_left = is_neighbour(coords + Pair { x: -1, y: 1 }, coords, grid);
    let bottom_right = is_neighbour(coords + Pair { x: 1, y: 1 }, coords, grid);
    let in_corners = (up && right && !top_right) as u32
        + (up && left && !top_left) as u32
        + (down && right && !bottom_right) as u32
        + (down && left && !bottom_left) as u32;
    // diagonals are separated
    let out_corners = (!up && !right) as u32
        + (!up && !left) as u32
        + (!down && !right) as u32
        + (!down && !left) as u32;
    in_corners + out_corners
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

struct Set(Vec<Vec<bool>>);

impl Set {
    fn new(size_y: usize, size_x: usize) -> Self {
        Self(vec![vec![false; size_x]; size_y])
    }

    fn contains(&self, pair: &Pair) -> bool {
        if pair.y < 0 || pair.y >= self.len() as i32 {
            return false;
        }
        if pair.x < 0 || pair.x >= self[0].len() as i32 {
            return false;
        }
        self[pair.y as usize][pair.x as usize]
    }

    fn insert(&mut self, pair: Pair) {
        self[pair.y as usize][pair.x as usize] = true;
    }
}

impl Deref for Set {
    type Target = Vec<Vec<bool>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Set {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test_part1() {
    let input = "AAAA
BBCD
BBCC
EEEC";
    assert_eq!(part1(input), 140);
}

#[test]
fn test_part2() {
    let input = "AAAA
BBCD
BBCC
EEEC";
    assert_eq!(part2(input), 80);
}

#[test]
fn test_e_part2() {
    let input = "EE
EX
EE
EX
EE";
    assert_eq!(part2(input), 8 * 12 + 2 * 4);

    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    assert_eq!(part2(input), 236);
}

#[test]
fn test_diagonal_part2() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    assert_eq!(part2(input), 368);
}
