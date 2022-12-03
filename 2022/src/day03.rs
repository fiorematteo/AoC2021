#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let compartments = line.split_at(line.len() / 2);
        let mut left = [false; 52];
        for c in compartments.0.chars() {
            left[to_priority(c) - 1] = true;
        }
        for c in compartments.1.chars() {
            let index = to_priority(c);
            if left[index - 1] {
                left[index - 1] = false;
                sum += index;
            }
        }
    }
    sum
}

pub fn to_priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 38
    } else {
        c as usize - 96
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    for group in input.lines().collect::<Vec<&str>>().chunks(3) {
        let mut contains = [[false; 3]; 52];
        for (i, elf) in group.iter().enumerate() {
            for c in elf.chars() {
                let index = to_priority(c);
                contains[index - 1][i] = true;
            }
        }
        for (i, c) in contains.iter().enumerate() {
            if c.iter().all(|x| *x) {
                sum += i + 1;
                break;
            }
        }
    }
    sum
}
