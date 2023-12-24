use std::{collections::HashSet, ops::RangeInclusive};

type Stone = (V3, V3);
type V3 = (f64, f64, f64);

#[aoc_generator(day24)]
fn generator(input: &str) -> Vec<Stone> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let mut pos = pos.split(',');
            let mut vel = vel.split(',');
            (
                (
                    pos.next().unwrap().trim().parse().unwrap(),
                    pos.next().unwrap().trim().parse().unwrap(),
                    pos.next().unwrap().trim().parse().unwrap(),
                ),
                (
                    vel.next().unwrap().trim().parse().unwrap(),
                    vel.next().unwrap().trim().parse().unwrap(),
                    vel.next().unwrap().trim().parse().unwrap(),
                ),
            )
        })
        .collect()
}

#[aoc(day24, part1)]
fn part1(stones: &[Stone]) -> usize {
    solve_part1(stones, 200000000000000..=400000000000000)
}

fn solve_part1(stones: &[Stone], area: RangeInclusive<usize>) -> usize {
    let intersect = |(a, b): &(&Stone, &Stone)| {
        let (a_p, a_v) = a;
        let (b_p, b_v) = b;

        if a_v.0 * b_v.1 == a_v.1 * b_v.0 {
            // parallel
            return false;
        }

        let num = b_p.1 - a_p.1 + (a_p.0 - b_p.0) / b_v.0 * b_v.1;
        let den = a_v.1 - a_v.0 * b_v.1 / b_v.0;
        let t = num / den;
        let s = (a_p.0 - b_p.0 + t * a_v.0) / b_v.0;

        if t < 0. || s < 0. {
            // in the past
            return false;
        }

        let x = a_p.0 + t * a_v.0;
        let y = a_p.1 + t * a_v.1;

        if x < *area.start() as f64
            || x > *area.end() as f64
            || y < *area.start() as f64
            || y > *area.end() as f64
        {
            return false;
        }

        true
    };

    stones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| [a].into_iter().cycle().zip(stones[i + 1..].iter()))
        .filter(intersect)
        .count()
}

#[aoc(day24, part2)]
fn part2(stones: &[Stone]) -> i64 {
    let mut x_velocity: HashSet<_> = (-1000..1000).collect();
    let mut y_velocity: HashSet<_> = (-1000..1000).collect();
    let mut z_velocity: HashSet<_> = (-1000..1000).collect();

    for ((a_p, a_v), (b_p, b_v)) in stones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| [a].into_iter().cycle().zip(stones[i + 1..].iter()))
    {
        if a_v.0 == b_v.0 {
            let n = (b_p.0 - a_p.0).abs() as i64;
            let v = a_v.0 as i64;
            x_velocity.retain(|&x| x != v && n % (x - v) == 0)
        }
        if a_v.1 == b_v.1 {
            let n = (b_p.1 - a_p.1).abs() as i64;
            let v = a_v.1 as i64;
            y_velocity.retain(|&y| y != v && n % (y - v) == 0)
        }
        if a_v.2 == b_v.2 {
            let n = (b_p.2 - a_p.2).abs() as i64;
            let v = a_v.2 as i64;
            z_velocity.retain(|&z| z != v && n % (z - v) == 0)
        }
    }

    let v = (
        *x_velocity.iter().next().unwrap() as f64,
        *y_velocity.iter().next().unwrap() as f64,
        *z_velocity.iter().next().unwrap() as f64,
    );

    let (a_p, a_v) = &stones[0];
    let (b_p, b_v) = &stones[1];

    let a_m = (a_v.1 - v.1) / (a_v.0 - v.0);
    let b_m = (b_v.1 - v.1) / (b_v.0 - v.0);

    let a_q = a_p.1 - (a_m * a_p.0);
    let b_q = b_p.1 - (b_m * b_p.0);

    let pos_x = (b_q - a_q) / (a_m - b_m);
    let pos_y = a_m * pos_x + a_q;
    let time = (pos_x - a_p.0) / (a_v.0 - v.0);

    let pos_z = a_p.2 + (a_v.2 - v.2) * time;

    (pos_x + pos_y + pos_z) as _
}

#[test]
fn test_part1() {
    let input = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
    assert_eq!(solve_part1(&generator(input), 7..=21), 2);
}
