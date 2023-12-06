#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let (time, record) = input.split_once('\n').unwrap();
    let zipped = time.split_whitespace().zip(record.split_whitespace());

    let mut out = 1;
    for (max_time, record) in zipped
        .skip(1)
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
    {
        let mut wins = 0;
        for k in 0..=max_time {
            if k * (max_time - k) > record {
                wins += 1;
            }
        }
        out *= wins;
    }
    out
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let (time, record) = input.split_once('\n').unwrap();
    let time = time
        .split_once(' ')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let record = record
        .split_once(' ')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    let mut wins = 0;
    for k in 0..=time {
        if k * (time - k) > record {
            wins += 1;
        }
    }
    wins
}
