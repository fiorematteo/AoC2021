use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    });
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    });
    let mut right_map = HashMap::new();
    for r in right {
        if let Some(v) = right_map.get(&r) {
            right_map.insert(r, *v + 1);
        } else {
            right_map.insert(r, 1);
        }
    }
    left.into_iter()
        .map(|l| l * right_map.get(&l).unwrap_or(&0))
        .sum()
}

#[test]
fn part1_test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part1(input), 11);
}

#[test]
fn part2_test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part2(input), 31);
}
