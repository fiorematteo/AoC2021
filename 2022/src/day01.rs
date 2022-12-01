use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    let elves = input.split("\n\n").collect::<Vec<_>>();
    elves
        .iter()
        .map(|e| e.lines().map(|a| a.parse::<i64>().unwrap()).sum::<i64>())
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i64>) -> i64 {
    let max_elf = input.iter().max().unwrap();
    *max_elf
}

#[aoc(day1, part2, ugly)]
pub fn part2_ugly(input: &Vec<i64>) -> i64 {
    let mut max = (0, 0, 0);
    for elf in input {
        if elf > &max.0 {
            max = (*elf, max.0, max.1);
        } else if elf > &max.1 {
            max = (max.0, *elf, max.1);
        } else if elf > &max.2 {
            max = (max.0, max.1, *elf);
        }
    }
    max.0 + max.1 + max.2
}

#[aoc(day1, part2, heap)]
pub fn part2_heap(input: &Vec<i64>) -> i64 {
    let mut heap = BinaryHeap::new();
    for elf in input {
        heap.push(*elf);
    }
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

#[aoc(day1, part2, sort)]
pub fn part2_sort(input: &Vec<i64>) -> i64 {
    let mut elves = input.clone();
    elves.sort();
    elves[elves.len() - 3..].iter().sum()
}
