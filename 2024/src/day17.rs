use std::{cmp::Reverse, collections::BinaryHeap};

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let mut registers = registers.lines();
    let mut get_register = || {
        registers
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse::<u64>()
            .unwrap()
    };
    let a = get_register();
    let instructions: Vec<_> = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    solve(a, &instructions)
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> String {
    let (_, instructions) = input.split_once("\n\n").unwrap();
    let instructions: Vec<_> = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let queue = vec![Reverse((instructions.len(), 0))];
    let mut queue = BinaryHeap::from(queue);
    loop {
        let Reverse((index, a)) = queue.pop().unwrap();
        if index == 0 {
            return a.to_string();
        }

        let next_index = index - 1;
        for i in 0..=0b111 {
            let new_a = i | a << 3;
            let out = solve(new_a, &instructions);
            if instructions[next_index] == out[0] {
                queue.push(Reverse((next_index, new_a)));
            }
        }
    }
}

pub fn solve(mut a: u64, instructions: &[u64]) -> Vec<u64> {
    let mut b = 0;
    let mut c = 0;
    let combo = |arg, a, b, c| match arg {
        0..=3 => arg,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    };

    let mut output = Vec::new();
    let mut i = 0;
    while i < instructions.len() - 1 {
        let op = instructions[i];
        let arg = instructions[i + 1];
        match op {
            0 => {
                //adv
                a >>= combo(arg, a, b, c);
            }
            1 => {
                //blx
                b = b ^ arg;
            }
            2 => {
                // bst
                b = combo(arg, a, b, c) & 0b111;
            }
            3 => {
                // jnz
                if a != 0 {
                    i = arg as usize;
                    continue;
                }
            }
            4 => {
                // bxc
                b = b ^ c;
            }
            5 => {
                // out
                output.push(combo(arg, a, b, c) & 0b111);
            }
            6 => {
                // bvd
                b = a >> combo(arg, a, b, c);
            }
            7 => {
                // cvd
                c = a >> combo(arg, a, b, c);
            }
            _ => unreachable!(),
        }
        i += 2;
    }
    output
}

#[test]
fn test_part1() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn test_part2() {
    let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    assert_eq!(part2(input), "117440");
}
