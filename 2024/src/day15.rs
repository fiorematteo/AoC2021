use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Sub},
};
#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<_>> = grid.lines().map(|line| line.chars().collect()).collect();
    let instructions = instructions.chars().filter_map(|c| match c {
        '<' => Some(Pair { x: -1, y: 0 }),
        '>' => Some(Pair { x: 1, y: 0 }),
        '^' => Some(Pair { x: 0, y: -1 }),
        'v' => Some(Pair { x: 0, y: 1 }),
        '\n' => None,
        _ => unreachable!(),
    });

    let mut robot = Pair { x: 0, y: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                robot = Pair {
                    x: x as i64,
                    y: y as i64,
                }
            }
        }
    }

    for step in instructions {
        if try_move(robot, step, &mut grid) {
            robot += step;
        }
    }

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'O' {
                total += y * 100 + x;
            }
        }
    }
    total
}

fn try_move(from: Pair, dir: Pair, grid: &mut [Vec<char>]) -> bool {
    let next = from + dir;
    let next_c = grid[next.y as usize][next.x as usize];
    let from_c = grid[from.y as usize][from.x as usize];
    if next_c == '#' {
        return false;
    } else if next_c == '.'
        || ((next_c == 'O' || next_c == ']' || next_c == '[') && try_move(next, dir, grid))
    {
        grid[next.y as usize][next.x as usize] = from_c;
        grid[from.y as usize][from.x as usize] = '.';
        return true;
    }
    false
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<_>> = grid
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '.' => ['.', '.'].into_iter(),
                    '#' => ['#', '#'].into_iter(),
                    '@' => ['@', '.'].into_iter(),
                    'O' => ['[', ']'].into_iter(),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let instructions = instructions.chars().filter_map(|c| match c {
        '<' => Some(Pair { x: -1, y: 0 }),
        '>' => Some(Pair { x: 1, y: 0 }),
        '^' => Some(Pair { x: 0, y: -1 }),
        'v' => Some(Pair { x: 0, y: 1 }),
        '\n' => None,
        _ => unreachable!(),
    });

    let mut robot = Pair { x: 0, y: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                robot = Pair {
                    x: x as i64,
                    y: y as i64,
                }
            }
        }
    }

    for step in instructions {
        let next = robot + step;
        if grid[next.y as usize][next.x as usize] == '#' {
            continue;
        }
        if step.y == 0 || grid[next.y as usize][next.x as usize] == '.' {
            // use part1
            if try_move(robot, step, &mut grid) {
                robot = next;
            }
        } else {
            assert!(
                grid[next.y as usize][next.x as usize] == '['
                    || grid[next.y as usize][next.x as usize] == ']'
            );
            // bfs to find all boxes that can be moved
            let mut boxes = Vec::new();
            let mut queue: VecDeque<_> = vec![robot].into();
            while let Some(new_box) = queue.pop_front() {
                if boxes.contains(&new_box) {
                    continue;
                }
                boxes.push(new_box);
                let next_step = new_box + step;
                match grid[next_step.y as usize][next_step.x as usize] {
                    '#' => {
                        // don't move any box
                        boxes.clear();
                        break;
                    }
                    '[' => {
                        queue.push_back(next_step);
                        queue.push_back(next_step + Pair { y: 0, x: 1 });
                    }
                    ']' => {
                        queue.push_back(next_step);
                        queue.push_back(next_step + Pair { y: 0, x: -1 });
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }
            if boxes.is_empty() {
                continue;
            }
            // children first
            for &box_pos in boxes.iter().rev() {
                let tmp = grid[box_pos.y as usize][box_pos.x as usize];
                grid[box_pos.y as usize][box_pos.x as usize] = '.';
                grid[(box_pos + step).y as usize][(box_pos + step).x as usize] = tmp;
            }
            robot = next;
        }
    }

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '[' {
                total += y * 100 + x;
            }
        }
    }
    total
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i64,
    y: i64,
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Pair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pair {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i64, i64)> for Pair {
    fn from(value: (i64, i64)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

#[test]
fn test_part1() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    assert_eq!(part1(input), 2028);
}

#[test]
fn test_part2() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    assert_eq!(part2(input), 9021);
}
