use std::collections::HashMap;

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (data, numbers) = line.split_once(' ').unwrap();
            let data = data.chars().collect::<Vec<_>>();
            let numbers = numbers
                .split(',')
                .map(&str::parse)
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();
            (data, numbers)
        })
        .map(|(d, n)| solve_line(&mut HashMap::new(), &d, &n, 0, 0, 0))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (data, numbers) = line.split_once(' ').unwrap();
            let data = [data, data, data, data, data]
                .join("?")
                .chars()
                .collect::<Vec<_>>();
            let numbers = numbers
                .split(',')
                .map(&str::parse)
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();
            (data, numbers.repeat(5))
        })
        .map(|(d, n)| solve_line(&mut HashMap::new(), &d, &n, 0, 0, 0))
        .sum()
}

fn solve_line(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    data: &[char],
    blocks: &[usize],
    index: usize,
    b_index: usize,
    block_len: usize,
) -> usize {
    let key = (index, b_index, block_len);
    if let Some(&v) = cache.get(&key) {
        return v;
    }
    if index == data.len() {
        if b_index == blocks.len() && block_len == 0 {
            // no more blocks
            return 1;
        } else if b_index == blocks.len() - 1 && block_len == blocks[b_index] {
            // last block
            return 1;
        } else {
            return 0;
        }
    }
    let mut out = 0;
    if data[index] == '.' || data[index] == '?' {
        // put .
        if block_len == 0 {
            // new block
            out += solve_line(cache, data, blocks, index + 1, b_index, 0);
        } else if b_index < blocks.len() && blocks[b_index] == block_len {
            // skip to next block
            out += solve_line(cache, data, blocks, index + 1, b_index + 1, 0);
        }
    }
    if data[index] == '#' || data[index] == '?' {
        // put #
        out += solve_line(cache, data, blocks, index + 1, b_index, block_len + 1);
    }
    cache.insert(key, out);
    out
}

#[test]
fn test_part1() {
    let input = r#"???.### 1,1,3"#;
    assert_eq!(part1(input), 1);

    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    assert_eq!(part1(input), 21);

    let input = r#"?###???????? 3,2,1"#;
    assert_eq!(part1(input), 10);
}
