use std::{collections::BTreeMap, fmt::Display};

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Vec<Blueprint> {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    input
        .lines()
        .map(|line| {
            let mut iter = line.split('.');
            let ore_robot = iter.next().unwrap();
            let clay_robot = iter.next().unwrap();
            let obsidian_robot = iter.next().unwrap();
            let geode_robot = iter.next().unwrap();

            let ore_robot = ore_robot
                .split_whitespace()
                .nth(6)
                .unwrap()
                .parse()
                .unwrap();
            let clay_robot = clay_robot
                .split_whitespace()
                .nth(4)
                .unwrap()
                .parse()
                .unwrap();

            let mut obsidian_iter = obsidian_robot.split_whitespace();
            let obsidian_robot = (
                obsidian_iter.nth(4).unwrap().parse().unwrap(),
                obsidian_iter.nth(2).unwrap().parse().unwrap(),
            );

            let mut geode_iter = geode_robot.split_whitespace();
            let geode_robot = (
                geode_iter.nth(4).unwrap().parse().unwrap(),
                geode_iter.nth(2).unwrap().parse().unwrap(),
            );

            Blueprint {
                ore_robot,
                clay_robot,
                obsidian_robot,
                geode_robot,
            }
        })
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> usize {
    const MAX_TIME: i32 = 24;

    let mut score = 0;
    for (id, blue) in input.iter().enumerate() {
        let mut opt = BTreeMap::new();
        let geode = best(
            blue,
            &mut opt,
            State {
                ore: 0,
                clay: 0,
                obsidian: 0,
                ore_robot: 1,
                clay_robot: 0,
                obsidian_robot: 0,
                time: MAX_TIME,
            },
        );
        score += geode as usize * (id + 1);
    }
    score
}

fn best(blue: &Blueprint, opt: &mut BTreeMap<State, i32>, state: State) -> i32 {
    if state.time <= 0 {
        return 0;
    }

    let max_ore = blue
        .ore_robot
        .max(blue.clay_robot)
        .max(blue.obsidian_robot.0)
        .max(blue.geode_robot.0);
    let max_clay = blue
        .clay_robot
        .max(blue.obsidian_robot.1)
        .max(blue.geode_robot.1);

    if let Some(&score) = opt.get(&state) {
        return score;
    }

    let State {
        ore,
        clay,
        obsidian,
        ore_robot,
        clay_robot,
        obsidian_robot,
        time,
    } = state;

    let mut geode_yield = 0;

    // geode_robot
    if obsidian_robot > 0 {
        let time_until_build = ((blue.geode_robot.0 - ore) as f32 / ore_robot as f32)
            .max((blue.geode_robot.1 - obsidian) as f32 / obsidian_robot as f32)
            .max(0.0);
        let time_until_build = time_until_build.ceil() as i32;
        if time_until_build <= time {
            geode_yield = geode_yield.max(
                time - time_until_build - 1
                    + best(
                        blue,
                        opt,
                        State {
                            ore: ore + time_until_build * ore_robot - blue.geode_robot.0,
                            clay: clay + time_until_build * clay_robot,
                            obsidian: obsidian + time_until_build * obsidian_robot
                                - blue.geode_robot.1,
                            time: time - time_until_build - 1,
                            ..state
                        },
                    ),
            )
        }
    }

    // obsidian_robot
    if clay_robot > 0 {
        let time_until_build = ((blue.obsidian_robot.0 - ore) as f32 / ore_robot as f32)
            .max((blue.obsidian_robot.1 - clay) as f32 / clay_robot as f32)
            .max(0.0);
        let time_until_build = time_until_build.ceil() as i32;
        if time_until_build <= time {
            geode_yield = geode_yield.max(best(
                blue,
                opt,
                State {
                    ore: ore + time_until_build * ore_robot - blue.obsidian_robot.0,
                    clay: clay + time_until_build * clay_robot - blue.obsidian_robot.1,
                    obsidian: obsidian + time_until_build * obsidian_robot,
                    obsidian_robot: obsidian_robot + 1,
                    time: time - time_until_build - 1,
                    ..state
                },
            ))
        }
    }

    // clay_robot
    if clay_robot <= max_clay {
        let time_until_build = ((blue.clay_robot - ore) as f32 / ore_robot as f32).max(0.0);
        let time_until_build = time_until_build.ceil() as i32;
        if time_until_build <= time {
            geode_yield = geode_yield.max(best(
                blue,
                opt,
                State {
                    ore: ore + time_until_build * ore_robot - blue.clay_robot,
                    clay: clay + time_until_build * clay_robot,
                    obsidian: obsidian + time_until_build * obsidian_robot,
                    clay_robot: clay_robot + 1,
                    time: time - time_until_build - 1,
                    ..state
                },
            ))
        }
    }

    // ore_robot
    if ore_robot <= max_ore {
        let time_until_build = ((blue.ore_robot - ore) as f32 / ore_robot as f32).max(0.0);
        let time_until_build = time_until_build.ceil() as i32;
        if time_until_build <= time {
            geode_yield = geode_yield.max(best(
                blue,
                opt,
                State {
                    ore: ore + time_until_build * ore_robot - blue.ore_robot,
                    clay: clay + time_until_build * clay_robot,
                    obsidian: obsidian + time_until_build * obsidian_robot,
                    ore_robot: ore_robot + 1,
                    time: time - time_until_build - 1,
                    ..state
                },
            ))
        }
    }

    opt.insert(state, geode_yield);
    geode_yield
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    time: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blueprint {
    ore_robot: i32,             // ore
    clay_robot: i32,            // ore
    obsidian_robot: (i32, i32), // ore, clay
    geode_robot: (i32, i32),    // ore, obsidian
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}",
            self.ore_robot,
            self.clay_robot,
            self.obsidian_robot.0,
            self.obsidian_robot.1,
            self.geode_robot.0,
            self.geode_robot.1
        )
    }
}
