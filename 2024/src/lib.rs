extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub use day01 as day1;
pub use day02 as day2;
pub use day03 as day3;
pub use day04 as day4;
pub use day05 as day5;
pub use day06 as day6;

aoc_lib! { year = 2024 }
