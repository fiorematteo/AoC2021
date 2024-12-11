use std::collections::HashMap;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|s| solve(s.parse().unwrap(), 0, 25, &mut cache))
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|s| solve(s.parse().unwrap(), 0, 75, &mut cache))
        .sum()
}

fn solve(stone: u64, depth: u64, max_depth: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(&out) = cache.get(&(stone, depth)) {
        return out;
    }
    if depth == max_depth {
        return 1;
    }
    let out = if stone == 0 {
        solve(1, depth + 1, max_depth, cache)
    } else {
        let digits = stone.ilog10() as usize + 1;
        if digits % 2 == 0 {
            let stone_str = stone.to_string();
            let half_point = digits / 2;
            let left: u64 = stone_str[..half_point].parse().unwrap();
            let right: u64 = stone_str[half_point..].parse().unwrap();
            solve(left, depth + 1, max_depth, cache) + solve(right, depth + 1, max_depth, cache)
        } else {
            solve(stone * 2024, depth + 1, max_depth, cache)
        }
    };
    cache.insert((stone, depth), out);
    out
}

#[test]
fn part1_test() {
    let input = "125 17";
    assert_eq!(part1(input), 55312)
}
