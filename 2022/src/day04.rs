#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let ranges = line.split_once(',').unwrap();

        let first = ranges.0.split_once('-').unwrap();
        let second = ranges.1.split_once('-').unwrap();

        let start_first = first.0.parse::<usize>().unwrap();
        let end_first = first.1.parse::<usize>().unwrap();
        let start_second = second.0.parse::<usize>().unwrap();
        let end_second = second.1.parse::<usize>().unwrap();

        if (start_second <= start_first && end_first <= end_second)
            || (start_first <= start_second && end_second <= end_first)
        {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let ranges = line.split_once(',').unwrap();

        let str_to_range = |input: &str| {
            let split = input.split_once('-').unwrap();
            split.0.parse::<usize>().unwrap()..=split.1.parse::<usize>().unwrap()
        };

        let mut first = str_to_range(ranges.0);
        let second = str_to_range(ranges.1);

        if first.any(|x| second.contains(&x)) {
            count += 1;
        }
    }
    count
}
