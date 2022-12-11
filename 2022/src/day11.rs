#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut monkeys = vec![];
    for monkey in input.split("\n\n") {
        let mut lines = monkey.lines();
        let _index = lines.next().unwrap();
        let items = lines.next().unwrap();
        let operation = lines.next().unwrap();
        let test = lines.next().unwrap();
        let true_index = lines.next().unwrap();
        let false_index = lines.next().unwrap();

        let items = items
            .get(18..)
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        let test = test.get(21..).unwrap().parse().unwrap();
        let true_index = true_index.get(29..).unwrap().parse().unwrap();
        let false_index = false_index.get(30..).unwrap().parse().unwrap();
        let operation = operation.split_once("= ").unwrap().1.to_owned();

        let m = Monkey {
            count: 0,
            items,
            operation,
            test,
            true_index,
            false_index,
        };
        monkeys.push(m);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let operation = monkeys[i].operation.clone();
            let test = monkeys[i].test;
            let true_index = monkeys[i].true_index;
            let false_index = monkeys[i].false_index;
            let items: Vec<usize> = monkeys[i].items.drain(..).collect();
            for mut item in items {
                let mut iter = operation.split(' ');
                let a = match iter.next().unwrap() {
                    "old" => item,
                    n => n.parse().unwrap(),
                };
                let op = iter.next().unwrap();
                let b = match iter.next().unwrap() {
                    "old" => item,
                    n => n.parse().unwrap(),
                };
                item = match op {
                    "+" => a + b,
                    "*" => a * b,
                    _ => unreachable!(),
                };

                monkeys[i].count += 1;

                item /= 3;

                if item % test == 0 {
                    monkeys[true_index].items.push(item);
                } else {
                    monkeys[false_index].items.push(item);
                }
            }
        }
    }
    let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut monkeys = vec![];
    for monkey in input.split("\n\n") {
        let mut lines = monkey.lines();
        let _index = lines.next().unwrap();
        let items = lines.next().unwrap();
        let operation = lines.next().unwrap();
        let test = lines.next().unwrap();
        let true_index = lines.next().unwrap();
        let false_index = lines.next().unwrap();

        let items = items
            .get(18..)
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();
        let test = test.get(21..).unwrap().parse().unwrap();
        let true_index = true_index.get(29..).unwrap().parse().unwrap();
        let false_index = false_index.get(30..).unwrap().parse().unwrap();
        let operation = operation.split_once("= ").unwrap().1.to_owned();

        let m = Monkey {
            count: 0,
            items,
            operation,
            test,
            true_index,
            false_index,
        };
        monkeys.push(m);
    }

    let mut big = 1;
    for k in monkeys.iter().map(|m| m.test) {
        big *= k;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let operation = monkeys[i].operation.clone();
            let test = monkeys[i].test;
            let true_index = monkeys[i].true_index;
            let false_index = monkeys[i].false_index;
            let items: Vec<usize> = monkeys[i].items.drain(..).collect();
            for mut item in items {
                let mut iter = operation.split(' ');
                let a = match iter.next().unwrap() {
                    "old" => item,
                    n => n.parse().unwrap(),
                };
                let op = iter.next().unwrap();
                let b = match iter.next().unwrap() {
                    "old" => item,
                    n => n.parse().unwrap(),
                };
                item = match op {
                    "+" => a.checked_add(b).unwrap(),
                    "*" => a.checked_mul(b).unwrap(),
                    _ => unreachable!(),
                };

                monkeys[i].count += 1;

                item %= big;

                if item % test == 0 {
                    monkeys[true_index].items.push(item);
                } else {
                    monkeys[false_index].items.push(item);
                }
            }
        }
    }
    let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

#[derive(Debug, Clone)]
struct Monkey {
    count: usize,
    items: Vec<usize>,
    operation: String,
    test: usize,
    true_index: usize,
    false_index: usize,
}
