use std::{collections::HashSet, ops::RangeInclusive};

use crate::{Answer, Day};

pub struct Day2 {}

impl Day for Day2 {
    fn part1(input: &str) -> impl Answer {
        let ranges = parse(input);
        ranges
            .into_iter()
            .flat_map(|r| {
                let mut h = HashSet::<u64>::new();
                n_repeats(&r, 2, &mut h);
                h
            })
            .sum::<u64>()
    }

    fn part2(input: &str) -> impl Answer {
        let ranges = parse(input);
        ranges.into_iter().flat_map(any_repeats).sum::<u64>()
    }
}

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(",")
        .filter_map(|pair| -> Option<RangeInclusive<u64>> {
            if let Some((l, r)) = pair.trim().split_once("-")
                && let Ok(l) = l.parse()
                && let Ok(r) = r.parse()
            {
                return Some(l..=r);
            }

            None
        })
        .collect()
}

fn count_digits(i: u64) -> u64 {
    let mut n = 1;
    let mut i = i;
    while i >= 100000000 {
        n += 8;
        i /= 100000000;
    }
    if i >= 10000 {
        n += 4;
        i /= 10000;
    }
    if i >= 100 {
        n += 2;
        i /= 100;
    }
    if i >= 10 {
        n += 1;
    }

    n
}

fn to_next_digit_count(n: u64) -> u64 {
    let d = count_digits(n);
    10u64.pow(d as u32)
}

fn n_repeats(r: &RangeInclusive<u64>, repeats: u64, outs: &mut HashSet<u64>) {
    let r1: u32 = repeats as u32 - 1;
    let end = *r.end();
    let mut check = *r.start();

    while check <= end {
        let d = count_digits(check);

        if !d.is_multiple_of(repeats) {
            // not possible with this number of digits, jump ahead to the next
            // power of 10
            check = to_next_digit_count(check);
            continue;
        }

        // factor is the 10^ size of our repeated segment
        let factor = 10u64.pow((d / repeats) as u32);
        // and base is the repeated segment we're looking for
        let base = check / factor.pow(r1);

        // so if we have 123456 and repeats = 3
        // base = 12 and factor = 10^2

        // now we repeatedly *factor+base, leaving us with e.g. 121212
        // we preemptively compute our next check at the same time
        let mut result = base;
        let base1 = base + 1;
        let mut next_result = base1;
        for _ in 0..r1 {
            result = result * factor + base;
            next_result = next_result * factor + base1;
        }

        if result > end {
            break;
        }

        // result is guaranteed a repeating number, just need to check it's big enough
        if result >= check {
            outs.insert(result);
        }

        // Continue with the next candidate
        check = next_result;
    }
}

fn any_repeats(r: RangeInclusive<u64>) -> impl IntoIterator<Item = u64> {
    let mut out = HashSet::<u64>::new();

    let digits = count_digits(*r.end());
    for ds in 2..=digits {
        n_repeats(&r, ds, &mut out);
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    const TEST_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn parsing() {
        let v = parse(TEST_INPUT);
        assert_eq!(v[0], 11..=22);
        assert_eq!(*v.last().unwrap(), 2121212118..=2121212124);
    }

    #[test]
    fn digits() {
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(100), 3);
        assert_eq!(count_digits(1001), 4);
        assert_eq!(count_digits(99999), 5);
    }

    fn run_n_repeats(r: RangeInclusive<u64>, repeats: u64) -> Vec<u64> {
        let mut h = HashSet::<u64>::new();
        n_repeats(&r, repeats, &mut h);

        let mut v: Vec<u64> = h.into_iter().collect();
        v.sort();
        v
    }

    #[test]
    fn repeats() {
        assert_eq!(run_n_repeats(95..=115, 2), vec![99]);
        assert_eq!(run_n_repeats(998..=1012, 2), vec![1010]);
        assert_eq!(run_n_repeats(1188511880..=1188511890, 2), vec![1188511885]);
    }

    #[test]
    fn sum_repeats() {
        assert_eq!(Day2::part1(TEST_INPUT).to_string(), "1227775554");
    }

    fn run_any_repeats(r: RangeInclusive<u64>) -> Vec<u64> {
        let r = any_repeats(r);

        let mut v: Vec<u64> = r.into_iter().collect();
        v.sort();
        v
    }

    #[test]
    fn test_any_repeats() {
        assert_eq!(run_any_repeats(11..=22), vec![11, 22]);
        assert_eq!(run_any_repeats(998..=1012), vec![999, 1010]);

        assert_eq!(Day2::part2(TEST_INPUT).to_string(), "4174379265");
    }

    #[test]
    fn test_n_repeats() {
        let mut outs = HashSet::<u64>::new();
        n_repeats(&(998..=1012), 2, &mut outs);

        assert_eq!(outs.iter().cloned().collect::<Vec<_>>(), vec![1010]);
        outs.clear();

        n_repeats(&(998..=1012), 3, &mut outs);
        assert_eq!(outs.iter().cloned().collect::<Vec<_>>(), vec![999]);
    }

    #[test]
    fn test_to_next_digits() {
        assert_eq!(to_next_digit_count(1), 10);
        assert_eq!(to_next_digit_count(9), 10);
        assert_eq!(to_next_digit_count(10), 100);
        assert_eq!(to_next_digit_count(555), 1000);
    }
}
