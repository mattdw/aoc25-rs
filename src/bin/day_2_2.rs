use aoc25_rs::{Day, Day2, fetch_input};

fn main() {
    let input = fetch_input(2).expect("no input");

    let mut count = 0;
    for _ in 0..1000 {
        count += 1;
        dbg!(Day2::part2(&input));
    }
    dbg!(count);
}
