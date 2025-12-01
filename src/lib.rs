use reqwest::header::COOKIE;
use std::{env, fmt::Display};

mod intmap;
mod iterext;

mod day01;
pub use day01::Day1;

pub trait Checkable: Eq + Display {}
impl<T: Eq + Display> Checkable for T {}

pub trait Day {
    fn part1(input: &str) -> impl Checkable;
    fn part2(input: &str) -> impl Checkable;
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
