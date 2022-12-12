use std::collections::HashMap;

#[allow(clippy::identity_op)]
#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut map = HashMap::new();

    map.insert("A X", 3 + 1);
    map.insert("A Y", 6 + 2);
    map.insert("A Z", 0 + 3);
    map.insert("B X", 0 + 1);
    map.insert("B Y", 3 + 2);
    map.insert("B Z", 6 + 3);
    map.insert("C X", 6 + 1);
    map.insert("C Y", 0 + 2);
    map.insert("C Z", 3 + 3);

    input.lines().map(|l| map.get(l).unwrap()).sum()
}

#[allow(clippy::identity_op)]
#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut map = HashMap::new();

    map.insert("A X", 0 + 3);
    map.insert("A Y", 3 + 1);
    map.insert("A Z", 6 + 2);
    map.insert("B X", 0 + 1);
    map.insert("B Y", 3 + 2);
    map.insert("B Z", 6 + 3);
    map.insert("C X", 0 + 2);
    map.insert("C Y", 3 + 3);
    map.insert("C Z", 6 + 1);

    input.lines().map(|l| map.get(l).unwrap()).sum()
}
