extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;
pub mod day03;

pub use day01 as day1;
pub use day02 as day2;
pub use day03 as day3;

aoc_lib! { year = 2024 }
