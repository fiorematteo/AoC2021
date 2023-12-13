fn solve(input: &str, target: usize) -> usize {
    let mut total = 0;
    for map in input.split("\n\n") {
        let map: Vec<Vec<bool>> = map
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let horizontal = (1..map.len())
            .filter(|&i| horizontal_reflecting(&map, i) == target)
            .collect::<Vec<_>>();
        let vertical = (1..map[0].len())
            .filter(|&i| vertical_reflecting(&map, i) == target)
            .collect::<Vec<_>>();
        if !vertical.is_empty() {
            total += vertical[0];
        } else if !horizontal.is_empty() {
            total += horizontal[0] * 100;
        }
    }
    total
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    solve(input, 0)
}

#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
    solve(input, 1)
}

fn horizontal_reflecting(map: &Vec<Vec<bool>>, index: usize) -> usize {
    let mut differences = 0;
    for i in 0..index {
        let to_check = index + index.abs_diff(i) - 1;
        if to_check >= map.len() {
            continue;
        }
        differences += (0..map[0].len())
            .filter(|&x| map[i][x] != map[to_check][x])
            .count();
    }
    differences
}

fn vertical_reflecting(map: &Vec<Vec<bool>>, index: usize) -> usize {
    let mut differences = 0;
    for i in 0..index {
        let to_check = index + index.abs_diff(i) - 1;
        if to_check >= map[0].len() {
            continue;
        }
        differences += (0..map.len())
            .filter(|&y| map[y][i] != map[y][to_check])
            .count();
    }
    differences
}

#[test]
fn test_part1() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!(part1(input), 405);
}

#[test]
fn test_part2() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!(part2(input), 400);
}
