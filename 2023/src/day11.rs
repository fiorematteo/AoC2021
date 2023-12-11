#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    solve(input, 1_000_000)
}
fn solve(input: &str, expand: usize) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<bool>>>();

    let empty_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .filter_map(|(index, row)| {
            if !row.iter().any(|&b| b) {
                Some(index)
            } else {
                None
            }
        })
        .collect();
    let empty_cols: Vec<usize> = map[0]
        .iter()
        .enumerate()
        .filter_map(|(index, _)| {
            if !map.iter().any(|row| row[index]) {
                Some(index)
            } else {
                None
            }
        })
        .collect();

    let mut galaxies = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el {
                galaxies.push((y, x));
            }
        }
    }

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let y_range = galaxies[i].0.min(galaxies[j].0)..galaxies[i].0.max(galaxies[j].0);
            let x_range = galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1);
            total += y_range.len() + x_range.len();

            for row in &empty_rows {
                if y_range.contains(row) {
                    total += expand - 1;
                }
            }
            for col in &empty_cols {
                if x_range.contains(col) {
                    total += expand - 1;
                }
            }
        }
    }
    total
}

#[test]
fn test_part1() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    assert_eq!(part1(input), 374);
}

#[test]
fn test_part2() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    assert_eq!(solve(input, 10), 1030);
}
