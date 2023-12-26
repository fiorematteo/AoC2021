use rand::prelude::*;

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let (node, v) = line.split_once(": ").unwrap();
        nodes.push(node.to_string());
        for next in v.split(' ').map(|s| s.to_string()) {
            edges.push((node.to_string(), next.to_string()));
            edges.push((next.to_string(), node.to_string()));
        }
    }
    nodes.sort();
    nodes.dedup();
    edges.sort();
    edges.dedup();

    loop {
        let mut edges = edges.clone();
        let mut nodes = nodes.clone();

        while nodes.len() > 2 {
            // get random edge
            let edge = edges.choose(&mut rand::thread_rng()).unwrap().clone();
            contract(&mut nodes, &mut edges, edge);
        }
        if edges.len() == 6 {
            let a = nodes[0].split('#').count();
            let b = nodes[1].split('#').count();
            return a * b;
        }
    }
}

fn contract(nodes: &mut Vec<String>, edges: &mut Vec<(String, String)>, edge: (String, String)) {
    let (a, b) = edge.clone();
    edges.retain(|e| *e != edge);
    edges.retain(|e| *e != (b.clone(), a.clone()));
    nodes.retain(|n| *n != a && *n != b);

    let new_node = format!("{}#{}", a, b);
    for edge in edges.iter_mut() {
        if edge.0 == a || edge.0 == b {
            edge.0 = new_node.clone();
        }
        if edge.1 == a || edge.1 == b {
            edge.1 = new_node.clone();
        }
    }
    nodes.push(new_node);
}

#[test]
fn test_part1() {
    let input = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    assert_eq!(part1(input), 54);
}
