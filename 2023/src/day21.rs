use std::collections::HashSet;

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    solve_part1(input, 64)
}
fn solve_part1(input: &str, steps: usize) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                pos = (y, x);
            }
        }
    }

    let mut queue = HashSet::new();
    queue.insert(pos);
    for _ in 0..steps {
        let old_queue = queue.clone();
        queue.clear();
        for (y, x) in old_queue.into_iter() {
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let n_y = y as i64 + dy;
                let n_x = x as i64 + dx;
                if 0 > n_y || n_y >= map.len() as i64 || 0 > n_x || n_x >= map[0].len() as i64 {
                    continue;
                }
                if map[n_y as usize][n_x as usize] == '#' {
                    continue;
                }
                queue.insert((n_y as usize, n_x as usize));
            }
        }
    }
    queue.len()
}

#[test]
fn test_part1() {
    let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    assert_eq!(solve_part1(input, 6), 16);
}
