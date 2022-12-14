use std::{cmp::Ordering, str::FromStr};

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let (left_str, right_str) = pair.split_once('\n').unwrap();
        let left = left_str.parse::<Message>().unwrap();
        let right = right_str.parse::<Message>().unwrap();
        if left < right {
            sum += i as i32 + 1;
        }
    }
    sum
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.replace("\n\n", "\n");
    let mut messages = input
        .split('\n')
        .map(|s| s.parse::<Message>().unwrap())
        .collect::<Vec<_>>();
    let divider = |n| Message::List(vec![Message::List(vec![Message::Number(n)])]);
    let first_divider = divider(2);
    let second_divider = divider(6);
    messages.push(first_divider.clone());
    messages.push(second_divider.clone());
    messages.sort();
    let dividers = messages
        .iter()
        .enumerate()
        .filter(|(_, m)| **m == first_divider || **m == second_divider)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    dividers[0] * dividers[1]
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    List(Vec<Message>),
    Number(i32),
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Message::List(s), Message::List(o)) => {
                if s.is_empty() && o.is_empty() {
                    return Some(Ordering::Equal);
                }
                let mut i = 0;
                loop {
                    if i >= s.len() && i >= o.len() {
                        return Some(Ordering::Equal);
                    }
                    if i >= s.len() {
                        return Some(Ordering::Less);
                    }
                    if i >= o.len() {
                        return Some(Ordering::Greater);
                    }
                    if s[i] < o[i] {
                        return Some(Ordering::Less);
                    }
                    if s[i] > o[i] {
                        return Some(Ordering::Greater);
                    }
                    i += 1;
                }
            }
            (Message::List(_), Message::Number(_)) => {
                self.partial_cmp(&Message::List(vec![other.clone()]))
            }
            (Message::Number(_), Message::List(_)) => {
                Self::List(vec![self.clone()]).partial_cmp(other)
            }
            (Message::Number(s), Message::Number(o)) => s.partial_cmp(o),
        }
    }
}

impl FromStr for Message {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Message::Number(n));
        }

        if s == "[]" {
            return Ok(Message::List(Vec::new()));
        }

        let s = &s[1..s.len() - 1];

        let mut comma_indexes = Vec::new();
        let mut depht = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => depht += 1,
                ']' => depht -= 1,
                ',' => {
                    if depht == 0 {
                        comma_indexes.push(i)
                    }
                }
                _ => (),
            }
        }

        let mut start = 0;
        let mut messages = Vec::new();
        for comma in comma_indexes {
            messages.push(s[start..comma].parse::<Message>()?);
            start = comma + 1;
        }
        messages.push(s[start..].parse::<Message>()?);

        Ok(Message::List(messages))
    }
}
