#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let difference: Vec<i32> = line.windows(2).map(|l| l[0] - l[1]).collect();

            let all_positive = difference.iter().all(|&n| n > 0);
            let all_negative = difference.iter().all(|&n| n < 0);
            let max_change = difference.iter().map(|n| n.abs()).all(|n| n <= 3);
            if (all_positive || all_negative) && max_change {
                return 1;
            }
            0
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            for i in 0..line.len() {
                let new_line: Vec<i32> = line[..i].iter().chain(&line[i + 1..]).cloned().collect();
                let difference: Vec<i32> = new_line.windows(2).map(|l| l[0] - l[1]).collect();

                let all_positive = difference.iter().all(|&n| n > 0);
                let all_negative = difference.iter().all(|&n| n < 0);
                let max_change = difference.iter().map(|n| n.abs()).all(|n| n <= 3);
                if (all_positive || all_negative) && max_change {
                    return 1;
                }
            }
            0
        })
        .sum()
}

#[test]
fn part1_test() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part1(input), 2);
}

#[test]
fn part2_test() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part2(input), 4);

    let input = "72 75 78 79 79";
    assert_eq!(part2(input), 1);
}
