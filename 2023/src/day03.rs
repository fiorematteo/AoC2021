use std::collections::HashMap;

type Table = Vec<Vec<char>>;
type Numbers = Vec<(u32, Vec<(usize, usize)>)>;

#[aoc_generator(day3)]
pub fn generator(input: &str) -> (Table, Numbers) {
    let table: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut current_number = Vec::new();
    let mut numbers: Vec<(u32, Vec<_>)> = Vec::new();
    for y in 0..table.len() {
        for x in 0..table[0].len() {
            if table[y][x].is_ascii_digit() {
                current_number.push((y, x));
            } else if !current_number.is_empty() {
                let value = current_number
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &(y, x))| table[y][x].to_digit(10).unwrap() * 10_u32.pow(i as u32))
                    .sum();
                numbers.push((value, current_number.clone()));
                current_number.clear();
            }
        }
    }
    (table, numbers)
}

#[aoc(day3, part1)]
pub fn part1((table, numbers): &(Table, Numbers)) -> u32 {
    numbers
        .iter()
        .filter_map(|(v, positions)| {
            for (y, x) in positions {
                for dy in -1..=1 {
                    let new_y = *y as i32 + dy;
                    if new_y < 0 || new_y >= table.len() as i32 {
                        continue;
                    }
                    for dx in -1..=1 {
                        let new_x = *x as i32 + dx;
                        if new_x < 0 || new_x >= table[0].len() as i32 {
                            continue;
                        }
                        let c = table[new_y as usize][new_x as usize];
                        if !c.is_ascii_digit() && c != '.' {
                            return Some(v);
                        }
                    }
                }
            }
            None
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2((table, numbers): &(Table, Numbers)) -> u32 {
    let mut gears: HashMap<(usize, usize), Vec<_>> = HashMap::new();
    numbers.iter().for_each(|(v, positions)| {
        for (y, x) in positions {
            for dy in -1..=1 {
                let new_y = *y as i32 + dy;
                if new_y < 0 || new_y >= table.len() as i32 {
                    continue;
                }
                for dx in -1..=1 {
                    let new_x = *x as i32 + dx;
                    if new_x < 0 || new_x >= table[0].len() as i32 {
                        continue;
                    }
                    let c = table[new_y as usize][new_x as usize];
                    if c == '*' {
                        gears
                            .entry((new_y as usize, new_x as usize))
                            .and_modify(|e| e.push(v))
                            .or_insert(vec![v]);
                        return;
                    }
                }
            }
        }
    });

    gears
        .into_iter()
        .filter_map(|(_, v)| {
            if v.len() != 2 {
                None
            } else {
                Some(v[0] * v[1])
            }
        })
        .sum()
}

#[test]
fn part1_test() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part1(&generator(input)), 4361);
}

#[test]
fn part2_test() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part2(&generator(input)), 467835);
}
