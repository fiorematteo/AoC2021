use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0, 1, 1, 0, 1));
    while let Some((current_dist, y, x, prev_y, prev_x)) = queue.pop() {
        dist.entry((y, x))
            .and_modify(|d| {
                *d = (*d).max(current_dist);
            })
            .or_insert(current_dist);
        for &(dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ny, nx) = (y as i64 + dy, x as i64 + dx);
            if ny < 0 || nx < 0 || ny >= map.len() as i64 || nx >= map[0].len() as i64 {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if ny == prev_y && nx == prev_x {
                continue;
            }
            if map[ny][nx] == '.' {
                queue.push((current_dist + 1, ny, nx, y, x));
            } else if map[ny][nx] == '^' && dy == -1 {
                queue.push((current_dist + 2, ny - 1, nx, y, x));
            } else if map[ny][nx] == '>' && dx == 1 {
                queue.push((current_dist + 2, ny, nx + 1, y, x));
            } else if map[ny][nx] == 'v' && dy == 1 {
                queue.push((current_dist + 2, ny + 1, nx, y, x));
            } else if map[ny][nx] == '<' && dx == -1 {
                queue.push((current_dist + 2, ny, nx - 1, y, x));
            }
        }
    }
    dist[&(map.len() - 1, map[0].len() - 2)] + 1
}

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut branch_points: Vec<_> = (1..map.len() - 1)
        .flat_map(|y| (1..map[0].len() - 1).map(move |x| (y, x)))
        .filter(|&(y, x)| is_branch_point(&map, y, x))
        .collect();
    branch_points.push((0, 1));
    branch_points.push((map.len() - 1, map[0].len() - 2));

    let mut edges: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for (from_i, &(from_y, from_x)) in branch_points.iter().enumerate() {
        for (to_i, &(to_y, to_x)) in branch_points.iter().enumerate() {
            if from_y == to_y && from_x == to_x {
                continue;
            }
            let d = get_distance(&map, &branch_points, from_y, from_x, to_y, to_x);
            if d >= 100_000 {
                continue;
            }
            edges.entry(from_i).or_default().push((to_i, d));
        }
    }

    let start_node = branch_points
        .iter()
        .position(|&(y, x)| y == 0 && x == 1)
        .unwrap();
    let end_node = branch_points
        .iter()
        .position(|&(y, x)| y == map.len() - 1 && x == map[0].len() - 2)
        .unwrap();

    let mut dist: HashMap<usize, usize> = HashMap::new();
    let visited = Bitvec::new();
    let mut queue = vec![(0, start_node, visited)];
    while let Some((current_dist, node, visited)) = queue.pop() {
        if visited.contains(node) {
            continue;
        }
        dist.entry(node)
            .and_modify(|d| {
                *d = (*d).max(current_dist);
            })
            .or_insert(current_dist);
        for &(next, d) in &edges[&node] {
            let mut next_visited = visited;
            next_visited.insert(node);
            queue.push((current_dist + d, next, next_visited));
        }
    }
    dist[&end_node]
}

fn get_distance(
    map: &Vec<Vec<char>>,
    branch_points: &[(usize, usize)],
    from_y: usize,
    from_x: usize,
    to_y: usize,
    to_x: usize,
) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, from_y, from_x)));
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some(Reverse((mut current_dist, y, x))) = queue.pop() {
        if y == to_y && x == to_x {
            return current_dist;
        }
        if branch_points.contains(&(y, x)) && (y != from_y || x != from_x) {
            current_dist = 100_000;
        }
        if let Some(d) = dist.get(&(y, x)) {
            if *d <= current_dist {
                continue;
            }
        }
        dist.insert((y, x), current_dist);

        for &(dy, dx) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ny, nx) = (y as i64 + dy, x as i64 + dx);
            if ny < 0 || nx < 0 || ny >= map.len() as i64 || nx >= map[0].len() as i64 {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if map[ny][nx] != '#' {
                queue.push(Reverse((current_dist + 1, ny, nx)));
            }
        }
    }
    unreachable!()
}

fn is_branch_point(map: &[Vec<char>], y: usize, x: usize) -> bool {
    map[y][x] == '.'
        && [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter(|&(dy, dx)| map[(y as i64 + dy) as usize][(x as i64 + dx) as usize] != '#')
            .count()
            > 2
}

#[derive(Clone, Copy)]
struct Bitvec(u64);

impl Bitvec {
    const fn new() -> Self {
        Self(0)
    }

    fn contains(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    fn insert(&mut self, i: usize) {
        self.0 |= 1 << i;
    }
}

#[test]
fn test_part1() {
    let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
    assert_eq!(part1(input), 94);
}

#[test]
fn test_part2() {
    let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
    assert_eq!(part2(input), 154);
}
