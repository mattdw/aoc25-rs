use core::slice::GetDisjointMutIndex;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

use crate::Day;

type Int = usize;

pub struct Day5 {}

impl Day for Day5 {
    fn part1(input: &str) -> impl crate::Answer {
        let db = parse(input);

        db.ingredients
            .into_iter()
            .filter(|i| db.fresh_ranges.iter().any(|r| r.contains(i)))
            .count()
    }

    fn part2(input: &str) -> impl crate::Answer {
        let db = parse(input);
        let mut merged_ranges: Vec<RangeInclusive<Int>> = vec![];

        'outer: for r in db.fresh_ranges {
            for existing in merged_ranges.iter_mut() {
                if r.is_overlapping(existing) {
                    let l = min(*r.start(), *existing.start());
                    let h = max(*r.end(), *existing.end());
                    *existing = l..=h;
                    continue 'outer;
                }
            }
            merged_ranges.push(r);
        }

        merged_ranges
            .iter()
            .map(|r| (*r.end() - *r.start()) + 1)
            .sum::<usize>()
    }
}

struct DB {
    pub fresh_ranges: Vec<RangeInclusive<Int>>,
    pub ingredients: Vec<Int>,
}

fn parse_range(input: &str) -> RangeInclusive<Int> {
    let (a, b) = input.trim().split_once("-").expect("need a -");
    if a <= b {
        a.parse().expect("left is num")..=b.parse().expect("right is num")
    } else {
        b.parse().expect("left is num")..=a.parse().expect("right is num")
    }
}

fn parse(input: &str) -> DB {
    let (fresh, ingred) = input
        .trim()
        .split_once("\n\n")
        .expect("found more than one double nl");

    let mut fresh_ranges: Vec<RangeInclusive<Int>> =
        fresh.trim().lines().map(parse_range).collect();
    let ingredients: Vec<Int> = ingred
        .trim()
        .lines()
        .map(|l| l.trim().parse().expect("should be num"))
        .collect();

    fresh_ranges.sort_by_key(|r| *r.start());

    DB {
        fresh_ranges,
        ingredients,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

    #[test]
    fn tparse() {
        let d = parse(TEST_INPUT);

        assert!(d.fresh_ranges.starts_with(&[3..=5, 10..=14]));
        assert!(d.fresh_ranges.ends_with(&[16..=20]));

        assert_eq!(d.ingredients, [1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn t1() {
        assert_eq!(Day5::part1(TEST_INPUT).to_string(), "3");
    }

    #[test]
    fn t2() {
        assert_eq!(Day5::part2(TEST_INPUT).to_string(), "14");
    }
}
