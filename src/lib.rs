use reqwest::header::COOKIE;
use std::{env, fmt::Display};

mod intmap;
mod iterext;

mod day01;
pub use day01::Day1;

mod day02;
pub use day02::Day2;

mod day03;
pub use day03::Day3;

mod day04;
pub use day04::Day4;

// mod day05;
// pub use day05::Day5;

// mod day06;
// pub use day06::Day6;

// mod day07;
// pub use day07::Day7;

// mod day08;
// pub use day08::Day8;

// mod day09;
// pub use day09::Day9;

// mod day10;
// pub use day10::Day10;

// mod day11;
// pub use day11::Day11;

// mod day12;
// pub use day12::Day12;

pub trait Answer: Eq + Display + std::fmt::Debug {}
impl<T: Eq + Display + std::fmt::Debug> Answer for T {}

pub trait Day {
    fn part1(input: &str) -> impl Answer;
    fn part2(input: &str) -> impl Answer;
}

pub fn fetch_input(day: u8) -> Result<String, anyhow::Error> {
    let _ = std::fs::create_dir("inputs");
    let existing = std::fs::read_to_string(format!("inputs/{day}.txt"));

    if let Ok(s) = existing {
        return Ok(s);
    }

    let session = env::var("SESSION").expect("SESSION env var is required to fetch input");
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(format!("https://adventofcode.com/2025/day/{day}/input"))
        .header(COOKIE, format!("session={session}"))
        .send()?;

    let text: String = resp.text()?;
    std::fs::write(format!("inputs/{day}.txt"), &text)?;

    Ok(text)
}

pub fn fetch_input_s(day: &str) -> Result<String, anyhow::Error> {
    let s: u8 = day
        .trim_matches(['d', 'a', 'y', 'D'])
        .parse()
        .expect("ends with a number");
    fetch_input(s)
}
