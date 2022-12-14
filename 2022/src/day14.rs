#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = vec![vec![Thing::Empty; 1000]; 1000];
    for line in input.lines() {
        let coords = line
            .split(" -> ")
            .map(|c| c.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect::<Vec<_>>();

        for points in coords.windows(2) {
            let start = points[0];
            let end = points[1];

            for x in start.0.min(end.0)..=start.0.max(end.0) {
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    grid[y as usize][x as usize] = Thing::Rock;
                }
            }
        }
    }
    grid[0][500] = Thing::Source;

    'outer: loop {
        let mut new_sand = (500, 1);
        loop {
            if new_sand.1 >= 999 {
                break 'outer;
            }
            if grid[new_sand.1 + 1][new_sand.0] == Thing::Empty {
                new_sand = (new_sand.0, new_sand.1 + 1);
            } else if grid[new_sand.1 + 1][new_sand.0 - 1] == Thing::Empty {
                new_sand = (new_sand.0 - 1, new_sand.1 + 1);
            } else if grid[new_sand.1 + 1][new_sand.0 + 1] == Thing::Empty {
                new_sand = (new_sand.0 + 1, new_sand.1 + 1);
            } else {
                break;
            }
        }
        grid[new_sand.1][new_sand.0] = Thing::Sand;
    }
    grid.iter()
        .map(|row| row.iter().filter(|t| **t == Thing::Sand).count())
        .sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let mut max_y: usize = 0;
    let mut grid = vec![vec![Thing::Empty; 1000]; 1000];
    for line in input.lines() {
        let coords = line
            .split(" -> ")
            .map(|c| c.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect::<Vec<_>>();

        for points in coords.windows(2) {
            let start = points[0];
            let end = points[1];

            for x in start.0.min(end.0)..=start.0.max(end.0) {
                for y in start.1.min(end.1)..=start.1.max(end.1) {
                    max_y = max_y.max(y as _);
                    grid[y as usize][x as usize] = Thing::Rock;
                }
            }
        }
    }
    for x in 0..1000 {
        grid[max_y + 2][x as usize] = Thing::Rock;
    }
    grid[0][500] = Thing::Source;

    while grid[0][500] == Thing::Source {
        let mut new_sand = (500, 0);
        loop {
            if grid[new_sand.1 + 1][new_sand.0] == Thing::Empty {
                new_sand = (new_sand.0, new_sand.1 + 1);
            } else if grid[new_sand.1 + 1][new_sand.0 - 1] == Thing::Empty {
                new_sand = (new_sand.0 - 1, new_sand.1 + 1);
            } else if grid[new_sand.1 + 1][new_sand.0 + 1] == Thing::Empty {
                new_sand = (new_sand.0 + 1, new_sand.1 + 1);
            } else {
                break;
            }
        }
        grid[new_sand.1][new_sand.0] = Thing::Sand;
    }
    grid.iter()
        .map(|row| row.iter().filter(|t| **t == Thing::Sand).count())
        .sum()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Thing {
    Sand,
    Source,
    Empty,
    Rock,
}
