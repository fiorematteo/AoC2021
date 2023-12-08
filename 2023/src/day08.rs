use std::collections::HashMap;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().cycle();

    let map: HashMap<&str, (&str, &str)> = map
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let left = &value[1..4];
            let right = &value[6..9];
            (key, (left, right))
        })
        .collect();

    let mut steps = 0;
    let mut current = "AAA";
    for inst in instructions {
        match inst {
            'L' => current = map[current].0,
            'R' => current = map[current].1,
            _ => unreachable!(),
        }
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }
    steps
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().cycle();

    let map: HashMap<&str, (&str, &str)> = map
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let left = &value[1..4];
            let right = &value[6..9];
            (key, (left, right))
        })
        .collect();

    let nodes: Vec<&str> = map.keys().filter(|k| k.ends_with("A")).cloned().collect();
    let mut times: Vec<usize> = Vec::new();
    for mut current in nodes.into_iter() {
        let mut steps = 0;
        for inst in instructions.clone() {
            match inst {
                'L' => current = map[current].0,
                'R' => current = map[current].1,
                _ => unreachable!(),
            }
            steps += 1;
            if current.ends_with('Z') {
                break;
            }
        }
        times.push(steps);
    }

    times.into_iter().reduce(num::integer::lcm).unwrap()
}

#[test]
fn test_part1() {
    let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
    assert_eq!(part1(input), 2);

    let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    assert_eq!(part1(input), 6);
}

#[test]
fn test_part2() {
    let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    assert_eq!(part2(input), 6);
}
