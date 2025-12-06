use std::{ascii, collections::HashMap, iter::zip};

use crate::Day;

pub struct Day6 {}

impl Day for Day6 {
    fn part1(input: &str) -> impl crate::Answer {
        let eqs = parse(input);
        solve_and_sum(eqs)
    }

    fn part2(input: &str) -> impl crate::Answer {
        let eqs = parse2(input);
        solve_and_sum(eqs)
    }
}

type Int = u64;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mult,
}

fn parse(input: &str) -> Vec<(Op, Vec<Int>)> {
    let mut cols: Vec<Vec<Int>> = vec![];
    let mut ops: Vec<Op> = vec![];
    input.trim().lines().for_each(|l| {
        let l: Vec<&str> = l
            .trim()
            .split_ascii_whitespace()
            .map(|c| c.trim())
            .collect();
        let nums: Vec<Int> = l.iter().map(|c| c.parse::<Int>()).flatten().collect();

        // dbg!(&l, &nums);
        if nums.len() == l.len() {
            for (idx, num) in nums.iter().enumerate() {
                if let Some(v) = cols.get_mut(idx) {
                    v.push(*num)
                } else {
                    cols.push(vec![*num]);
                }
            }
        } else {
            ops = l
                .into_iter()
                .map(|c| match c {
                    "+" => Op::Add,
                    "*" => Op::Mult,
                    _ => panic!("bad op"),
                })
                .collect();
        }
    });

    zip(ops, cols).collect()
}

fn parse2(input: &str) -> Vec<(Op, Vec<Int>)> {
    // ok, whitespace now matters
    let (l, _) = input.split_once("\n").expect("has a linebreak");
    let cols = l.len();
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut eqs: Vec<(Op, Vec<Int>)> = vec![];
    let mut curr_nums: Vec<Int> = vec![];
    let mut curr_num: Int = 0;

    for c in (0..cols).rev() {
        for l in 0..lines.len() {
            let char = lines[l][c];
            match char {
                0x20 => continue,
                0x30..=0x39 => curr_num = curr_num * 10 + (char as Int - 0x30),
                0x2a => {
                    curr_nums.push(curr_num);
                    curr_num = 0;
                    eqs.push((Op::Mult, curr_nums));
                    curr_nums = vec![];
                }
                0x2b => {
                    curr_nums.push(curr_num);
                    curr_num = 0;
                    eqs.push((Op::Add, curr_nums));
                    curr_nums = vec![];
                }
                _ => continue,
            }
            // dbg!(curr_num);
        }
        if curr_num != 0 {
            curr_nums.push(curr_num as Int);
        }
        curr_num = 0;
    }

    eqs
}

fn solve_and_sum(eqs: Vec<(Op, Vec<Int>)>) -> Int {
    eqs.into_iter()
        .map(|(op, nums)| match op {
            Op::Add => nums.into_iter().sum::<Int>(),
            Op::Mult => nums.into_iter().fold(1, |a, b| a * b),
        })
        .sum::<Int>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn tparse() {
        let eqs = parse(TEST_INPUT);
        assert_eq!(eqs.len(), 4);
        assert_eq!(eqs[0].0, Op::Mult);
    }

    #[test]
    fn t1() {
        assert_eq!(Day6::part1(TEST_INPUT).to_string(), "4277556");
    }

    #[test]
    fn tparse2() {
        let eqs = parse2(TEST_INPUT);
        assert_eq!(eqs[0], (Op::Add, vec![4, 431, 623]));
    }

    #[test]
    fn t2() {
        assert_eq!(Day6::part2(TEST_INPUT).to_string(), "3263827");
    }
}
