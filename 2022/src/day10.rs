#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut value = vec![];
    for line in input.lines() {
        if line.starts_with("addx") {
            let (_, n) = line.split_once(" ").unwrap();
            let n = n.parse::<i32>().unwrap();
            value.push(0);
            value.push(n);
        } else {
            value.push(0);
        }
    }

    let mut x = 1;
    let mut total = 0;
    for (cycle, v) in value.iter().enumerate() {
        let cycle = cycle + 1;
        if (cycle + 20) % 40 == 0 {
            total += x * cycle as i32;
        }
        x += v;
    }
    total
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i32 {
    let mut value = vec![];
    for line in input.lines() {
        if line.starts_with("addx") {
            let (_, n) = line.split_once(" ").unwrap();
            let n = n.parse::<i32>().unwrap();
            value.push(0);
            value.push(n);
        } else {
            value.push(0);
        }
    }

    let mut x = 1;
    for (cycle, &v) in value.iter().enumerate() {
        if cycle % 40 == 0 {
            println!();
        }
        let c = cycle as i32 % 40;
        print!("{}", if x - 1 <= c && c <= x + 1 { "#" } else { " " });
        x += v;
    }
    println!();
    0
}
