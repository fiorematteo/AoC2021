#[aoc_generator(day16)]
pub fn generator(input: &str) -> (Vec<String>, Vec<u64>, Vec<Vec<String>>) {
    let mut values = Vec::new();
    for line in input.lines() {
        let line = line
            .replace("Valve", "")
            .replace(" has flow rate=", ",")
            .replace("; tunnels lead to valves ", ",")
            .replace("; tunnel leads to valve ", ",")
            .replace(' ', "");
        let mut iter = line.split(',');

        let name = iter.next().unwrap().to_string();
        let flow_rate = iter.next().unwrap().parse::<u64>().unwrap();
        let connections = iter.map(|s| s.into()).collect::<Vec<String>>();
        values.push((name, flow_rate, connections));
    }
    values.sort_by_key(|v| -(v.1 as i64));

    let mut valves = Vec::new();
    let mut connections: Vec<Vec<String>> = Vec::new();
    let mut flow_rate = Vec::new();

    values.iter().for_each(|v| {
        valves.push(v.0.clone());
        connections.push(v.2.clone());
        flow_rate.push(v.1);
    });

    (valves, flow_rate, connections)
}

#[aoc(day16, part1)]
pub fn part1((valves, flow_rate, connections): &(Vec<String>, Vec<u64>, Vec<Vec<String>>)) -> u64 {
    let max_time = 30;

    // dynamic programming [time left, current node, available valves]
    let max_available_valves = valves
        .iter()
        .enumerate()
        .filter(|&(i, _)| flow_rate[i] > 0)
        .count();
    let mut sol: Vec<Vec<Vec<u64>>> =
        vec![vec![vec![0; 1 << max_available_valves]; valves.len()]; max_time];
    for time in 1..max_time {
        for node in 0..valves.len() {
            let ii = 1 << node;
            for available in 0..(1 << max_available_valves) {
                let mut current = sol[time][node][available];
                if ii & available != 0 {
                    current = current
                        .max(sol[time - 1][node][available - ii] + flow_rate[node] * time as u64);
                }
                for name in &connections[node] {
                    let index = valves.iter().position(|v| v == name).unwrap();
                    current = current.max(sol[time - 1][index][available]);
                }
                sol[time][node][available] = current;
            }
        }
    }

    let first_index = valves.iter().position(|v| v == "AA").unwrap();
    sol[max_time - 1][first_index][(1 << max_available_valves) - 1] as _
}

#[aoc(day16, part2)]
pub fn part2((valves, flow_rate, connections): &(Vec<String>, Vec<u64>, Vec<Vec<String>>)) -> u64 {
    let max_time = 26;

    // dynamic programming [time left, current node, available valves]
    let max_available_valves = valves
        .iter()
        .enumerate()
        .filter(|&(i, _)| flow_rate[i] > 0)
        .count();
    let mut sol: Vec<Vec<Vec<u64>>> =
        vec![vec![vec![0; 1 << max_available_valves]; valves.len()]; max_time];
    for time in 1..max_time {
        for node in 0..valves.len() {
            let ii = 1 << node;
            for available in 0..(1 << max_available_valves) {
                let mut current = sol[time][node][available];
                if ii & available != 0 {
                    current = current
                        .max(sol[time - 1][node][available - ii] + flow_rate[node] * time as u64);
                }
                for name in &connections[node] {
                    let index = valves.iter().position(|v| v == name).unwrap();
                    current = current.max(sol[time - 1][index][available]);
                }
                sol[time][node][available] = current;
            }
        }
    }

    let first_index = valves.iter().position(|v| v == "AA").unwrap();
    let mut best = 0;
    let mm = 1 << max_available_valves;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(sol[max_time - 1][first_index][x] + sol[max_time - 1][first_index][y]);
    }
    best
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct Valve {
    flow_rate: u64,
    connections: Vec<String>,
}
