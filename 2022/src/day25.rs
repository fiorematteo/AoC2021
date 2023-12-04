use itertools::{EitherOrBoth::*, Itertools};

#[aoc(day25, part1)]
pub fn part1(input: &str) -> String {
    let lines = input.lines().collect_vec();
    let mut total = lines[0].to_string();
    for line in lines.iter().skip(1) {
        total = snafu_sum(&total, line);
    }
    total
}

fn snafu_sum(a: &str, b: &str) -> String {
    let digits = ['=', '-', '0', '1', '2'];
    let index = |c| {
        digits
            .iter()
            .position(|&d| d == c)
            .map(|d| d as i64)
            .unwrap()
    };
    let mut out = String::new();
    let mut carry: i64 = 0;
    for pair in a.chars().rev().zip_longest(b.chars().rev()) {
        let (a, b) = match pair {
            Both(a, b) => (a, b),
            Left(a) => (a, '0'),
            Right(b) => ('0', b),
        };
        let a_value = index(a) - 2;
        let b_value = index(b) - 2;
        let mut sum: i64 = a_value + b_value + carry;
        if sum < -2 {
            carry = -1;
            sum += 5;
        } else if sum > 2 {
            carry = 1;
            sum -= 5;
        } else {
            carry = 0;
        }
        out.push(digits[(sum + 2) as usize]);
    }
    if carry != 0 {
        out.push(digits[(carry + 2) as usize]);
    }
    out.chars().rev().collect()
}

#[test]
fn test_part1() {
    let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;
    assert_eq!(part1(input), "2=-1=0");

    assert_eq!(snafu_sum("1=", "1="), "11");
    assert_eq!(snafu_sum("2=", "1="), "21");
    assert_eq!(snafu_sum("2=", "2="), "1=1");
}
