use std::collections::{BTreeMap, BTreeSet};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let mut edges = BTreeMap::new();
    for (from, to) in input.lines().map(|l| l.split_once('-').unwrap()) {
        edges.entry(from).or_insert(Vec::new()).push(to);
        edges.entry(to).or_insert(Vec::new()).push(from);
    }
    let mut cliques = BTreeSet::new();
    for (a, near_a) in &edges {
        for b in near_a {
            let near_b = &edges[b];
            for c in near_b {
                if edges[c].contains(a) {
                    let mut key = vec![a, b, c];
                    if key.iter().any(|x| x.starts_with('t')) {
                        key.sort();
                        cliques.insert(key);
                    }
                }
            }
        }
    }
    cliques.len()
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let mut edges = BTreeMap::new();
    for (from, to) in input.lines().map(|l| l.split_once('-').unwrap()) {
        edges.entry(from).or_insert(Vec::new()).push(to);
        edges.entry(to).or_insert(Vec::new()).push(from);
    }
    let vertices = edges.keys().cloned().collect::<BTreeSet<_>>();
    let mut cliques = BTreeSet::new();
    bron_kerbosch(
        BTreeSet::new(),
        vertices.clone(),
        BTreeSet::new(),
        &edges,
        &mut cliques,
    );
    let mut largest = cliques.iter().max_by_key(|c| c.len()).unwrap().clone();
    largest.sort();
    largest.join(",")
}

fn bron_kerbosch<'a>(
    r: BTreeSet<&'a str>,
    mut p: BTreeSet<&'a str>,
    mut x: BTreeSet<&'a str>,
    edges: &BTreeMap<&str, Vec<&'a str>>,
    cliques: &mut BTreeSet<Vec<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.insert(r.iter().cloned().collect());
    }
    while let Some(v) = p.pop_first() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let v_neighbors: BTreeSet<_> = edges[v].iter().cloned().collect();
        let new_p: BTreeSet<_> = p.intersection(&v_neighbors).cloned().collect();
        let new_x: BTreeSet<_> = x.intersection(&v_neighbors).cloned().collect();
        bron_kerbosch(new_r, new_p, new_x, edges, cliques);
        x.insert(v);
    }
}

#[test]
fn test_part1() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    assert_eq!(part1(input), 7);
}

#[test]
fn test_part2() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    assert_eq!(part2(input), "co,de,ka,ta");
}
