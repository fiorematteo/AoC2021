#![allow(clippy::needless_range_loop)]
use std::collections::HashMap;

#[aoc(day14, part1)]
fn part1(input: &str) -> i64 {
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    tilt_up(&mut map);

    let mut total: i64 = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 'O' {
                total += (map.len() - y) as i64;
            }
        }
    }
    total
}

#[aoc(day14, part2)]
fn part2(input: &str) -> i64 {
    let mut cycle = HashMap::new();
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut i = 0;
    let big = 1_000_000_000;
    while i < big {
        tilt_up(&mut map);
        tilt_left(&mut map);
        tilt_down(&mut map);
        tilt_right(&mut map);
        let hash: String = map.iter().flat_map(|l| l.iter()).collect();
        if let Some(e) = cycle.get(&hash) {
            i = big - (big - i) % (i - e);
        } else {
            cycle.insert(hash, i);
        }
        i += 1;
    }

    let mut total: i64 = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 'O' {
                total += (map.len() - y) as i64;
            }
        }
    }
    total
}

fn tilt_down(map: &mut Vec<Vec<char>>) {
    for x in 0..map[0].len() {
        let mut rock_count = 0;
        let mut first_empty_spot = None;
        for y in (0..map.len()).rev() {
            match map[y][x] {
                '.' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(y);
                    }
                }
                '#' => {
                    if let Some(first) = first_empty_spot {
                        for rock_y in first - rock_count + 1..=first {
                            map[rock_y][x] = 'O';
                        }
                        rock_count = 0;
                        first_empty_spot = None;
                    };
                }
                'O' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(y);
                    }
                    rock_count += 1;
                    map[y][x] = '.';
                }
                _ => unreachable!(),
            };
        }
        if let Some(first) = first_empty_spot {
            for rock_y in first + 1 - rock_count..=first {
                map[rock_y][x] = 'O';
            }
        };
    }
}

fn tilt_up(map: &mut Vec<Vec<char>>) {
    for x in 0..map[0].len() {
        let mut rock_count = 0;
        let mut first_empty_spot = None;
        for y in 0..map.len() {
            match map[y][x] {
                '.' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(y);
                    }
                }
                '#' => {
                    if let Some(first) = first_empty_spot {
                        for rock_y in first..first + rock_count {
                            map[rock_y][x] = 'O';
                        }
                        rock_count = 0;
                        first_empty_spot = None;
                    };
                }
                'O' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(y);
                    }
                    rock_count += 1;
                    map[y][x] = '.';
                }
                _ => unreachable!(),
            };
        }
        if let Some(first) = first_empty_spot {
            for rock_y in first..first + rock_count {
                map[rock_y][x] = 'O';
            }
        };
    }
}

fn tilt_left(map: &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        let mut rock_count = 0;
        let mut first_empty_spot = None;
        for x in 0..map[0].len() {
            match map[y][x] {
                '.' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(x);
                    }
                }
                '#' => {
                    if let Some(first) = first_empty_spot {
                        for rock_x in first..first + rock_count {
                            map[y][rock_x] = 'O';
                        }
                        rock_count = 0;
                        first_empty_spot = None;
                    };
                }
                'O' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(x);
                    }
                    rock_count += 1;
                    map[y][x] = '.';
                }
                _ => unreachable!(),
            };
        }
        if let Some(first) = first_empty_spot {
            for rock_x in first..first + rock_count {
                map[y][rock_x] = 'O';
            }
        };
    }
}

fn tilt_right(map: &mut Vec<Vec<char>>) {
    for y in 0..map.len() {
        let mut rock_count = 0;
        let mut first_empty_spot = None;
        for x in (0..map[0].len()).rev() {
            match map[y][x] {
                '.' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(x);
                    }
                }
                '#' => {
                    if let Some(first) = first_empty_spot {
                        for rock_x in first - rock_count + 1..=first {
                            map[y][rock_x] = 'O';
                        }
                        rock_count = 0;
                        first_empty_spot = None;
                    };
                }
                'O' => {
                    if first_empty_spot.is_none() {
                        first_empty_spot = Some(x);
                    }
                    rock_count += 1;
                    map[y][x] = '.';
                }
                _ => unreachable!(),
            };
        }
        if let Some(first) = first_empty_spot {
            for rock_x in first - rock_count + 1..=first {
                map[y][rock_x] = 'O';
            }
        };
    }
}

#[test]
fn test_part1() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    assert_eq!(part1(input), 136);
}

#[test]
fn test_part2() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    assert_eq!(part2(input), 64);
}
