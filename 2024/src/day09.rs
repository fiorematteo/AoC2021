#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut total = 0;
    let mut expanded = Vec::new();
    let mut id = 0;
    for (i, n) in input.chars().enumerate() {
        let n = n.to_digit(10).unwrap();
        if i % 2 == 0 {
            // block
            for _ in 0..n {
                expanded.push(Some(id));
            }
            id += 1;
        } else {
            // empty
            for _ in 0..n {
                expanded.push(None);
            }
        }
    }

    // two pointers
    let mut left = 0;
    let mut right = expanded.len() - 1;
    while left <= right {
        if let Some(id) = expanded[left] {
            total += left * id;
            left += 1;
        } else {
            if let Some(id) = expanded[right] {
                total += left * id;
                left += 1;
            }
            right -= 1;
        }
    }

    total
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let mut blocks = Vec::new();
    let mut id = 0;
    for (i, n) in input.chars().enumerate() {
        let n = n.to_digit(10).unwrap() as i64;
        if i % 2 == 0 {
            if i == 0 {
                blocks.push(Block::File(id, n, true));
            } else {
                blocks.push(Block::File(id, n, false));
            }
            id += 1;
        } else {
            blocks.push(Block::Free(n));
        }
    }
    // two pointers
    let mut right = blocks.len() - 1;
    while right > 0 {
        let mut left = 0;
        while left <= right {
            let Block::Free(free_space) = blocks[left] else {
                left += 1;
                continue;
            };
            let Block::File(id, size, false) = blocks[right] else {
                right -= 1;
                continue;
            };
            let diff = free_space - size;
            if diff >= 0 {
                blocks[left] = Block::File(id, size, true);
                blocks[right] = Block::Free(size);
                if diff > 0 {
                    blocks.insert(left + 1, Block::Free(diff));
                }
                break;
            }
            left += 1;
        }
        if let Block::File(_, _, moved) = &mut blocks[right] {
            *moved = true;
        }
        right -= 1;
    }

    // hash
    let mut index = 0;
    let mut total = 0;
    for block in blocks {
        match block {
            Block::File(id, len, _) => {
                total += (index..index + len).map(|i| i * id).sum::<i64>();
                index += len;
            }
            Block::Free(len) => index += len,
        }
    }
    total
}

#[derive(Clone, Copy, Debug)]
enum Block {
    File(i64, i64, bool),
    Free(i64),
}

#[test]
fn test_part1() {
    let input = "2333133121414131402";
    assert_eq!(part1(input), 1928);
}

#[test]
fn test_part2() {
    let input = "2333133121414131402";
    assert_eq!(part2(input), 2858);
}
