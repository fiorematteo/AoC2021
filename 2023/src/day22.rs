use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Map = HashMap<usize, HashSet<usize>>;
type Bricks = Vec<(
    RangeInclusive<usize>,
    RangeInclusive<usize>,
    RangeInclusive<usize>,
)>;

#[aoc_generator(day22)]
fn generator(input: &str) -> (Map, Map, Bricks) {
    let mut bricks = input
        .lines()
        .map(|line| {
            let mut parts = line.split('~');
            let start = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let end = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            assert!(start[0] <= end[0]);
            assert!(start[1] <= end[1]);
            assert!(start[2] <= end[2]);
            let x_points = start[0]..=end[0];
            let y_points = start[1]..=end[1];
            let z_points = start[2]..=end[2];
            (x_points, y_points, z_points)
        })
        .collect::<Vec<_>>();

    let max_x = bricks.iter().map(|(x, _, _)| *x.end()).max().unwrap();
    let max_y = bricks.iter().map(|(_, y, _)| *y.end()).max().unwrap();
    let max_z = bricks.iter().map(|(_, _, z)| *z.end()).max().unwrap();

    let mut map = vec![vec![vec![-1; max_z + 1]; max_x + 1]; max_y + 1];
    for (i, brick) in bricks.iter().enumerate() {
        for x in brick.0.clone() {
            for y in brick.1.clone() {
                for z in brick.2.clone() {
                    map[y][x][z] = i as i64;
                }
            }
        }
    }

    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut settled_bricks: HashSet<usize> = HashSet::new();
    while settled_bricks.len() != bricks.len() {
        let (i, lowest_brick) = bricks
            .iter()
            .enumerate()
            .filter(|(i, _)| !settled_bricks.contains(i))
            .min_by_key(|(_, brick)| *brick.2.start())
            .unwrap();
        settled_bricks.insert(i);
        let mut z_start = *lowest_brick.2.start();
        let mut z_end = *lowest_brick.2.end();
        let mut yx_points = vec![];
        for x in lowest_brick.0.clone() {
            for y in lowest_brick.1.clone() {
                yx_points.push((y, x));
            }
        }

        loop {
            let mut found_support = false;
            for &(y, x) in yx_points.iter() {
                if z_start - 1 == 0 {
                    found_support = true;
                } else if map[y][x][z_start - 1] != -1 {
                    supports
                        .entry(map[y][x][z_start - 1] as usize)
                        .or_default()
                        .insert(i);
                    found_support = true;
                };
            }
            if found_support {
                break;
            }
            z_start -= 1;
            z_end -= 1;
        }

        for x in lowest_brick.0.clone() {
            for y in lowest_brick.1.clone() {
                for z in 0..max_z {
                    if map[y][x][z] == i as i64 {
                        map[y][x][z] = -1;
                    }
                }
            }
        }

        for x in lowest_brick.0.clone() {
            for y in lowest_brick.1.clone() {
                for z in z_start..=z_end {
                    assert!(map[y][x][z] == -1 || map[y][x][z] == i as _);
                    map[y][x][z] = i as i64;
                }
            }
        }

        bricks[i].2 = z_start..=z_end;
    }

    let mut is_supported: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (brick, supported) in supports.clone() {
        for s in supported {
            is_supported.entry(s).or_default().insert(brick);
        }
    }

    (supports, is_supported, bricks)
}

#[aoc(day22, part1)]
fn part1((supports, is_supported, bricks): &(Map, Map, Bricks)) -> usize {
    let mut count = 0;
    for brick in 0..bricks.len() {
        if let Some(supported) = supports.get(&brick) {
            if supported
                .iter()
                .all(|s| is_supported.get(s).map(|s| s.len() > 1).unwrap_or(false))
            {
                count += 1;
            }
        } else {
            count += 1;
        }
    }
    count
}

#[aoc(day22, part2)]
fn part2((supports, is_supported, bricks): &(Map, Map, Bricks)) -> usize {
    (0..bricks.len())
        .map(|brick| count_reachable(supports, is_supported, brick))
        .sum()
}

fn count_reachable(
    supports: &HashMap<usize, HashSet<usize>>,
    is_supported: &HashMap<usize, HashSet<usize>>,
    brick: usize,
) -> usize {
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(brick);
    let mut queue = vec![brick];
    while let Some(supported) = queue.pop() {
        if let Some(children) = supports.get(&supported) {
            for child in children {
                if !visited.contains(child)
                    && is_supported[child]
                        .iter()
                        .filter(|b| !visited.contains(b))
                        .count()
                        == 0
                {
                    visited.insert(*child);
                    queue.push(*child);
                }
            }
        }
    }
    visited.len() - 1
}

#[test]
fn test_part1() {
    let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    assert_eq!(part1(&generator(input)), 5);
}

#[test]
fn test_part2() {
    let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    assert_eq!(part2(&generator(input)), 7);
}
