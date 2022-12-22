use std::collections::BTreeMap;

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map: BTreeMap<Position, char> = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64).into(), c))
        })
        .filter(|(_, c)| *c != ' ')
        .collect();

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

    let mut current: Position = *map
        .iter()
        .filter(|&(&p, &c)| p.y == 0 && c == '.')
        .min_by_key(|&(&p, &_)| p.x)
        .unwrap()
        .0;
    let mut direction: Position = (1, 0).into();

    new_moves.iter().for_each(|&m| match m {
        Move::Move(n) => {
            for _ in 0..n {
                if let Some(&c) = map.get(&(current + direction)) {
                    if c == '#' {
                        break;
                    }
                    current = current + direction;
                    continue;
                }

                let mut current_copy = current - direction;
                let mut prev_char = ' ';
                while let Some(&next) = map.get(&current_copy) {
                    current_copy = current_copy - direction;
                    prev_char = next;
                }
                if prev_char == '.' {
                    current = current_copy;
                }
            }
        }
        Move::Rotate(c) => match c {
            'R' => direction = direction.rotate_right(),
            'L' => direction = direction.rotate_left(),
            _ => unreachable!(),
        },
    });
    (current.y + 1) * 1000
        + (current.x + 1) * 4
        + match direction {
            Position { x: 1, y: 0 } => 0,  // right
            Position { x: 0, y: 1 } => 1,  // down
            Position { x: -1, y: 0 } => 2, // left
            Position { x: 0, y: -1 } => 3, // up
            _ => unreachable!(),
        }
}

pub fn dump_map(map: &BTreeMap<Position, char>, player: Position) {
    let min_x = map.iter().map(|(&p, _)| p.x).min().unwrap();
    let max_x = map.iter().map(|(&p, _)| p.x).max().unwrap();
    let min_y = map.iter().map(|(&p, _)| p.y).min().unwrap();
    let max_y = map.iter().map(|(&p, _)| p.y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = map.get(&(x, y).into()).unwrap_or(&' ');
            if Position::from((x, y)) == player {
                print!("@");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Rotate(char),
    Move(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn rotate_left(self) -> Self {
        match self {
            Position { x: 1, y: 0 } => (0, -1).into(),
            Position { x: -1, y: 0 } => (0, 1).into(),
            Position { x: 0, y: 1 } => (1, 0).into(),
            Position { x: 0, y: -1 } => (-1, 0).into(),
            _ => unreachable!(),
        }
    }
    pub fn rotate_right(self) -> Self {
        match self {
            Position { x: 1, y: 0 } => (0, 1).into(),
            Position { x: -1, y: 0 } => (0, -1).into(),
            Position { x: 0, y: 1 } => (-1, 0).into(),
            Position { x: 0, y: -1 } => (1, 0).into(),
            _ => unreachable!(),
        }
    }
}

impl From<(i64, i64)> for Position {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x.checked_add(rhs.x).unwrap(),
            y: self.y.checked_add(rhs.y).unwrap(),
        }
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x.checked_sub(rhs.x).unwrap(),
            y: self.y.checked_sub(rhs.y).unwrap(),
        }
    }
}
