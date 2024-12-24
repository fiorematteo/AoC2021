use std::collections::BTreeMap;

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let (initial, connections) = input.split_once("\n\n").unwrap();
    let initial = initial
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(k, v)| (k, v.parse::<usize>().unwrap() == 1))
        .collect::<BTreeMap<_, _>>();
    let connections = connections
        .lines()
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let left = tokens.next().unwrap();
            let op = tokens.next().unwrap();
            let right = tokens.next().unwrap();
            tokens.next();
            let result = tokens.next().unwrap();
            (result, (left, op, right))
        })
        .collect::<BTreeMap<_, _>>();

    let mut z = 0;
    for key in connections.keys() {
        if let Some(index) = key.strip_prefix("z") {
            if compute(&initial, &connections, key, 1000).unwrap() {
                z |= 1 << index.parse::<usize>().unwrap();
            }
        }
    }
    z
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
    let (initial, connections) = input.split_once("\n\n").unwrap();
    let initial = initial
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(k, v)| (k, v.parse::<usize>().unwrap() == 1))
        .collect::<BTreeMap<_, _>>();
    let mut connections = connections
        .lines()
        .map(|l| {
            let mut tokens = l.split_whitespace();
            let left = tokens.next().unwrap();
            let op = tokens.next().unwrap();
            let right = tokens.next().unwrap();
            tokens.next();
            let result = tokens.next().unwrap();
            (result, (left, op, right))
        })
        .collect::<BTreeMap<_, _>>();

    let tmp = connections["REDACTED"];
    connections.insert("REDACTED", connections["REDACTED"]);
    connections.insert("REDACTED", tmp);
    let tmp = connections["REDACTED"];
    connections.insert("REDACTED", connections["REDACTED"]);
    connections.insert("REDACTED", tmp);
    let tmp = connections["REDACTED"];
    connections.insert("REDACTED", connections["REDACTED"]);
    connections.insert("REDACTED", tmp);
    let tmp = connections["REDACTED"];
    connections.insert("REDACTED", connections["REDACTED"]);
    connections.insert("REDACTED", tmp);

    // graphviz
    for (target, (l, op, r)) in &connections {
        println!("{} -> {} [label=\"{}\"]", l, target, op);
        println!("{} -> {} [label=\"{}\"]", r, target, op);

        if target.starts_with("z") {
            if *op != "XOR" {
                println!("ANOMALY: {} is not XOR\n", target);
            }
            if let Some(&(_, l_op, _)) = connections.get(l) {
                if let Some(&(_, r_op, _)) = connections.get(r) {
                    match (l_op, r_op) {
                        ("XOR", "OR") => (),
                        ("OR", "XOR") => (),
                        _ => {
                            println!("ANOMALY: l_op: {l_op} r_op: {r_op} (parent of {target})");
                        }
                    }
                }
            }
        }
    }

    let mut y: usize = 0;
    let mut x: usize = 0;
    for a in &initial {
        if let Some(index) = a.0.strip_prefix("y") {
            if *a.1 {
                y |= 1 << index.parse::<usize>().unwrap();
            }
        } else if let Some(index) = a.0.strip_prefix("x") {
            if *a.1 {
                x |= 1 << index.parse::<usize>().unwrap();
            }
        }
    }
    let mut z: usize = 0;
    for key in connections.keys() {
        if let Some(index) = key.strip_prefix("z") {
            if compute(&initial, &connections, key, 1000).unwrap() {
                z |= 1 << index.parse::<usize>().unwrap();
            }
        }
    }
    dbg!(z, x + y);
    let diff = z & !(x + y);
    println!("diff: {:b}", diff);

    todo!()
}

fn compute(
    initial: &BTreeMap<&str, bool>,
    connections: &BTreeMap<&str, (&str, &str, &str)>,
    target: &str,
    depth: usize,
) -> Option<bool> {
    if depth == 0 {
        return None;
    }
    if let Some(result) = initial.get(target) {
        return Some(*result);
    }
    let (left, op, right) = connections[target];
    let l = initial
        .get(left)
        .cloned()
        .or_else(|| compute(initial, connections, left, depth - 1))?;
    let r = initial
        .get(right)
        .cloned()
        .or_else(|| compute(initial, connections, right, depth - 1))?;
    let result = match op {
        "AND" => l & r,
        "OR" => l | r,
        "XOR" => l ^ r,
        _ => unreachable!(),
    };
    Some(result)
}

#[test]
fn test_part1() {
    let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    assert_eq!(part1(input), 4);

    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    assert_eq!(part1(input), 2024);
}

#[test]
fn test_part2() {
    let input = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
    assert_eq!(part2(input), "z00,z01,z02,z05");
}
