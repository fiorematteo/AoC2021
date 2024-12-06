#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut guard = (0_i32, 0_i32);
    for (y, row) in grid.iter().enumerate() {
        for (x, &el) in row.iter().enumerate() {
            if el == '^' {
                guard = (y as _, x as _);
            }
        }
    }
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut total = 0;
    let mut direction = (-1, 0);
    loop {
        if !visited[guard.0 as usize][guard.1 as usize] {
            visited[guard.0 as usize][guard.1 as usize] = true;
            total += 1;
        }
        if !step(&grid, &mut guard, &mut direction) {
            break;
        }
    }
    total
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut guard = (0_i32, 0_i32);
    for (y, row) in grid.iter().enumerate() {
        for (x, &el) in row.iter().enumerate() {
            if el == '^' {
                guard = (y as _, x as _);
            }
        }
    }
    let mut total = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut direction = (-1, 0);
    loop {
        let has_visited = &mut visited[guard.0 as usize][guard.1 as usize];
        if !*has_visited {
            *has_visited = true;
        }

        let obstacle = (guard.0 + direction.0, guard.1 + direction.1);
        if in_bounds(obstacle, &grid)
            && grid[obstacle.0 as usize][obstacle.1 as usize] == '.'
            && !visited[obstacle.0 as usize][obstacle.1 as usize]
        {
            grid[obstacle.0 as usize][obstacle.1 as usize] = '#';
            if is_looping(&grid, (guard.0 as i32, guard.1 as i32), direction, false) {
                total += 1;
            }
            grid[obstacle.0 as usize][obstacle.1 as usize] = '.';
        }

        if !step(&grid, &mut guard, &mut direction) {
            break;
        }
    }
    total
}

fn dump_grid(grid: &[Vec<char>], (obstacle_y, obstacle_x): (usize, usize)) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if x == obstacle_x && y == obstacle_y {
                print!("O");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}

fn is_looping(
    grid: &[Vec<char>],
    mut guard: (i32, i32),
    mut direction: (i32, i32),
    log: bool,
) -> bool {
    let mut visited = vec![vec![(false, vec![]); grid[0].len()]; grid.len()];
    loop {
        if log {
            println!("{guard:?}");
        }
        let (has_visited, visited_directions) = &mut visited[guard.0 as usize][guard.1 as usize];
        if *has_visited {
            if visited_directions.contains(&direction) {
                return true;
            }
        } else {
            *has_visited = true;
        }
        visited_directions.push(direction);
        if !step(&grid, &mut guard, &mut direction) {
            break;
        }
    }
    false
}

fn step(grid: &[Vec<char>], guard: &mut (i32, i32), direction: &mut (i32, i32)) -> bool {
    let next_position = (guard.0 + direction.0, guard.1 + direction.1);
    if !in_bounds(next_position, &grid) {
        return false;
    }
    if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
        // rotate
        *direction = match *direction {
            (1, 0) => (0, -1),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (0, -1) => (-1, 0),
            _ => unreachable!(),
        };
    } else {
        *guard = next_position;
    }
    true
}

fn in_bounds((y, x): (i32, i32), grid: &[Vec<char>]) -> bool {
    y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32
}

#[test]
fn test_part1() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part1(input), 41);
}

#[test]
fn test_part2() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part2(input), 6);
}

#[test]
fn test_part2_line() {
    let input = "......
.#^..#
....#.";
    assert_eq!(part2(input), 1);
}
