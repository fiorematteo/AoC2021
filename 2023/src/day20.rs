use num::integer::lcm;
use std::collections::{BTreeMap, VecDeque};

type Modules = BTreeMap<String, (Module, Vec<String>)>;

#[aoc_generator(day20)]
fn generator(input: &str) -> Modules {
    let mut modules: Modules = input
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(" -> ").unwrap();
            let v: Vec<_> = v.split(", ").map(|s| s.to_string()).collect();
            let (name, module) = match &k[..1] {
                "b" => (k.to_string(), Module::Broadcast),
                "%" => (k[1..].to_string(), Module::Flip(false)),
                "&" => (k[1..].to_string(), Module::Conjunction(BTreeMap::new())),
                _ => unreachable!(),
            };
            (name, (module, v))
        })
        .collect();

    for (k, v) in modules.clone().into_iter() {
        for name in v.1 {
            let Some((m, _)) = modules.get_mut(name.as_str()) else {
                continue;
            };
            if let Module::Conjunction(map) = m {
                map.insert(k.clone(), Pulse::Low);
            }
        }
    }
    modules
}

#[aoc(day20, part1)]
fn part1(modules: &Modules) -> usize {
    let mut modules = modules.clone();
    let (mut high, mut low) = (0, 0);
    for _ in 0..1000 {
        let mut queue: VecDeque<_> =
            vec![("broadcaster".to_string(), Pulse::Low, "button".to_string())].into();
        while let Some((name, signal, from)) = queue.pop_back() {
            let (current_high, current_low) = signal.value();
            high += current_high;
            low += current_low;
            let Some((module, next)) = modules.get_mut(&name) else {
                continue;
            };
            match module {
                Module::Flip(state) => {
                    if matches!(signal, Pulse::Low) {
                        let next_signal = if *state { Pulse::Low } else { Pulse::High };
                        *state = !*state;
                        for n in next {
                            queue.push_front((n.to_string(), next_signal, name.to_string()));
                        }
                    }
                }
                Module::Conjunction(ref mut map) => {
                    map.insert(from, signal);
                    let next_signal = if map.values().all(|p| matches!(p, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for n in next {
                        queue.push_front((n.to_string(), next_signal, name.to_string()));
                    }
                }
                Module::Broadcast => {
                    for n in next {
                        queue.push_front((n.to_string(), signal, name.to_string()));
                    }
                }
            }
        }
    }
    high * low
}

#[aoc(day20, part2)]
fn part2(modules: &Modules) -> usize {
    let mut modules = modules.clone();
    let mut nodes_to_watch: Vec<_> = vec!["rx".to_string()];
    while nodes_to_watch.len() == 1 {
        let target = nodes_to_watch.pop().unwrap();
        nodes_to_watch = modules
            .iter()
            .filter(|(_, (_, v))| v.contains(&target))
            .map(|(name, _)| name.to_string())
            .collect();
    }
    let mut lcm_map = BTreeMap::new();

    let (mut high, mut low) = (0, 0);
    for i in 1.. {
        let mut queue: VecDeque<_> =
            vec![("broadcaster".to_string(), Pulse::Low, "button".to_string())].into();
        while let Some((name, signal, from)) = queue.pop_back() {
            if nodes_to_watch.contains(&name) && signal == Pulse::Low {
                if !lcm_map.contains_key(&name) {
                    lcm_map.insert(name.clone(), i);
                }
                if lcm_map.len() == nodes_to_watch.len() {
                    let mut out: usize = 1;
                    for n in lcm_map.values() {
                        out = lcm(out, *n)
                    }
                    return out;
                }
            }

            let (current_high, current_low) = signal.value();
            high += current_high;
            low += current_low;
            let Some((module, next)) = modules.get_mut(&name) else {
                continue;
            };
            match module {
                Module::Flip(state) => {
                    if matches!(signal, Pulse::Low) {
                        let next_signal = if *state { Pulse::Low } else { Pulse::High };
                        *state = !*state;
                        for n in next {
                            queue.push_front((n.to_string(), next_signal, name.to_string()));
                        }
                    }
                }
                Module::Conjunction(ref mut map) => {
                    map.insert(from, signal);
                    let next_signal = if map.values().all(|p| matches!(p, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for n in next {
                        queue.push_front((n.to_string(), next_signal, name.to_string()));
                    }
                }
                Module::Broadcast => {
                    for n in next {
                        queue.push_front((n.to_string(), signal, name.to_string()));
                    }
                }
            }
        }
    }
    high * low
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn value(&self) -> (usize, usize) {
        match self {
            Pulse::High => (1, 0),
            Pulse::Low => (0, 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Module {
    Flip(bool),
    Conjunction(BTreeMap<String, Pulse>),
    Broadcast,
}

#[test]
fn test_part1() {
    let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    assert_eq!(part1(&generator(input)), 32000000);
}
