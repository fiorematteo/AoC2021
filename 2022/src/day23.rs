use std::collections::{BTreeMap, BTreeSet};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let (elfs, _) = solution(input, 9);

    let mut count = 0;
    let min_x = *elfs.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *elfs.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *elfs.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *elfs.iter().map(|(_, y)| y).max().unwrap();
    for y in min_y.min(max_y)..=min_y.max(max_y) {
        for x in min_x.min(max_x)..=min_x.max(max_x) {
            if !elfs.contains(&(x, y)) {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> i64 {
    solution(input, i64::MAX).1 + 1
}

pub fn solution(input: &str, max_iterations: i64) -> (BTreeSet<(i64, i64)>, i64) {
    let mut elfs = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elfs.insert((x as i64, y as i64));
            }
        }
    }

    let mut directions = [
        [(-1, -1), (0, -1), (1, -1)], // north
        [(-1, 1), (0, 1), (1, 1)],    // south
        [(-1, -1), (-1, 0), (-1, 1)], // west
        [(1, -1), (1, 0), (1, 1)],    // east
    ];

    let mut i = 0;
    loop {
        let mut declarations = BTreeMap::new();
        for &elf in &elfs {
            if directions.iter().all(|d| {
                d.iter()
                    .all(|p| elfs.get(&(elf.0 + p.0, elf.1 + p.1)).is_none())
            }) {
                continue;
            }

            for d in &directions {
                if d.iter()
                    .all(|p| elfs.get(&(elf.0 + p.0, elf.1 + p.1)).is_none())
                {
                    let new_position = (elf.0 + d[1].0, elf.1 + d[1].1);
                    declarations.insert(elf, new_position);
                    break;
                }
            }
        }

        let mut copy = elfs.clone();
        elfs = BTreeSet::new();
        for (elf, next) in &declarations {
            if declarations.iter().filter(|(_, b)| b == &next).count() == 1 {
                copy.remove(elf);
                elfs.insert(*next);
            }
        }
        if elfs.is_empty() {
            return (elfs, i);
        }
        for e in &copy {
            elfs.insert(*e);
        }
        if i == max_iterations {
            return (elfs, i);
        }
        directions.rotate_left(1);
        i += 1;
    }
}

pub fn dump_elfs(elfs: &BTreeSet<(i64, i64)>) {
    let min_x = *elfs.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = *elfs.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = *elfs.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = *elfs.iter().map(|(_, y)| y).max().unwrap() + 1;

    println!("len: {}", elfs.len());
    for y in min_y.min(max_y)..=min_y.max(max_y) {
        for x in min_x.min(max_x)..=min_x.max(max_x) {
            if elfs.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
