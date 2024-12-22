use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Sub},
};

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 25)
}

pub fn solve(input: &str, depth: usize) -> usize {
    let mut cache = BTreeMap::new();
    let mut total = 0;
    for (number, value) in input
        .lines()
        .map(|n| (n, n[..n.len() - 1].parse::<usize>().unwrap()))
    {
        let mut pos = number_keypad('A');
        let instruction_len = number
            .chars()
            .map(|c| {
                let target = number_keypad(c);
                let diff = target - pos;
                let out = if pos.y == 3 && target.x == 0 {
                    // go up
                    compute_moves(diff, depth, false, &mut cache)
                } else if pos.x == 0 && target.y == 3 {
                    // go right
                    compute_moves(diff, depth, true, &mut cache)
                } else {
                    let horizontal = compute_moves(diff, depth, true, &mut cache);
                    let vertical = compute_moves(diff, depth, false, &mut cache);
                    horizontal.min(vertical)
                };
                pos = target;
                out
            })
            .sum::<usize>();
        total += value * instruction_len;
    }
    total
}

fn compute_moves(
    diff: Pair,
    depth: usize,
    horizontal_first: bool,
    cache: &mut BTreeMap<(Pair, usize, bool), usize>,
) -> usize {
    let key = (diff, depth, horizontal_first);
    if let Some(&value) = cache.get(&key) {
        return value;
    }
    let mut sequence = vec![if diff.y < 0 { '^' } else { 'v' }; diff.y.abs() as usize];
    sequence.extend(vec![
        if diff.x < 0 { '<' } else { '>' };
        diff.x.abs() as usize
    ]);
    if horizontal_first {
        sequence.reverse();
    }
    // every sequence must end with an activation
    sequence.push('A');

    if depth == 0 {
        return sequence.len();
    }

    let mut pos = arrow_keypad('A');
    let out = sequence
        .into_iter()
        .map(|c| {
            let target = arrow_keypad(c);
            let diff = target - pos;
            let out = if target == arrow_keypad('<') && pos.y == 0 {
                // go down
                compute_moves(diff, depth - 1, false, cache)
            } else if pos == arrow_keypad('<') && target.y == 0 {
                // go right
                compute_moves(diff, depth - 1, true, cache)
            } else {
                let horizontal = compute_moves(diff, depth - 1, true, cache);
                let vertical = compute_moves(diff, depth - 1, false, cache);
                horizontal.min(vertical)
            };
            pos = target;
            out
        })
        .sum::<usize>();
    cache.insert(key, out);
    out
}

fn number_keypad(c: char) -> Pair {
    // '7', '8', '9'
    // '4', '5', '6'
    // '1', '2', '3'
    // ' ', '0', 'A'
    match c {
        '7' => Pair { x: 0, y: 0 },
        '8' => Pair { x: 1, y: 0 },
        '9' => Pair { x: 2, y: 0 },
        '4' => Pair { x: 0, y: 1 },
        '5' => Pair { x: 1, y: 1 },
        '6' => Pair { x: 2, y: 1 },
        '1' => Pair { x: 0, y: 2 },
        '2' => Pair { x: 1, y: 2 },
        '3' => Pair { x: 2, y: 2 },
        ' ' => Pair { x: 0, y: 3 },
        '0' => Pair { x: 1, y: 3 },
        'A' => Pair { x: 2, y: 3 },
        _ => unreachable!(),
    }
}

fn arrow_keypad(c: char) -> Pair {
    // ' ', '^', 'A'
    // '<', 'v', '>'
    match c {
        ' ' => Pair { x: 0, y: 0 },
        '^' => Pair { x: 1, y: 0 },
        'A' => Pair { x: 2, y: 0 },
        '<' => Pair { x: 0, y: 1 },
        'v' => Pair { x: 1, y: 1 },
        '>' => Pair { x: 2, y: 1 },
        _ => unreachable!(),
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

#[test]
fn test_part1() {
    let input = "029A
980A
179A
456A
379A";
    assert_eq!(part1(input), 126384);
}

#[test]
fn test_part2() {
    let input = "029A
980A
179A
456A
379A";
    assert_eq!(part2(input), 154115708116294);
}
