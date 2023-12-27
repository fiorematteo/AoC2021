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

    count_reachable(&map, pos.0, pos.1, steps)
}

fn count_reachable(map: &Vec<Vec<char>>, start_y: usize, start_x: usize, steps: usize) -> usize {
    let mut queue = HashSet::new();
    queue.insert((start_y, start_x));
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

#[aoc(day21, part2)]
fn part2(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = (y, x);
            }
        }
    }

    let target = 26501365;
    let (x_1, x_2, x_3) = (64, 195, 326);

    let mut y_1 = 0;
    let mut y_2 = 0;
    let mut y_3 = 0;

    let mut queue = HashSet::new();
    queue.insert((start.0, start.1));
    for i in 0.. {
        let old_queue = queue.clone();
        queue.clear();
        for (y, x) in old_queue.into_iter() {
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let n_y = y as i64 + dy;
                let n_x = x as i64 + dx;
                let mod_y = n_y.rem_euclid(map.len() as i64);
                let mod_x = n_x.rem_euclid(map[0].len() as i64);
                if map[mod_y as usize][mod_x as usize] != '#' {
                    queue.insert((n_y as usize, n_x as usize));
                }
            }
        }
        if i == x_1 {
            y_1 = queue.len() as i64;
        }
        if i == x_2 {
            y_2 = queue.len() as i64;
        }
        if i == x_3 {
            y_3 = queue.len() as i64;
            break;
        }
    }

    // lagrange interpolation
    let f = |x: i64| {
        let x = x as f64;
        let (x_1, x_2, x_3) = (x_1 as f64, x_2 as f64, x_3 as f64);
        let (y_1, y_2, y_3) = (y_1 as f64, y_2 as f64, y_3 as f64);

        let a = y_1 * (x - x_2) * (x - x_3) / ((x_1 - x_2) * (x_1 - x_3));
        let b = y_2 * (x - x_3) * (x - x_1) / ((x_2 - x_1) * (x_2 - x_3));
        let c = y_3 * (x - x_1) * (x - x_2) / ((x_3 - x_1) * (x_3 - x_2));
        a + b + c
    };

    assert_eq!(f(x_1), y_1 as _);
    assert_eq!(f(x_2), y_2 as _);
    assert_eq!(f(x_3), y_3 as _);

    f(target - 1) as _
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
