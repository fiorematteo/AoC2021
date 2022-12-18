use std::collections::{BTreeSet, VecDeque};

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
    let cubes: Vec<_> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(",").map(|x| x.parse::<i32>().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();

    let count: i32 = cubes
        .iter()
        .map(|a| {
            cubes.iter().filter_map(move |b| {
                if a == b {
                    None
                } else {
                    Some(touching(a, b) as i32)
                }
            })
        })
        .flatten()
        .sum();
    (cubes.len() as i32 * 6) - count
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> i32 {
    let cubes: Vec<_> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(",").map(|x| x.parse::<i32>().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();

    let max_x = *cubes.iter().map(|(x, _, _)| x).max().unwrap();
    let max_y = *cubes.iter().map(|(_, y, _)| y).max().unwrap();
    let max_z = *cubes.iter().map(|(_, _, z)| z).max().unwrap();
    let min_x = *cubes.iter().map(|(x, _, _)| x).min().unwrap();
    let min_y = *cubes.iter().map(|(_, y, _)| y).min().unwrap();
    let min_z = *cubes.iter().map(|(_, _, z)| z).min().unwrap();

    let mut air: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
    let start = (min_x - 1, min_y - 1, min_z - 1);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();
        if n.0 > max_x + 1 || n.1 > max_y + 1 || n.2 > max_z + 1 {
            continue;
        }
        if n.0 < min_x - 1 || n.1 < min_y - 1 || n.2 < min_z - 1 {
            continue;
        }
        if !cubes.contains(&n) && !air.contains(&n) {
            air.insert(n);
            queue.push_back((n.0 - 1, n.1, n.2));
            queue.push_back((n.0 + 1, n.1, n.2));
            queue.push_back((n.0, n.1 - 1, n.2));
            queue.push_back((n.0, n.1 + 1, n.2));
            queue.push_back((n.0, n.1, n.2 - 1));
            queue.push_back((n.0, n.1, n.2 + 1));
        }
    }

    air.iter()
        .map(|a| cubes.iter().map(|c| touching(c, a) as i32))
        .flatten()
        .sum()
}

pub fn touching(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> bool {
    if a.0 == b.0 && a.1 == b.1 {
        a.2.abs_diff(b.2) == 1
    } else if a.0 == b.0 && a.2 == b.2 {
        a.1.abs_diff(b.1) == 1
    } else if a.1 == b.1 && a.2 == b.2 {
        a.0.abs_diff(b.0) == 1
    } else {
        false
    }
}
