use regex::Regex;
use std::ops::{Add, Sub};

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input, false)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input, true)
}

pub fn solve(input: &str, part2: bool) -> i64 {
    let button = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let build_pair = |line, re: &Regex| {
        let matches = re.captures(line).unwrap();
        Pair {
            x: matches.get(1).unwrap().as_str().parse().unwrap(),
            y: matches.get(2).unwrap().as_str().parse().unwrap(),
        }
    };
    let machines = input.split("\n\n").map(|block| {
        let mut lines = block.lines();
        let a = build_pair(lines.next().unwrap(), &button);
        let b = build_pair(lines.next().unwrap(), &button);
        let target = build_pair(lines.next().unwrap(), &prize);
        (a, b, target)
    });
    let mut total = 0;
    for (a, b, mut target) in machines {
        if part2 {
            target.x += 10000000000000;
            target.y += 10000000000000;
        }
        // best solution is also the only solution?
        // | a.x b.x |
        // | a.y b.y |
        let det = a.x * b.y - a.y * b.x;
        assert_ne!(det, 0);
        // cramer
        // | target.x b.x |
        // | target.y b.y |
        let x = target.x * b.y - target.y * b.x;
        // | a.x target.x |
        // | a.y target.y |
        let y = a.x * target.y - a.y * target.x;
        if x % det == 0 && y % det == 0 {
            total += x / det * 3 + y / det;
        }
    }
    total
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i64,
    y: i64,
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
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    assert_eq!(part1(input), 480);
}

#[test]
fn test_part2() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    assert_eq!(part2(input), 875318608908);
}
