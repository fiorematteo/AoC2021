use std::collections::HashSet;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    let mut positions = HashSet::new();
    positions.insert(tail);
    for line in input.lines() {
        let (direction, n) = line.split_once(' ').unwrap();
        let n = n.parse::<i32>().unwrap();

        for _ in 0..n {
            match direction {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                _ => unreachable!(),
            }
            let x_distance = (head.0 - tail.0).abs();
            let y_distance = (head.1 - tail.1).abs();
            if x_distance > 1 || y_distance > 1 {
                if head.0 == tail.0 {
                    // only move y
                    tail.1 += if head.1 > tail.1 { 1 } else { -1 };
                } else if head.1 == tail.1 {
                    // only move x
                    tail.0 += if head.0 > tail.0 { 1 } else { -1 };
                } else {
                    // move both
                    tail.0 += if head.0 > tail.0 { 1 } else { -1 };
                    tail.1 += if head.1 > tail.1 { 1 } else { -1 };
                }
                positions.insert(tail);
            }
        }
    }
    positions.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut head = (0i32, 0i32);
    let mut tail = (0..9).map(|_| (0i32, 0i32)).collect::<Vec<_>>();
    let mut positions = HashSet::new();
    positions.insert(head);
    for line in input.lines() {
        let (direction, n) = line.split_once(' ').unwrap();
        let n = n.parse::<i32>().unwrap();

        for _ in 0..n {
            match direction {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                _ => unreachable!(),
            }

            let mut i = 0;
            let mut current_head = head;
            while i < tail.len() {
                let x_distance = (current_head.0 - tail[i].0).abs();
                let y_distance = (current_head.1 - tail[i].1).abs();
                if x_distance > 1 || y_distance > 1 {
                    if current_head.0 == tail[i].0 {
                        // only move y
                        tail[i].1 += if current_head.1 > tail[i].1 { 1 } else { -1 };
                    } else if current_head.1 == tail[i].1 {
                        // only move x
                        tail[i].0 += if current_head.0 > tail[i].0 { 1 } else { -1 };
                    } else {
                        // move both
                        tail[i].0 += if current_head.0 > tail[i].0 { 1 } else { -1 };
                        tail[i].1 += if current_head.1 > tail[i].1 { 1 } else { -1 };
                    }
                }
                current_head = tail[i];
                i += 1;
            }
            positions.insert(tail[8]);
        }
    }
    positions.len()
}
