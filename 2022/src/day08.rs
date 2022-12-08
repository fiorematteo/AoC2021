const SIZE: usize = 99;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = [[0; SIZE]; SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap();
        }
    }
    let mut is_visible = [[false; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            if x == 0 || y == 0 || x == SIZE - 1 || y == SIZE - 1 {
                is_visible[y][x] = true;
                continue;
            }
            let up = (0..y).all(|i| grid[i][x] < grid[y][x]);
            let down = ((y + 1)..SIZE).all(|i| grid[i][x] < grid[y][x]);
            let left = (0..x).all(|i| grid[y][i] < grid[y][x]);
            let right = ((x + 1)..SIZE).all(|i| grid[y][i] < grid[y][x]);

            is_visible[y][x] = up || down || left || right;
        }
    }
    is_visible.iter().flatten().filter(|&&v| v).count()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = [[0; SIZE]; SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c.to_digit(10).unwrap();
        }
    }
    let mut score = [[0; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            if x == 0 || y == 0 || x == SIZE - 1 || y == SIZE - 1 {
                score[y][x] = 0;
                continue;
            }
            let mut up = 0;
            for i in (0..y).rev() {
                up += 1;
                if grid[i][x] >= grid[y][x] {
                    break;
                }
            }
            let mut down = 0;
            for i in y + 1..SIZE {
                down += 1;
                if grid[i][x] >= grid[y][x] {
                    break;
                }
            }
            let mut left = 0;
            for i in (0..x).rev() {
                left += 1;
                if grid[y][i] >= grid[y][x] {
                    break;
                }
            }
            let mut right = 0;
            for i in x + 1..SIZE {
                right += 1;
                if grid[y][i] >= grid[y][x] {
                    break;
                }
            }
            score[y][x] = up * down * left * right;
        }
    }
    *score.iter().flatten().max().unwrap()
}
