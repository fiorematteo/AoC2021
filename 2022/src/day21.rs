use std::collections::BTreeMap;

#[aoc_generator(day21)]
pub fn generator(input: &str) -> BTreeMap<String, Monkey> {
    let mut monkeys = BTreeMap::new();
    for line in input.lines() {
        let (name, right) = line.split_once(": ").unwrap();
        let m = if right.chars().next().unwrap().is_ascii_digit() {
            Monkey::Number(right.parse().unwrap())
        } else {
            let mut iter = right.split_whitespace();
            let l = iter.next().unwrap().to_string();
            let op = iter.next().unwrap();
            let r = iter.next().unwrap().to_string();
            match op {
                "+" => Monkey::Plus(l, r),
                "-" => Monkey::Minus(l, r),
                "*" => Monkey::Multiply(l, r),
                "/" => Monkey::Divide(l, r),
                _ => unreachable!(),
            }
        };
        monkeys.insert(name.to_string(), m);
    }
    monkeys
}

#[aoc(day21, part1)]
pub fn part1(monkeys: &BTreeMap<String, Monkey>) -> f64 {
    fn solve(map: &BTreeMap<String, Monkey>, start: &str) -> f64 {
        let m = map.get(start).unwrap();
        match m {
            Monkey::Number(n) => *n,
            Monkey::Plus(l, r) => solve(map, l) + solve(map, r),
            Monkey::Minus(l, r) => solve(map, l) - solve(map, r),
            Monkey::Multiply(l, r) => solve(map, l) * solve(map, r),
            Monkey::Divide(l, r) => solve(map, l) / solve(map, r),
        }
    }
    solve(monkeys, "root")
}

#[aoc(day21, part2)]
pub fn part2(monkeys: &BTreeMap<String, Monkey>) -> f64 {
    let (l, _, r) = monkeys.get("root").unwrap().get_triple().unwrap();
    match (solve_or_human(monkeys, l), solve_or_human(monkeys, r)) {
        (Some(l_value), None) => actual_solve(monkeys, monkeys.get(r).unwrap().clone(), l_value),
        (None, Some(r_value)) => actual_solve(monkeys, monkeys.get(l).unwrap().clone(), r_value),
        _ => unreachable!(),
    }
}

pub fn actual_solve(map: &BTreeMap<String, Monkey>, current: Monkey, target: f64) -> f64 {
    let Some((l, op, r)) =  current.get_triple() else { return target };

    let l_value = solve_or_human(map, l);
    let r_value = solve_or_human(map, r);

    match (l_value, r_value) {
        (None, Some(r)) => {
            let new_target = match op {
                "+" => target - r,
                "-" => target + r,
                "*" => target / r,
                "/" => target * r,
                _ => unreachable!(),
            };
            actual_solve(map, map.get(l).unwrap().clone(), new_target)
        }
        (Some(l), None) => {
            let new_target = match op {
                "+" => target - l,
                "-" => -target + l,
                "*" => target / l,
                _ => unreachable!(),
            };
            actual_solve(map, map.get(r).unwrap().clone(), new_target)
        }
        _ => unreachable!(),
    }
}

pub fn solve_or_human(map: &BTreeMap<String, Monkey>, start: &str) -> Option<f64> {
    if start == "humn" {
        return None;
    }
    let m = map.get(start).unwrap();
    Some(match m {
        Monkey::Number(n) => *n,
        Monkey::Plus(l, r) => solve_or_human(map, l)? + solve_or_human(map, r)?,
        Monkey::Minus(l, r) => solve_or_human(map, l)? - solve_or_human(map, r)?,
        Monkey::Multiply(l, r) => solve_or_human(map, l)? * solve_or_human(map, r)?,
        Monkey::Divide(l, r) => solve_or_human(map, l)? / solve_or_human(map, r)?,
    })
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Monkey {
    Number(f64),
    Plus(String, String),
    Minus(String, String),
    Multiply(String, String),
    Divide(String, String),
}

impl Monkey {
    fn get_triple(&self) -> Option<(&str, &str, &str)> {
        Some(match self {
            Monkey::Plus(l, r) => (l, "+", r),
            Monkey::Minus(l, r) => (l, "-", r),
            Monkey::Multiply(l, r) => (l, "*", r),
            Monkey::Divide(l, r) => (l, "/", r),
            Monkey::Number(_) => return None,
        })
    }
}
