use std::{collections::HashMap, ops::RangeInclusive};

#[aoc_generator(day19)]
fn generator(input: &str) -> (Workflows, Parts) {
    let (workflow, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflow
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let rules: Vec<_> = rest[..rest.len() - 1]
                .split(',')
                .map(|r| {
                    let Some((condition, outcome)) = r.split_once(':') else {
                        let outcome = match r {
                            "A" => Outcome::Accept,
                            "R" => Outcome::Reject,
                            o => Outcome::Next(o.to_string()),
                        };
                        return (Condition::True, outcome);
                    };
                    let num: usize = condition[2..].parse().unwrap();
                    let condition = match &condition[1..2] {
                        "<" => Condition::Smaller(condition[..1].to_string(), num),
                        ">" => Condition::Greater(condition[..1].to_string(), num),
                        _ => unreachable!(),
                    };
                    let outcome = match outcome {
                        "A" => Outcome::Accept,
                        "R" => Outcome::Reject,
                        o => Outcome::Next(o.to_string()),
                    };
                    (condition, outcome)
                })
                .collect();
            (name.to_string(), rules)
        })
        .collect::<HashMap<_, _>>();
    let parts: Vec<HashMap<_, _>> = parts
        .lines()
        .map(|l| {
            l[1..l.len() - 1]
                .split(',')
                .map(|x| {
                    let (k, v) = x.split_once('=').unwrap();
                    (k.to_string(), v.parse::<usize>().unwrap())
                })
                .collect()
        })
        .collect();
    (workflows, parts)
}

#[aoc(day19, part1)]
fn part1((workflows, parts): &(Workflows, Parts)) -> usize {
    parts
        .iter()
        .filter(|part| {
            let mut current_workflow = &workflows["in"];
            loop {
                for (condition, outcome) in current_workflow {
                    let result = match condition {
                        Condition::Greater(target, v) => part[target.as_str()] > *v,
                        Condition::Smaller(target, v) => part[target.as_str()] < *v,
                        Condition::True => true,
                    };
                    if result {
                        match outcome {
                            Outcome::Reject => return false,
                            Outcome::Accept => return true,
                            Outcome::Next(n) => {
                                current_workflow = &workflows[n.as_str()];
                                break;
                            }
                        }
                    }
                }
            }
        })
        .map(|part| part.values().sum::<usize>())
        .sum()
}

#[aoc(day19, part2)]
fn part2((workflows, _): &(Workflows, Parts)) -> usize {
    solve(workflows, "in", 0, new_state())
}

fn solve(
    workflows: &Workflows,
    current_workflow: &str,
    current_rule: usize,
    state: State,
) -> usize {
    let w = &workflows[current_workflow];
    assert!(current_rule < w.len());
    assert!(state.values().all(|r| !r.is_empty()));

    let (condition, outcome) = &w[current_rule];
    let states = match condition {
        Condition::Greater(target, v) => {
            let mut a = state.clone();
            let mut b = state.clone();
            let start = *state[target].start();
            let end = *state[target].end();
            a.insert(target.to_string(), *v + 1..=end);
            b.insert(target.to_string(), start..=*v);
            (a, Some(b))
        }
        Condition::Smaller(target, v) => {
            let mut a = state.clone();
            let mut b = state.clone();
            let start = *state[target].start();
            let end = *state[target].end();
            a.insert(target.to_string(), start..=*v - 1);
            b.insert(target.to_string(), *v..=end);
            (a, Some(b))
        }
        Condition::True => (state, None),
    };
    match outcome {
        Outcome::Reject => {
            if let Some(s) = states.1 {
                solve(workflows, current_workflow, current_rule + 1, s)
            } else {
                0
            }
        }
        Outcome::Accept => {
            let mut out = count_possible_states(&states.0);
            if let Some(s) = states.1 {
                out += solve(workflows, current_workflow, current_rule + 1, s);
            }
            out
        }
        Outcome::Next(n) => {
            let mut out = solve(workflows, n, 0, states.0);
            if let Some(s) = states.1 {
                out += solve(workflows, current_workflow, current_rule + 1, s);
            }
            out
        }
    }
}

fn count_possible_states(state: &State) -> usize {
    state.iter().map(|(_, v)| v.clone().count()).product()
}

type State = HashMap<String, RangeInclusive<usize>>;
type Workflows = HashMap<String, Vec<(Condition, Outcome)>>;
type Parts = Vec<HashMap<String, usize>>;

fn new_state() -> State {
    let mut map = HashMap::new();
    map.insert("x".to_string(), 1..=4000);
    map.insert("s".to_string(), 1..=4000);
    map.insert("m".to_string(), 1..=4000);
    map.insert("a".to_string(), 1..=4000);
    map
}

#[derive(Debug, Clone)]
enum Condition {
    Greater(String, usize),
    Smaller(String, usize),
    True,
}

#[derive(Debug, Clone)]
enum Outcome {
    Reject,
    Accept,
    Next(String),
}

#[test]
fn test_part1() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    assert_eq!(part1(&generator(input)), 19114);
}

#[test]
fn test_part2() {
    let input = r#"in{x>2000:b,R}
b{m>2000:c,R}
c{a>2000:d,R}
d{s>2000:A,R}

"#;
    let p2 = part2(&generator(input));
    assert_eq!(p2, 16000000000000);

    let input = r#"in{x<2000:b,R}
b{m<2000:c,R}
c{a<2000:d,R}
d{s<2000:A,R}

"#;
    let p2 = part2(&generator(input));
    assert_eq!(p2, 15968023992001);

    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    let p2 = part2(&generator(input));
    assert_eq!(p2, 167409079868000);
}
