use crate::Day;

type Int = isize;

pub struct Day3 {}

impl Day for Day3 {
    fn part1(input: &str) -> impl crate::Answer {
        let inp = parse(input);
        inp.into_iter()
            .map(|line| {
                let (loc, d1) = max_in_slice(&line[0..line.len() - 1]);
                let (_, d2) = max_in_slice(&line[loc + 1..]);

                d1 * 10 + d2
            })
            .sum::<Int>()
    }

    fn part2(input: &str) -> impl crate::Answer {
        let inp = parse(input);
        inp.into_iter()
            .map(|line| {
                let mut s = 0;
                let mut last_loc = 0;
                for i in (0..12).into_iter().rev() {
                    let (loc, n) = max_in_slice(&line[last_loc..(line.len() - i)]);
                    s = s * 10 + n;
                    last_loc = last_loc + loc + 1;
                }

                s
            })
            .sum::<Int>()
    }
}

fn parse(input: &str) -> Vec<Vec<Int>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .as_bytes()
                .iter()
                .map(|b| (b - 48) as Int)
                .collect()
        })
        .collect()
}

fn max_in_slice(slice: &[Int]) -> (usize, Int) {
    // all Rust's iter::max(_by) let last equal element win;
    // we need the first
    let mut curr = (0, slice[0]);
    for (idx, &val) in slice.into_iter().enumerate() {
        if val > curr.1 {
            curr = (idx, val);
        }
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111";

    #[test]
    fn tparse() {
        let i = parse(TEST_INPUT);
        assert_eq!(i[0][0], 9);
        assert_eq!(i[1][0], 8);
        assert_eq!(*i[3].last().unwrap(), 1);
        assert_eq!(i[3][0..=6], vec![8, 1, 8, 1, 8, 1, 9]);
    }

    #[test]
    fn maxmax() {
        let max1 = max_in_slice(&[1, 2, 3, 4, 3, 2, 1]);
        assert_eq!(max1, (3, 4));

        let max2 = max_in_slice(&[1, 2, 3, 4, 3, 2, 1][max1.0 + 1..]);
        assert_eq!(max2, (0, 3));
    }

    #[test]
    fn p1() {
        assert_eq!(Day3::part1(TEST_INPUT).to_string(), "357");
    }

    #[test]
    fn p2() {
        assert_eq!(Day3::part2(TEST_INPUT).to_string(), "3121910778619");
    }
}
