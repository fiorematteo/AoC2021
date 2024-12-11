#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = rules
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

#[aoc(day5, part1)]
pub fn part1((rules, updates): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let mut total = 0;
    for update in updates {
        if rules.iter().all(|rule| is_correct(rule, update)) {
            let middle_index = update.len() / 2;
            total += update[middle_index];
        }
    }
    total
}

#[aoc(day5, part2)]
pub fn part2((rules, updates): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let mut total = 0;
    for update in updates
        .iter()
        .filter(|update| !rules.iter().all(|rule| is_correct(rule, update)))
    {
        let mut update = update.clone();
        let mut done = false;
        while !done {
            done = true;
            for rule in rules {
                if !is_correct(rule, &update) {
                    let first_index = update.iter().position(|&x| x == rule.0).unwrap();
                    let second_index = update.iter().position(|&x| x == rule.1).unwrap();
                    update.swap(first_index, second_index);
                    done = false;
                    break;
                }
            }
        }
        let middle_index = update.len() / 2;
        total += update[middle_index];
    }
    total
}

fn is_correct(rule: &(u32, u32), update: &[u32]) -> bool {
    if !update.contains(&rule.0) || !update.contains(&rule.1) {
        return true;
    }
    for &x in update.iter() {
        if x == rule.0 {
            return true;
        }
        if x == rule.1 {
            return false;
        }
    }
    false
}

#[test]
fn part1_test() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part1(&generator(input)), 143);
}

#[test]
fn part2_test() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part2(&generator(input)), 123);
}
