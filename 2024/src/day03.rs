use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    re.captures_iter(input)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * m.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let mut dos: Vec<usize> = do_regex
        .captures_iter(input)
        .map(|m| m.get(0).unwrap().start())
        .collect();
    dos.insert(0, 0);
    let donts: Vec<usize> = dont_regex
        .captures_iter(input)
        .map(|m| m.get(0).unwrap().start())
        .collect();

    mul_regex
        .captures_iter(input)
        .map(|m| {
            let current_position = m.get(0).unwrap().start();
            let mut last_do = 0;
            for do_position in &dos {
                if *do_position > current_position {
                    break;
                }
                last_do = *do_position;
            }
            let mut last_dont = 0;
            for dont_position in &donts {
                if *dont_position > current_position {
                    break;
                }
                last_dont = *dont_position;
            }
            if last_dont > last_do {
                return 0;
            }
            m.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * m.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}

#[test]
pub fn part1_test() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(input), 161);
}

#[test]
pub fn part2_test() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(input), 48);
}
