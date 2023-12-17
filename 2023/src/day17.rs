use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[aoc(day17, part1)]
fn part1(input: &str) -> i64 {
    solve(input, true)
}

#[aoc(day17, part2)]
fn part2(input: &str) -> i64 {
    solve(input, false)
}

fn solve(input: &str, part1: bool) -> i64 {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect();

    let mut q: BinaryHeap<_> = vec![Reverse((0, 0, 0, Direction::Down, -1))].into();
    let mut dist = HashMap::new();
    while let Some(node) = q.pop() {
        let Reverse((heat, y, x, direction, line_lenght)) = node;

        if y == map.len() as i64 - 1 && x == map[0].len() as i64 - 1 {
            return heat;
        }
        if let Some(old_heat) = dist.insert((y, x, direction, line_lenght), heat) {
            assert!(heat >= old_heat);
            continue;
        }

        for new_dir in [direction.rotate_left(), direction.rotate_right(), direction] {
            let new_line_lenght = if new_dir == direction {
                line_lenght + 1
            } else {
                1
            };
            let (new_y, new_x) = new_dir.apply((y, x));
            let special_condition = if part1 {
                new_line_lenght <= 3
            } else {
                new_line_lenght <= 10
                    && (new_dir == direction || line_lenght >= 4 || line_lenght == -1)
            };

            if 0 <= new_y
                && new_y < map.len() as i64
                && 0 <= new_x
                && new_x < map[0].len() as i64
                && special_condition
            {
                let new_heat = heat + map[new_y as usize][new_x as usize];
                q.push(Reverse((
                    new_heat,
                    new_y,
                    new_x,
                    new_dir,
                    new_line_lenght,
                )));
            }
        }
    }
    unreachable!()
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn apply(&self, (y, x): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (y - 1, x),
            Direction::Right => (y, x + 1),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
        }
    }
}

#[test]
fn test_part1() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    assert_eq!(part1(input), 102);
}

#[test]
fn test_part2() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    assert_eq!(part2(input), 94);
}
