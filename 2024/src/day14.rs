use std::ops::{Add, AddAssign, Sub};

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let width = 101;
    let height = 103;
    solve(input, width, height)
}

pub fn solve(input: &str, width: i64, height: i64) -> usize {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    let mut grid = vec![vec![0; width as usize]; height as usize];
    for line in input.lines() {
        let (position, speed) = line.split_once(' ').unwrap();
        let (x, y) = position
            .split_once("p=")
            .unwrap()
            .1
            .split_once(',')
            .unwrap();
        let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        let (vx, vy) = speed.split_once("v=").unwrap().1.split_once(',').unwrap();
        let (vx, vy) = (vx.parse::<i64>().unwrap(), vy.parse::<i64>().unwrap());
        let position = Pair {
            x: (x + vx * 100).rem_euclid(width),
            y: (y + vy * 100).rem_euclid(height),
        };

        if position.x < width / 2 && position.y < height / 2 {
            top_left += 1;
        }
        if position.x > width / 2 && position.y < height / 2 {
            top_right += 1;
        }
        if position.x < width / 2 && position.y > height / 2 {
            bottom_left += 1;
        }
        if position.x > width / 2 && position.y > height / 2 {
            bottom_right += 1;
        }
        grid[position.y as usize][position.x as usize] += 1;
    }
    top_left * top_right * bottom_left * bottom_right
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let width = 101;
    let height = 103;
    let mut lowest_danger = 100000000;
    let mut lowest_iteration = 0;
    let mut robots: Vec<(Pair, Pair)> = input
        .lines()
        .map(|line| {
            let (position, speed) = line.split_once(' ').unwrap();
            let (x, y) = position
                .split_once("p=")
                .unwrap()
                .1
                .split_once(',')
                .unwrap();
            let position = Pair {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };
            let (x, y) = speed.split_once("v=").unwrap().1.split_once(',').unwrap();
            let speed = Pair {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };
            (position, speed)
        })
        .collect();

    for i in 0..10000 {
        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;
        for (position, speed) in &mut robots {
            *position += *speed;
            position.x = position.x.rem_euclid(width);
            position.y = position.y.rem_euclid(height);

            if position.x < width / 2 && position.y < height / 2 {
                top_left += 1;
            }
            if position.x > width / 2 && position.y < height / 2 {
                top_right += 1;
            }
            if position.x < width / 2 && position.y > height / 2 {
                bottom_left += 1;
            }
            if position.x > width / 2 && position.y > height / 2 {
                bottom_right += 1;
            }
        }
        let danger_level = top_left * top_right * bottom_left * bottom_right;
        if danger_level < lowest_danger {
            lowest_iteration = i;
            lowest_danger = danger_level;
            // let mut grid = vec![vec![false; width as usize]; height as usize];
            // for (position, _) in &robots {
            //     grid[position.y as usize][position.x as usize] = true;
            // }
            // println!("Danger level: {}, iteration: {}", danger_level, i);
            // for row in grid {
            //     for cell in row {
            //         print!("{}", if cell { '#' } else { '.' });
            //     }
            //     println!();
            // }
            // println!("###############");
        }
    }

    lowest_iteration + 1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    assert_eq!(solve(input, 11, 7), 12);
}
