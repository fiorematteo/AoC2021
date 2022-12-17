use std::{collections::BTreeSet, ops::Range};

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let (x, y) = l.split_once(", ").unwrap();
            let x: i32 = x.split_once('=').unwrap().1.parse().unwrap();
            let y: i32 = y.split_once('=').unwrap().1.parse().unwrap();

            let (bx, by) = r.split_once(", ").unwrap();
            let bx: i32 = bx.split_once('=').unwrap().1.parse().unwrap();
            let by: i32 = by.split_once('=').unwrap().1.parse().unwrap();
            let d = x.abs_diff(bx) + y.abs_diff(by);
            (x, y, d as _)
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[(i32, i32, i32)]) -> usize {
    const Y: i32 = 2000000;

    let mut covered = BTreeSet::new();
    for &(x, y, d) in input {
        let d = d - Y.abs_diff(y) as i32;
        for i in x - d..x + d {
            covered.insert(i);
        }
    }
    covered.len()
}

#[aoc(day15, part2)]
pub fn part2(input: &[(i32, i32, i32)]) -> u64 {
    const Y: i32 = 4000000;

    for y in 0..Y {
        let mut covered: Vec<Range<i32>> = Vec::new();

        for (sx, sy, sd) in input.iter() {
            let dy = sy.abs_diff(y as _) as i32;

            if dy < *sd {
                let dx = sd - dy;
                covered.push(0.max(sx - dx)..Y.min(sx + dx));
            }
        }

        covered.sort_by(|a, b| (a.start, a.end).cmp(&(b.start, b.end)));
        let mut combined_ranges: Vec<Range<i32>> = Vec::new();
        combined_ranges.push(covered[0].clone());

        for r in covered.clone() {
            let last = combined_ranges.last_mut().unwrap();
            if r.start <= last.end {
                *last = last.start..r.end.max(last.end);
            } else {
                combined_ranges.push(r);
            }
        }

        if combined_ranges.len() > 1 {
            return 4000000u64
                .checked_mul((combined_ranges.first().unwrap().end) as u64 + 1)
                .unwrap()
                + y as u64;
        }
    }
    unreachable!();
}
