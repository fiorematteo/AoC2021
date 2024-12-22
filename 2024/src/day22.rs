use std::collections::{BTreeMap, BTreeSet};

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let mut total = 0;
    for mut secret_number in input.lines().map(|line| line.parse::<usize>().unwrap()) {
        for _ in 0..2000 {
            secret_number = ((secret_number * 64) ^ secret_number) % 16777216;
            secret_number = ((secret_number / 32) ^ secret_number) % 16777216;
            secret_number = ((secret_number * 2048) ^ secret_number) % 16777216;
        }
        total += secret_number;
    }
    total
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let mut profit: BTreeMap<(i64, i64, i64, i64), i64> = BTreeMap::new();
    for mut secret_number in input.lines().map(|line| line.parse::<i64>().unwrap()) {
        let mut digits: Vec<_> = vec![secret_number % 10];
        for _ in 0..2000 {
            secret_number = ((secret_number * 64) ^ secret_number) % 16777216;
            secret_number = ((secret_number / 32) ^ secret_number) % 16777216;
            secret_number = ((secret_number * 2048) ^ secret_number) % 16777216;
            digits.push(secret_number % 10);
        }

        let changes = digits
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();

        let mut seen: BTreeSet<(i64, i64, i64, i64)> = BTreeSet::new();
        for i in 4..digits.len() {
            let key = (
                changes[i - 4],
                changes[i - 3],
                changes[i - 2],
                changes[i - 1],
            );
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);
            *profit.entry(key).or_default() += digits[i];
        }
    }
    profit.values().max().unwrap().clone() as usize
}

#[test]
fn test_part1() {
    let input = "1
10
100
2024";
    assert_eq!(part1(input), 37327623);
}

#[test]
fn test_part2() {
    let input = "1
2
3
2024";
    assert_eq!(part2(input), 23);
}
