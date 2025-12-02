use crate::{Answer, Day};

pub struct Day1 {}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left(isize),
    Right(isize),
}

impl Day for Day1 {
    fn part1(input: &str) -> impl Answer {
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

    fn part2(input: &str) -> impl Answer {
        let steps = parse(input);
        let mut curr = 50;
        let mut zeroes = 0;

        for dir in steps {
            let (mut c, by) = match dir {
                Direction::Left(c) => (-c, -1),
                Direction::Right(c) => (c, 1),
            };

            while c.abs() > 100 {
                c -= 100 * by;
                zeroes += 1;
            }

            while c.abs() > 0 {
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
            let (l, r) = line.trim().split_at(1);
            let r: isize = r.parse().expect("bad num");
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

    const TEST_INPUT: &'static str = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

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

    #[test]
    fn p2() {
        let res = Day1::part2(TEST_INPUT);
        assert_eq!(res.to_string(), "6");
    }

    #[test]
    fn andrew_test() {
        let input = std::fs::read_to_string("inputs/AB_1.txt").expect("Missing Input");
        let res = Day1::part2(&input);

        assert_eq!(res.to_string(), "💣");
    }
}
