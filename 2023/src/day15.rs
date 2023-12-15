use std::collections::BTreeMap;

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let nums: Vec<_> = input
        .split(',')
        .map(|s| {
            let cs = s.chars().collect::<Vec<_>>();
            if cs[cs.len() - 1] == '-' {
                (&s[..s.len() - 1], Op::Remove)
            } else {
                (
                    &s[..s.len() - 2],
                    Op::Add(cs[cs.len() - 1].to_digit(10).unwrap() as usize),
                )
            }
        })
        .collect();

    let mut boxes: BTreeMap<usize, Vec<(&str, usize)>> = BTreeMap::new();
    for (label, op) in nums {
        let hash = hash(label);
        match op {
            Op::Add(n) => {
                boxes
                    .entry(hash)
                    .and_modify(|v| {
                        let pos = v.iter().position(|&(l, _)| l == label);
                        if let Some(pos) = pos {
                            v[pos] = (label, n);
                        } else {
                            v.push((label, n));
                        }
                    })
                    .or_insert(vec![(label, n)]);
            }
            Op::Remove => {
                boxes
                    .entry(hash)
                    .and_modify(|v| v.retain(|&(l, _)| l != label));
            }
        };
    }
    boxes.into_iter().fold(0_usize, |acc, (box_id, v)| {
        acc + v
            .into_iter()
            .enumerate()
            .map(|(i, (_, power))| (i + 1) * power * (box_id + 1))
            .sum::<usize>()
    })
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((c as usize + acc) * 17) % 256)
}

#[derive(Debug)]
enum Op {
    Add(usize),
    Remove,
}

#[test]
fn test_part1() {
    assert_eq!(part1("HASH"), 52);

    assert_eq!(
        part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    );
}
