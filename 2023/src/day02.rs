use std::{cmp::max, collections::HashMap};

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<HashMap<String, usize>>> {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|game| {
                    game.split(',')
                        .map(|hand| {
                            let split = hand.trim().split_once(' ').unwrap();
                            (split.1.to_string(), split.0.parse::<usize>().unwrap())
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &[Vec<HashMap<String, usize>>]) -> usize {
    games
        .iter()
        .enumerate()
        .filter_map(|g| {
            if is_game_valid(g.1) {
                Some(g.0 + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(games: &[Vec<HashMap<String, usize>>]) -> usize {
    games
        .iter()
        .map(|game| {
            let mut minimum_set: HashMap<String, usize> = HashMap::new();
            for hand in game {
                for (color, value) in hand {
                    let minimum = minimum_set.entry(color.to_string()).or_default();
                    *minimum = max(*minimum, *value);
                }
            }
            minimum_set.values().product::<usize>()
        })
        .sum()
}

pub fn is_game_valid(game: &Vec<HashMap<String, usize>>) -> bool {
    let max_values = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    for hand in game {
        for (color, max_value) in max_values.iter() {
            if hand.contains_key(*color) && hand[*color] > *max_value {
                return false;
            }
        }
    }
    true
}

#[test]
fn part1_test() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(part1(&generator(input)), 8);
}

#[test]
fn part2_test() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(part2(&generator(input)), 2286);
}
