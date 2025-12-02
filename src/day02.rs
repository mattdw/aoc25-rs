use std::{collections::HashSet, ops::RangeInclusive};

use crate::{Answer, Day};

pub struct Day2 {}

impl Day for Day2 {
    fn part1(input: &str) -> impl Answer {
        let ranges = parse(input);
        ranges
            .into_iter()
            .flat_map(|r| collect_repeats(r, next_2_repeat))
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

fn next_2_repeat(n: u64) -> u64 {
    let digits = count_digits(n);

    let (start, shift) = if digits.is_multiple_of(2) {
        let fac = 10u64.pow(digits as u32 / 2);
        (n / fac, fac)
    } else {
        return next_2_repeat(10u64.pow(digits as u32));
    };

    let mut attempt = start * shift + start;
    // println!("{} => {} / {} / {} = {}", n, digits, start, shift, attempt);
    while attempt < n {
        attempt = next_2_repeat((start + 1) * shift + (start + 1));
    }
    attempt
}

fn collect_repeats<T: Fn(u64) -> u64>(r: RangeInclusive<u64>, strategy: T) -> Vec<u64> {
    let mut rs = vec![];

    // println!("searching {:?}", r);

    let mut n = *r.start();
    loop {
        n = strategy(n);
        if r.contains(&n) {
            rs.push(n);
        } else {
            break;
        }
        println!("{}", n);
        n += 1;
    }

    rs
}

/*
 *  Abandoned attempt that nearly worked - jumping straight to the next
 *  viable solution rather than searching. (It worked above for the 2-case.)
*/

// fn next_n_repeat(start: u64, r: &RangeInclusive<u64>, repeats: u64) -> Option<u64> {
//     if !r.contains(&start) {
//         return None;
//     }

//     let fac = (10 as u64).pow((count_digits(start) / repeats) as u32);
//     let mut n = start;
//     for _ in 0..(repeats - 1) {
//         n /= fac;
//     }
//     dbg!(n, repeats);

//     let mut res = n;
//     for _ in 0..(repeats - 1) {
//         res *= fac;
//         res += n;
//     }
//     dbg!(res);

//     if res < start {
//         let next_start = {
//             let mut s = n + 1;
//             for _ in 0..(repeats - 1) {
//                 s *= fac;
//             }
//             s
//         };
//         dbg!(next_start);
//         return next_n_repeat(next_start, r, repeats);
//     }

//     if r.contains(&res) {
//         return Some(res);
//     }

//     None
// }

fn n_repeats(r: &RangeInclusive<u64>, repeats: u64) -> Vec<u64> {
    let mut outs: Vec<u64> = vec![];
    let r1: u32 = repeats as u32 - 1;
    for i in r.clone() {
        let d = count_digits(i);
        if !d.is_multiple_of(repeats) {
            continue;
        }
        let fac = 10u64.pow((d / repeats) as u32);
        let base = i / fac.pow(r1);

        // quick check before full
        if i % base != 0 {
            continue;
        }

        let mut res = base;
        for _ in 0..r1 {
            res = res * fac + base
        }
        if res == i {
            outs.push(i);
        }
    }

    // dbg!(outs)
    outs
}

fn any_repeats(r: RangeInclusive<u64>) -> Vec<u64> {
    let mut out = HashSet::<u64>::new();

    let digits = count_digits(*r.end());
    for ds in 2..=digits {
        let rs = n_repeats(&r, ds);
        for n in rs {
            out.insert(n);
        }
    }

    let mut r: Vec<u64> = out.into_iter().collect();
    r.sort();
    // dbg!(r)
    r
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
    fn nexts() {
        assert_eq!(next_2_repeat(11), 11);
        assert_eq!(next_2_repeat(12), 22);
    }

    #[test]
    fn digits() {
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(100), 3);
        assert_eq!(count_digits(1001), 4);
        assert_eq!(count_digits(99999), 5);
    }

    #[test]
    fn repeats() {
        assert_eq!(collect_repeats(95..=115, next_2_repeat), vec![99]);
        assert_eq!(collect_repeats(998..=1012, next_2_repeat), vec![1010]);
        assert_eq!(
            collect_repeats(1188511880..=1188511890, next_2_repeat),
            vec![1188511885]
        );
    }

    #[test]
    fn sum_repeats() {
        assert_eq!(Day2::part1(TEST_INPUT).to_string(), "1227775554");
    }

    #[test]
    fn test_any_repeats() {
        assert_eq!(any_repeats(11..=22), vec![11, 22]);
        assert_eq!(any_repeats(998..=1012), vec![999, 1010]);

        assert_eq!(Day2::part2(TEST_INPUT).to_string(), "4174379265");
    }

    #[test]
    fn test_n_repeats() {
        assert_eq!(n_repeats(&(998..=1012), 2), vec![1010]);
        assert_eq!(n_repeats(&(998..=1012), 3), vec![999]);
    }
}
