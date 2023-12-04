use std::collections::{BTreeMap, BTreeSet};

#[aoc(day17, part1)]
pub fn part1(input: &str) -> u64 {
    let mut jets = input.chars().cycle();
    let line = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let elle = vec![(2, 0), (2, 1), (2, 2), (0, 0), (1, 0)];
    let vertical_line = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let block = vec![(0, 0), (0, 1), (1, 0), (1, 1)];

    let mut rocks: BTreeSet<Point> = BTreeSet::from_iter((-10..10).map(|i| (i, 0).into()));
    let pieces: Vec<Vec<Point>> = [line, plus, elle, vertical_line, block]
        .iter()
        .map(|piece| piece.iter().map(|&point| Point::from(point)).collect())
        .collect();
    let mut pieces = pieces.iter().cycle();
    let mut tallest_rock = 0;

    for _ in 0..2023 {
        // spawn new piece
        let mut piece = pieces.next().unwrap().clone();
        let bottom_edge = piece.iter().min_by_key(|p| p.y).unwrap().y;
        tallest_rock = rocks.iter().max_by_key(|p| p.y).unwrap().y;
        for point in piece.iter_mut() {
            point.y += tallest_rock + bottom_edge + 4;
            point.x += 2;
        }
        loop {
            // push
            let direction = match jets.next().unwrap() {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };
            let valid_move = !piece.iter().any(|p| {
                let x = p.x + direction;
                rocks.contains(&(x, p.y).into()) || !(0..7).contains(&x)
            });
            if valid_move {
                for point in piece.iter_mut() {
                    point.x += direction;
                }
            }

            // fall
            let valid_move = !piece.iter().any(|p| rocks.contains(&(p.x, p.y - 1).into()));
            if valid_move {
                for point in piece.iter_mut() {
                    point.y -= 1;
                }
            } else {
                for point in piece.iter_mut() {
                    rocks.insert(point.clone());
                }
                break;
            }
        }
    }
    tallest_rock as _
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> isize {
    let mut jets = input.chars().enumerate().cycle().peekable();
    let line = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let elle = vec![(2, 0), (2, 1), (2, 2), (0, 0), (1, 0)];
    let vertical_line = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let block = vec![(0, 0), (0, 1), (1, 0), (1, 1)];

    let mut rocks: BTreeSet<Point> = BTreeSet::from_iter((-10i32..10).map(|i| (i, 0).into()));
    let pieces: Vec<Vec<Point>> = [line, plus, elle, vertical_line, block]
        .iter()
        .map(|piece| piece.iter().map(|&point| Point::from(point)).collect())
        .collect();
    let mut pieces = pieces.iter().enumerate().cycle().peekable();
    let mut tallest_rock = 0isize;

    let mut cache: BTreeMap<(isize, isize), (isize, isize)> = BTreeMap::new();

    const BIG: isize = 1_000_000_000_000;
    for big_index in 0..BIG {
        tallest_rock = rocks.iter().max_by_key(|p| p.y).unwrap().y as _;

        let key = (
            pieces.peek().unwrap().0 as isize,
            jets.peek().unwrap().0 as isize,
        );
        if let Some((n, h)) = cache.get(&key) {
            let d = (BIG - big_index) / (n - big_index);
            let m = (BIG - big_index) % (n - big_index);
            if m == 0 {
                return tallest_rock + (h - tallest_rock) * d;
            }
        } else {
            cache.insert(key, (big_index, tallest_rock));
        }

        // spawn new piece
        let (_, piece) = pieces.next().unwrap();
        let mut piece = piece.clone();
        let bottom_edge = piece.iter().min_by_key(|p| p.y).unwrap().y;
        for point in piece.iter_mut() {
            point.y += tallest_rock as i64 + bottom_edge + 4;
            point.x += 2;
        }
        loop {
            // push
            let (_, jet) = jets.next().unwrap();
            let direction = jet as u8 as i64 - 61;
            let valid_move = !piece.iter().any(|p| {
                let x = p.x + direction;
                rocks.contains(&(x, p.y).into()) || !(0..7).contains(&x)
            });
            if valid_move {
                for point in piece.iter_mut() {
                    point.x += direction;
                }
            }

            // fall
            let valid_move = !piece.iter().any(|p| rocks.contains(&(p.x, p.y - 1).into()));
            if valid_move {
                for point in piece.iter_mut() {
                    point.y -= 1;
                }
            } else {
                for point in piece.iter_mut() {
                    rocks.insert(point.clone());
                }
                break;
            }
        }
    }
    tallest_rock as _
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl<T: Into<i64>> From<(T, T)> for Point {
    fn from((x, y): (T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}
