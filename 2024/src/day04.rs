#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut letters = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                for direction in (-1..=1)
                    .into_iter()
                    .flat_map(|y| (-1..=1).into_iter().map(move |x| (y, x)))
                    .filter(|&(y, x)| !(y == 0 && x == 0))
                {
                    letters.push((y, x, direction, 'X'))
                }
            }
        }
    }
    let mut xmas_counter = 0;
    while let Some((y, x, direction, last_letter)) = letters.pop() {
        let (delta_y, delta_x) = direction;
        let next_y = y as i32 + delta_y;
        if next_y < 0 || next_y as usize == grid.len() {
            continue;
        }
        let next_x = x as i32 + delta_x;
        if next_x < 0 || next_x as usize == grid[0].len() {
            continue;
        }
        let (next_y, next_x) = (next_y as usize, next_x as usize);

        match (last_letter, grid[next_y as usize][next_x]) {
            ('X', 'M') => letters.push((next_y, next_x, direction, 'M')),
            ('M', 'A') => letters.push((next_y, next_x, direction, 'A')),
            ('A', 'S') => xmas_counter += 1,
            _ => (),
        }
    }
    xmas_counter
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut letters = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'M' {
                for direction in [(1, 1), (-1, -1), (1, -1), (-1, 1)] {
                    letters.push((y, x, direction, 'M'))
                }
            }
        }
    }
    let mut freq = vec![vec![0; grid[0].len()]; grid.len()];
    while let Some((y, x, direction, last_letter)) = letters.pop() {
        let (delta_y, delta_x) = direction;
        let next_y = y as i32 + delta_y;
        if next_y < 0 || next_y as usize == grid.len() {
            continue;
        }
        let next_x = x as i32 + delta_x;
        if next_x < 0 || next_x as usize == grid[0].len() {
            continue;
        }
        let (next_y, next_x) = (next_y as usize, next_x as usize);

        match (last_letter, grid[next_y as usize][next_x]) {
            ('M', 'A') => letters.push((next_y, next_x, direction, 'A')),
            ('A', 'S') => freq[y][x] += 1,
            _ => (),
        }
    }
    freq.into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&f| f == 2)
        .count() as _
}

#[test]
pub fn part1_test() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(part1(input), 18);
}

#[test]
pub fn part1_short_test() {
    let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
    assert_eq!(part1(input), 4);
}
