use std::collections::HashMap;

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let (towels, targets) = input.split_once("\n\n").unwrap();
    let towels: Vec<_> = towels.split(", ").collect();
    let targets: Vec<_> = targets.lines().collect();
    let mut cache = HashMap::new();

    targets
        .iter()
        .filter(|target| composable(&towels, target, &mut cache) > 0)
        .count()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let (towels, targets) = input.split_once("\n\n").unwrap();
    let towels: Vec<_> = towels.split(", ").collect();
    let targets: Vec<_> = targets.lines().collect();
    let mut cache = HashMap::new();

    targets
        .into_iter()
        .map(|target| composable(&towels, target, &mut cache))
        .sum()
}

fn composable<'a>(towels: &[&str], target: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    if target.is_empty() {
        return 1;
    }
    if let Some(&v) = cache.get(target) {
        return v;
    }
    let mut out = 0;
    for towel in towels {
        if let Some(next) = target.strip_prefix(towel) {
            let x = composable(towels, next, cache);
            cache.insert(next, x);
            out += x;
        }
    }
    out
}

#[test]
fn test_part1() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part1(input), 6);
}

#[test]
fn test_part2() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    assert_eq!(part2(input), 16);
}
