#[aoc(day9, part1)]
fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mut nums: Vec<i32> = line
            .split(' ')
            .map(&str::parse)
            .map(Result::unwrap)
            .collect();

        let mut last_nums = vec![];
        while !nums.iter().all(|&n| n == 0) {
            for i in 1..nums.len() {
                nums[i - 1] = nums[i] - nums[i - 1];
            }
            last_nums.push(nums.pop().unwrap());
        }
        total += last_nums.into_iter().sum::<i32>();
    }
    total
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mut nums: Vec<i32> = line
            .split(' ')
            .map(&str::parse)
            .map(Result::unwrap)
            .collect();

        let mut last_nums = vec![];
        while !nums.iter().all(|&n| n == 0) {
            last_nums.push(nums[0]);
            for i in 1..nums.len() {
                nums[i - 1] = nums[i] - nums[i - 1];
            }
            nums.pop().unwrap();
        }
        total += last_nums.into_iter().rev().reduce(|a, b| b - a).unwrap();
    }
    total
}

#[test]
fn test_part1() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    assert_eq!(part1(input), 114);
}

#[test]
fn test_part2() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    assert_eq!(part2(input), 2);
}
