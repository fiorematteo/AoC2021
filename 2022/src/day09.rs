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
                let x_condition = head.0 == tail.0;
                let y_condition = head.1 == tail.1;
                if x_condition || !y_condition {
                    tail.1 += if head.1 > tail.1 { 1 } else { -1 };
                }
                if x_condition || !y_condition {
                    tail.0 += if head.0 > tail.0 { 1 } else { -1 };
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
    let mut tail = vec![(0i32, 0i32); 9];
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

            let mut current_head = head;
            for i in 0..tail.len() {
                let x_distance = (current_head.0 - tail[i].0).abs();
                let y_distance = (current_head.1 - tail[i].1).abs();
                if x_distance > 1 || y_distance > 1 {
                    let x_condition = current_head.0 == tail[i].0;
                    let y_condition = current_head.1 == tail[i].1;

                    if x_condition || !y_condition {
                        tail[i].1 += if current_head.1 > tail[i].1 { 1 } else { -1 };
                    }
                    if y_condition || !x_condition {
                        tail[i].0 += if current_head.0 > tail[i].0 { 1 } else { -1 };
                    }
                }
                current_head = tail[i];
            }
            positions.insert(tail[8]);
        }
    }
    positions.len()
}
