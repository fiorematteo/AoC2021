use std::collections::HashSet;

#[aoc(day18, part1, flood_fill)]
fn part1(input: &str) -> usize {
    let mut map = HashSet::new();
    let mut y = 0;
    let mut x = 0;
    map.insert((y, x));
    for line in input.lines() {
        let mut s = line.split_whitespace();
        let dir: Direction = s.next().unwrap().into();
        let steps: i64 = s.next().unwrap().parse().unwrap();
        for _ in 0..steps {
            (y, x) = dir.apply(y, x);
            map.insert((y, x));
        }
    }
    assert!(!map.contains(&(1, 1)));
    flood_fill(&mut map, 1, 1);
    map.len()
}

#[aoc(day18, part1, sholace)]
fn part1_sholace(input: &str) -> i64 {
    let mut edges = vec![];
    for line in input.lines() {
        let mut s = line.split_whitespace();
        let dir: Direction = s.next().unwrap().into();
        let steps: i64 = s.next().unwrap().parse().unwrap();
        edges.push((steps, dir));
    }
    sholace(&edges)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    let mut edges = vec![];
    for line in input.lines() {
        let mut s = line.split_whitespace().skip(2);
        let s = s.next().unwrap();
        let s = &s[2..s.len() - 1];
        let (steps, dir) = s.split_at(5);
        let steps = i64::from_str_radix(steps, 16).unwrap();
        let dir: Direction = dir.into();
        edges.push((steps, dir));
    }

    sholace(&edges)
}

fn sholace(edges: &[(i64, Direction)]) -> i64 {
    let mut y = 0;
    let mut x = 0;
    let mut perimiter = 0;
    let mut area = 0;
    for &(steps, dir) in edges {
        perimiter += steps;
        let (y1, x1) = dir.apply_s(y, x, steps);
        area += (y + y1) * (x - x1);
        (y, x) = (y1, x1);
    }
    (area + perimiter) / 2 + 1
}

fn flood_fill(map: &mut HashSet<(i64, i64)>, y: i64, x: i64) {
    for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let ny = y + dy;
        let nx = x + dx;
        if !map.contains(&(ny, nx)) {
            map.insert((ny, nx));
            flood_fill(map, ny, nx);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn apply(&self, y: i64, x: i64) -> (i64, i64) {
        match self {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Right => (y, x + 1),
            Direction::Left => (y, x - 1),
        }
    }

    fn apply_s(&self, y: i64, x: i64, steps: i64) -> (i64, i64) {
        match self {
            Direction::Up => (y - steps, x),
            Direction::Down => (y + steps, x),
            Direction::Right => (y, x + steps),
            Direction::Left => (y, x - steps),
        }
    }
}

#[test]
fn test_part1() {
    let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    assert_eq!(part1(input), 62);
}

#[test]
fn test_part2() {
    let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    assert_eq!(part2(input), 952408144115);
}
