#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i64>>()
}

#[aoc(day20, part1)]
pub fn part1(numbers: &[i64]) -> i64 {
    solve(numbers, 1)
}

#[aoc(day20, part2)]
pub fn part2(numbers: &[i64]) -> i64 {
    let numbers = numbers
        .iter()
        .map(|n| n * 811589153i64)
        .collect::<Vec<i64>>();

    solve(&numbers, 10)
}

pub fn solve(numbers: &[i64], iterations: usize) -> i64 {
    let mut mixed = (0..numbers.len()).collect::<Vec<_>>();

    for _ in 0..iterations {
        for (i, &n) in numbers.iter().enumerate() {
            let index = mixed.iter().position(|&x| x == i).unwrap();
            mixed.remove(index);
            let new_i = (index as i64 + n).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_i, i);
        }
    }

    let index = mixed
        .iter()
        .position(|&i| i == numbers.iter().position(|&i| i == 0).unwrap())
        .unwrap();
    let a = numbers[mixed[(index + 1000) % mixed.len()]];
    let b = numbers[mixed[(index + 2000) % mixed.len()]];
    let c = numbers[mixed[(index + 3000) % mixed.len()]];
    a.checked_add(b).unwrap().checked_add(c).unwrap()
}
