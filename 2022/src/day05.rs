#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let callback: fn(usize, usize, usize, &mut [Vec<char>; 9]) = |n, from, to, stacks| {
        for _ in 0..n {
            let Some(v) = stacks[from].pop() else { break; };
            stacks[to].push(v);
        }
    };

    solution(input, callback)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let callback: fn(usize, usize, usize, &mut [Vec<char>; 9]) = |n, from, to, stacks| {
        let mut tmp_vec = vec![];
        for _ in 0..n {
            let Some(v) = stacks[from].pop() else { break; };
            tmp_vec.push(v);
        }
        tmp_vec.reverse();
        for v in tmp_vec {
            stacks[to].push(v);
        }
    };

    solution(input, callback)
}

pub fn solution(input: &str, callback: fn(usize, usize, usize, &mut [Vec<char>; 9])) -> String {
    let (starting_config, moves) = input.split_once("\n\n").unwrap();
    let mut stacks: [Vec<char>; 9] = Default::default();

    let reference_line = starting_config.lines().last().unwrap();
    for line in starting_config.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == ' ' || c == '[' || c == ']' {
                continue;
            }
            if c.is_numeric() {
                break;
            }
            let index = reference_line.chars().nth(i).unwrap().to_digit(10).unwrap() - 1;
            stacks.get_mut(index as usize).unwrap().push(c);
        }
    }
    for s in &mut stacks {
        s.reverse();
    }

    for instruction in moves.lines() {
        let split = instruction.split_once(" from ").unwrap();
        let n = split
            .0
            .split_once("move ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let (from, to) = split.1.split_once(" to ").unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;

        callback(n, from, to, &mut stacks);
    }

    let out: String = stacks
        .iter()
        .map(|s| s.iter().last().unwrap_or(&' '))
        .collect();
    out
}
