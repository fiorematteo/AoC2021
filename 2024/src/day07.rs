#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    solve(input, false)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u64 {
    solve(input, true)
}

fn solve(input: &str, third_op: bool) -> u64 {
    input
        .lines()
        .map(|l| {
            let (result, nums) = l.split_once(": ").unwrap();
            let target = result.parse().unwrap();
            let nums: Vec<_> = nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            if is_possible(target, &nums, third_op) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn is_possible(target: u64, nums: &[u64], third_op: bool) -> bool {
    let mut queue = vec![(1, nums[0])];
    while let Some((start_index, partial)) = queue.pop() {
        if partial > target {
            continue;
        }
        if start_index == nums.len() {
            if partial == target {
                return true;
            }
            continue;
        }
        // plus
        queue.push((start_index + 1, partial + nums[start_index]));
        // mul
        queue.push((start_index + 1, partial * nums[start_index]));
        // concatenate
        if third_op {
            let digits = nums[start_index].ilog10() + 1;
            let next_num = (partial * 10_u64.pow(digits)) + nums[start_index];
            queue.push((start_index + 1, next_num));
        }
    }
    false
}

#[test]
fn part1_test() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(part1(input), 3749);
}

#[test]
fn part2_test() {
    let input = "2064067023: 40 189 34 9 6 90 9 3 3 3";
    assert_eq!(part2(input), 0);

    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(part2(input), 11387);
}
