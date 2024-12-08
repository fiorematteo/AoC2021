use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[aoc_generator(day8)]
fn generator(input: &str) -> (HashMap<char, Vec<Pair>>, i32, i32) {
    let mut grid: HashMap<char, Vec<Pair>> = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, row) in input.lines().enumerate() {
        max_y = y as i32;
        for (x, a) in row.chars().enumerate() {
            max_x = x as i32;
            if a != '.' {
                grid.entry(a)
                    .or_default()
                    .push(Pair::from((y as _, x as _)));
            }
        }
    }
    (grid, max_y, max_x)
}

#[aoc(day8, part1)]
fn part1(&(ref grid, max_y, max_x): &(HashMap<char, Vec<Pair>>, i32, i32)) -> u32 {
    let mut antipodes = HashSet::new();
    for (_, antennas) in grid.iter() {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let diff = antennas[i] - antennas[j];
                let new_antipode = antennas[i] + diff;
                if new_antipode.y >= 0
                    && new_antipode.y <= max_y
                    && new_antipode.x >= 0
                    && new_antipode.x <= max_x
                {
                    antipodes.insert(new_antipode);
                }
                let new_antipode = antennas[j] - diff;
                if new_antipode.y >= 0
                    && new_antipode.y <= max_y
                    && new_antipode.x >= 0
                    && new_antipode.x <= max_x
                {
                    antipodes.insert(new_antipode);
                }
            }
        }
    }
    antipodes.len() as _
}

#[aoc(day8, part2)]
fn part2(&(ref grid, max_y, max_x): &(HashMap<char, Vec<Pair>>, i32, i32)) -> u32 {
    let mut antipodes = HashSet::new();
    for (_, antennas) in grid.iter() {
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                antipodes.insert(antennas[i]);
                antipodes.insert(antennas[j]);
                let diff = antennas[i] - antennas[j];
                // positive
                let mut new_antipode = antennas[i] + diff;
                while new_antipode.y >= 0
                    && new_antipode.y <= max_y
                    && new_antipode.x >= 0
                    && new_antipode.x <= max_x
                {
                    antipodes.insert(new_antipode);
                    new_antipode = new_antipode + diff;
                }
                // negative
                let mut new_antipode = antennas[j] - diff;
                while new_antipode.y >= 0
                    && new_antipode.y <= max_y
                    && new_antipode.x >= 0
                    && new_antipode.x <= max_x
                {
                    antipodes.insert(new_antipode);
                    new_antipode = new_antipode - diff;
                }
            }
        }
    }
    antipodes.len() as _
}

#[test]
fn test_part1() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part1(&generator(input)), 14);
}

#[test]
fn test_part2() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part2(&generator(input)), 34);
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
