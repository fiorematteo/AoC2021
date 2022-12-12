use std::collections::{HashMap, HashSet, VecDeque};

const MAX_X: usize = 113;
const MAX_Y: usize = 41;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y)
            }
        }
    }
    solution(input, start)
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
    let mut starts = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'a' || c == 'S' {
                starts.push((x, y));
            }
        }
    }
    starts
        .iter()
        .map(|&start| solution(input, start))
        .min()
        .unwrap()
}

pub fn solution(input: &str, start: (usize, usize)) -> i32 {
    let mut end = (0, 0);
    let mut grid = [[0; MAX_X]; MAX_Y];
    for (y, line) in input.lines().enumerate() {
        for (x, mut c) in line.chars().enumerate() {
            match c {
                'S' => c = 'a',
                'E' => {
                    end = (x, y);
                    c = 'z'
                }
                _ => (),
            }
            grid[y][x] = c as i32 - 97;
        }
    }

    let mut nodes: VecDeque<_> = vec![(start, 0)].into();
    let mut dist: HashMap<(usize, usize), i32> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    dist.insert(start, 0);
    while !nodes.is_empty() {
        let (current, current_distance) = nodes.pop_front().unwrap();
        if current == end {
            return current_distance;
        }
        for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (
                (current.0 as i32).checked_add(i).unwrap(),
                (current.1 as i32).checked_add(j).unwrap(),
            );
            if x < 0 || x >= MAX_X as _ || y < 0 || y >= MAX_Y as _ {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if grid[y][x] - grid[current.1][current.0] > 1 {
                continue;
            }
            let new = (x, y);
            if !visited.insert(new) {
                continue;
            }
            nodes.push_back((new, current_distance + 1));
        }
    }
    i32::MAX
}
