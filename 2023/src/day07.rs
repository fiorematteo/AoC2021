use std::cmp::Ordering;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    solve(input, cmp)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    solve(input, cmp_jocker)
}

fn solve(input: &str, cmp_fn: fn(&Cards, &Cards) -> Ordering) -> usize {
    let mut game: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').unwrap();
            let cards = Cards(cards.chars().collect::<Vec<_>>().try_into().unwrap());
            let bet = bet.parse::<usize>().unwrap();
            Hand { cards, bet }
        })
        .collect();
    game.sort_by(|hand_1, hand_2| cmp_fn(&hand_1.cards, &hand_2.cards));
    game.into_iter()
        .rev()
        .enumerate()
        .map(|(index, Hand { bet, .. })| bet * (index + 1))
        .sum()
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Cards,
    bet: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Cards([char; 5]);

fn cmp(a: &Cards, b: &Cards) -> Ordering {
    let ord = a.get_value().cmp(&b.get_value());
    if ord == Ordering::Equal {
        for (&a, &b) in a.0.iter().zip(b.0.iter()) {
            let ord = Cards::index_of(a).cmp(&Cards::index_of(b));
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    } else {
        ord
    }
}

fn cmp_jocker(a: &Cards, b: &Cards) -> Ordering {
    let ord = a.get_value_jocker().cmp(&b.get_value_jocker());
    if ord == Ordering::Equal {
        for (&a, &b) in a.0.iter().zip(b.0.iter()) {
            let ord = Cards::index_of_jocker(a).cmp(&Cards::index_of_jocker(b));
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    } else {
        ord
    }
}

impl Cards {
    fn get_value(&self) -> usize {
        let mut counter = [0; 13];
        for &c in self.0.iter() {
            counter[Self::index_of(c)] += 1;
        }
        get_value(&counter)
    }

    fn index_of(c: char) -> usize {
        let arr = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        arr.iter().position(|&x| x == c).unwrap()
    }

    fn get_value_jocker(&self) -> usize {
        let mut counter = [0; 13];
        for &c in self.0.iter() {
            counter[Self::index_of(c)] += 1;
        }

        let jockers = counter[Self::index_of('J')];
        counter[Self::index_of('J')] = 0;
        let mut best_value = usize::MAX;
        for index in 0..counter.len() {
            counter[index] += jockers;
            let value = get_value(&counter);
            best_value = best_value.min(value);
            counter[index] -= jockers;
        }

        best_value
    }

    fn index_of_jocker(c: char) -> usize {
        let arr = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];
        arr.iter().position(|&x| x == c).unwrap()
    }
}

fn get_value(counter: &[usize]) -> usize {
    let n_of_a_kind = |n| counter.iter().any(|&x| x == n);
    if n_of_a_kind(5) {
        0 // 5 of a kind
    } else if n_of_a_kind(4) {
        1 // 4 of a kind
    } else if n_of_a_kind(3) && n_of_a_kind(2) {
        2 // full house
    } else if n_of_a_kind(3) {
        3 // three of a kind
    } else if counter.iter().filter(|&&x| x == 2).count() == 2 {
        4 // two pairs
    } else if n_of_a_kind(2) {
        5 // one pair
    } else {
        6
    }
}

#[test]
fn test_part2() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part2(input), 5905);
}

#[test]
fn test_part1() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part1(input), 6440)
}
