use std::ops::Range;

#[aoc_generator(day22)]
pub fn generator(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let biggest_row = map.iter().map(|row| row.len()).max().unwrap();

    for row in map.iter_mut() {
        while row.len() < biggest_row {
            row.push(' ');
        }
    }

    let mut new_moves = Vec::new();
    let mut tmp = String::new();
    for c in moves.chars() {
        if c.is_ascii_digit() {
            tmp.push(c)
        } else {
            new_moves.push(Move::Move(tmp.parse().unwrap()));
            tmp = String::new();
            new_moves.push(Move::Rotate(c));
        }
    }
    if !tmp.is_empty() {
        new_moves.push(Move::Move(tmp.parse().unwrap()));
    }

    (map, new_moves)
}

#[aoc(day22, part1)]
pub fn part1((map, moves): &(Vec<Vec<char>>, Vec<Move>)) -> usize {
    let mut start_x = 0;
    for x in 0..map[0].len() {
        if map[0][x] == '.' {
            start_x = x;
            break;
        }
    }

    let (mut current_y, mut current_x) = (0_usize, start_x as usize);
    let mut direction = Direction::Right;

    for m in moves {
        match m {
            Move::Rotate(r) => match r {
                'R' => direction = direction.rotate_right(),
                'L' => direction = direction.rotate_left(),
                _ => unreachable!(),
            },
            Move::Move(mut n) => {
                while n > 0 {
                    let (mut new_y, mut new_x) = direction.apply((current_y, current_x));
                    if new_y < 0 {
                        new_y = map.len() as i64 - 1;
                    } else if new_y == map.len() as i64 {
                        new_y = 0;
                    }
                    if new_x < 0 {
                        new_x = map[0].len() as i64 - 1;
                    } else if new_x == map[0].len() as i64 {
                        new_x = 0;
                    }
                    if map[new_y as usize][new_x as usize] == '.' {
                        (current_y, current_x) = (new_y as usize, new_x as usize);
                    } else if map[new_y as usize][new_x as usize] == ' ' {
                        while map[new_y as usize][new_x as usize] == ' ' {
                            new_y = (new_y + direction.y()).rem_euclid(map.len() as i64);
                            new_x = (new_x + direction.x()).rem_euclid(map[0].len() as i64);
                        }
                        if map[new_y as usize][new_x as usize] == '.' {
                            (current_y, current_x) = (new_y as usize, new_x as usize);
                        }
                    }
                    n -= 1;
                }
            }
        }
    }
    1000 * (current_y + 1)
        + 4 * (current_x + 1)
        + match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

#[aoc(day22, part2)]
pub fn part2((map, moves): &(Vec<Vec<char>>, Vec<Move>)) -> usize {
    let face_size = map[0].iter().filter(|&&c| c != ' ').count();
    let mut regions = vec![];
    regions.push(extract_region(
        // 1
        map,
        0..face_size,
        face_size * 2..face_size * 3,
    ));
    regions.push(extract_region(
        // 2
        map,
        face_size..face_size * 2,
        0..face_size,
    ));
    regions.push(extract_region(
        // 3
        map,
        face_size..face_size * 2,
        face_size..face_size * 2,
    ));
    regions.push(extract_region(
        // 4
        map,
        face_size..face_size * 2,
        face_size * 2..face_size * 3,
    ));
    regions.push(extract_region(
        // 5
        map,
        face_size * 2..face_size * 3,
        face_size * 2..face_size * 3,
    ));
    regions.push(extract_region(
        // 6
        map,
        face_size * 2..face_size * 3,
        face_size * 3..face_size * 4,
    ));
    for row in &regions[0].iter(){}
    todo!()
}

pub fn rotate_region_right(region: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut out = vec![vec![' '; region.len()]; region.len()];
    for y in 0..region.len() {
        for x in 0..region.len() {
            out[x][region.len() - 1 - y] = region[y][x];
        }
    }
    out
}

pub fn extract_region(
    map: &[Vec<char>],
    y_range: Range<usize>,
    x_range: Range<usize>,
) -> Vec<Vec<char>> {
    map[y_range]
        .iter()
        .map(|v| v[x_range.clone()].to_vec())
        .collect()
}

pub fn region(y: usize, x: usize, face_size: usize) -> usize {
    if (face_size * 2..face_size * 3).contains(&x) && (0..face_size).contains(&y) {
        return 1;
    }
    if (face_size * 2..face_size * 3).contains(&y) {
        if (0..face_size).contains(&x) {
            return 2;
        }
        if (face_size..face_size * 2).contains(&x) {
            return 3;
        }
        if (face_size * 3..face_size * 4).contains(&x) {
            return 4;
        }
    }
    if (face_size * 2..face_size * 3).contains(&y) {
        if (face_size * 2..face_size * 3).contains(&x) {
            return 5;
        }
        if (face_size * 3..face_size * 4).contains(&x) {
            return 6;
        }
    }
    unreachable!()
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Rotate(char),
    Move(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    pub fn rotate_left(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }
    pub fn rotate_right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    pub fn apply(self, (y, x): (usize, usize)) -> (i64, i64) {
        let new_y = y as i64 + self.y();
        let new_x = x as i64 + self.x();
        (new_y, new_x)
    }

    pub fn x(&self) -> i64 {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }

    pub fn y(&self) -> i64 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }
}

#[test]
fn part1_test() {
    let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    assert_eq!(part1(&generator(input)), 6032);
}

#[test]
fn part2_test() {
    let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    assert_eq!(part2(&generator(input)), 5031);
}
