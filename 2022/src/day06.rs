#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    solution(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    solution(input, 14)
}

pub fn solution(input: &str, window_size: usize) -> usize {
    let input: Vec<char> = input.chars().collect();
    let mut i = window_size;
    while i < input.len() {
        let window: Vec<char> = (0..window_size).map(|j| input[i - j]).collect();

        if window
            .iter()
            .all(|c| window.iter().filter(|&x| x == c).count() == 1)
        {
            return i + 1;
        }
        i += 1;
    }
    unreachable!();
}

#[test]
fn test_part1() {
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
}

#[test]
fn test_part2() {
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
}
