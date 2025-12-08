use aoc25_rs::{Day, Day8, fetch_input};

fn main() {
    let input = fetch_input(8).expect("no input");
    dbg!(Day8::part1(&input));
    dbg!(Day8::part2(&input));
}
