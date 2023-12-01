#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut all_numbers = vec![];
            for c in l.chars() {
                if c.is_digit(10) {
                    all_numbers.push(c.to_digit(10).unwrap());
                }
            }
            all_numbers[0] * 10 + all_numbers[all_numbers.len() - 1]
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let mut all_numbers = vec![];
            for j in 0..line.len() {
                let d = line.chars().nth(j).unwrap();
                if d.is_digit(10) {
                    all_numbers.push(d.to_digit(10).unwrap() as usize);
                } else {
                    for (i, digit) in digits.iter().enumerate() {
                        if line[j..].starts_with(digit) {
                            all_numbers.push(i + 1);
                            break;
                        }
                    }
                }
            }
            all_numbers[0] * 10 + all_numbers[all_numbers.len() - 1]
        })
        .sum()
}

#[test]
fn part1_test() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    assert_eq!(part1(input), 142);
}

#[test]
fn part2_test() {
    let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    assert_eq!(part2(input), 281);
}

#[test]
fn part2_test_small_input() {
    let input = r#"g4"#;
    assert_eq!(part2(input), 44);
}
