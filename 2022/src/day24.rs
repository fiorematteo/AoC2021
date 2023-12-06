use std::collections::BinaryHeap;

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<Vec<Vec<bool>>> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>().to_vec())
        .collect::<Vec<_>>();
    let x_dim = lines[0].len();
    let y_dim = lines.len();
    let time_dim = num::integer::lcm(x_dim, y_dim);
    let mut field = vec![vec![vec![true; time_dim]; x_dim]; y_dim]; // true = empty
    for x in 0..x_dim {
        for y in 0..y_dim {
            match lines[y][x] {
                '.' => field[y][x][0] = true,
                '>' => {
                    let mut curr_x = x;
                    for time in 0..time_dim {
                        field[y][curr_x][time] = false;
                        curr_x += 1;
                        if curr_x == x_dim - 1 {
                            curr_x = 1
                        }
                    }
                }
                '<' => {
                    let mut curr_x = x;
                    for time in 0..time_dim {
                        field[y][curr_x][time] = false;
                        curr_x -= 1;
                        if curr_x == 0 {
                            curr_x = x_dim - 2
                        }
                    }
                }
                '^' => {
                    let mut curr_y = y;
                    for time in 0..time_dim {
                        field[curr_y][x][time] = false;
                        curr_y -= 1;
                        if curr_y == 0 {
                            curr_y = y_dim - 2
                        }
                    }
                }
                'v' => {
                    let mut curr_y = y;
                    for time in 0..time_dim {
                        field[curr_y][x][time] = false;
                        curr_y += 1;
                        if curr_y == y_dim - 1 {
                            curr_y = 1
                        }
                    }
                }
                '#' => {
                    for time in 0..time_dim {
                        field[y][x][time] = false;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    field
}

#[aoc(day24, part1)]
pub fn part1(field: &[Vec<Vec<bool>>]) -> usize {
    let x_dim = field[0].len();
    let y_dim = field.len();
    dijkstra(field, (0, 1), (y_dim - 1, x_dim - 2), 0)
}

#[aoc(day24, part2)]
pub fn part2(field: &[Vec<Vec<bool>>]) -> usize {
    let x_dim = field[0].len();
    let y_dim = field.len();

    let start = (0, 1);
    let goal = (y_dim - 1, x_dim - 2);
    let first_trip = dijkstra(field, start, goal, 0);
    let second_trip = dijkstra(field, goal, start, first_trip);
    first_trip + second_trip + dijkstra(field, start, goal, first_trip + second_trip)
}

pub fn dijkstra(
    field: &[Vec<Vec<bool>>],
    (start_y, start_x): (usize, usize),
    (end_y, end_x): (usize, usize),
    start_time: usize,
) -> usize {
    let y_dim = field.len();
    let x_dim = field[0].len();
    let time_dim = field[0][0].len();

    let mut dist = vec![vec![vec![std::usize::MAX; time_dim]; x_dim]; y_dim];
    let mut q = BinaryHeap::new();

    let start_time = start_time % time_dim;
    dist[start_y][start_x][start_time] = 0;
    q.push((0_usize, start_y as i64, start_x as i64, start_time as i64));

    let moves = vec![(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];

    while let Some((d, y, x, time)) = q.pop() {
        for &(dy, dx) in &moves {
            let ny = y + dy;
            if ny < 0 || ny >= y_dim as i64 {
                continue;
            }
            let nx = x + dx;
            if nx < 0 || nx >= x_dim as i64 {
                continue;
            }
            let ntime = (time + 1) % time_dim as i64;
            if ntime >= time_dim as i64 {
                continue;
            }

            let (ny, nx, ntime) = (ny as usize, nx as usize, ntime as usize);

            if field[ny][nx][ntime] {
                let nd = d + 1;
                if nd < dist[ny][nx][ntime] {
                    dist[ny][nx][ntime] = nd;
                    q.push((nd, ny as i64, nx as i64, ntime as i64));
                }
            }
        }
    }

    (0..time_dim)
        .map(|time| dist[end_y][end_x][time])
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
    assert_eq!(part1(&generator(input)), 18);
}

#[test]
fn test_part2() {
    let input = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;
    assert_eq!(part2(&generator(input)), 54);
}
