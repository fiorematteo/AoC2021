use std::ops::Range;

type Map = Vec<(Range<usize>, i64)>;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> (Vec<usize>, Vec<Map>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<usize> = seeds
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let maps: Vec<Map> = maps
        .split("\n\n")
        .map(|map| {
            let mut lines = map.lines();
            lines.next();
            lines
                .map(|line| {
                    let nums: Vec<usize> = line
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    let to = nums[0];
                    let from = nums[1];
                    let range_len = nums[2];
                    (from..from + range_len, (to as i64 - from as i64))
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

#[aoc(day5, part1)]
pub fn part1((seeds, maps): &(Vec<usize>, Vec<Map>)) -> usize {
    let mut lowest = usize::MAX;
    for seed in seeds {
        let mut current_seed = *seed;
        for map in maps {
            for (range, diff) in map {
                if range.contains(&current_seed) {
                    current_seed = (current_seed as i64 + diff).try_into().unwrap();
                    break;
                }
            }
        }
        lowest = lowest.min(current_seed);
    }
    lowest
}

#[aoc(day5, part2)]
pub fn part2((seeds, maps): &(Vec<usize>, Vec<Map>)) -> usize {
    let seeds: Vec<_> = seeds.chunks(2).map(|t| t[0]..t[0] + t[1]).collect();
    let mut lowest = usize::MAX;
    for seed_range in seeds {
        for seed in seed_range {
            let mut current_seed = seed;
            for map in maps {
                for (range, diff) in map {
                    if range.contains(&current_seed) {
                        current_seed = (current_seed as i64 + diff).try_into().unwrap();
                        break;
                    }
                }
            }
            lowest = lowest.min(current_seed);
        }
    }
    lowest
}

#[test]
fn test_part1() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(part1(&generator(input)), 35);
}

#[test]
fn test_part2() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(part2(&generator(input)), 46);
}
