use crate::{Checkable, Day};

pub struct Day1 {}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left(isize),
    Right(isize),
}

impl Day for Day1 {
    fn part1(input: &str) -> impl Checkable {
        let steps = parse(input);
        let mut curr = 50;
        let mut zeroes = 0;

        for dir in steps {
            match dir {
                Direction::Left(c) => curr -= c,
                Direction::Right(c) => curr += c,
            }
            if (curr % 100) == 0 {
                zeroes += 1
            }
        }

        zeroes
    }

    fn part2(input: &str) -> impl Checkable {
        let steps = parse(input);
        let mut curr = 50;
        let mut zeroes = 0;

        for dir in steps {
            let (mut c, by) = match dir {
                Direction::Left(c) => (-c, -1),
                Direction::Right(c) => (c, 1),
            };

            while c != 0 {
                curr += by;
                c -= by;

                if curr % 100 == 0 {
                    zeroes += 1
                }
            }
        }

        zeroes
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.split_at(1);
            let r: isize = isize::from_str_radix(r, 10).expect("bad num");
            match l {
                "L" => Direction::Left(r),
                "R" => Direction::Right(r),
                _ => panic!("bad direction"),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p() {
        let steps = parse("L2\nR82\nL10");
        assert_eq!(
            steps,
            vec![
                Direction::Left(2),
                Direction::Right(82),
                Direction::Left(10)
            ]
        );
    }
}
