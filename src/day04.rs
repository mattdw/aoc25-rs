use crate::{
    Day,
    intmap::{Coord, IntMap},
};

pub struct Day4 {}

impl Day for Day4 {
    fn part1(input: &str) -> impl crate::Answer {
        let m = parse(input);
        m.iter()
            .map(|co| {
                let Some(c) = m.get(co) else {
                    return 0;
                };
                if *c == Cell::Empty {
                    return 0;
                }

                if count_adjacent_paper(&m, co) < 4 {
                    1
                } else {
                    0
                }
            })
            .sum::<isize>()
    }

    fn part2(input: &str) -> impl crate::Answer {
        let mut m = parse(input);
        let mut removed = 0;
        let mut removed_this_round = 0;

        let cos: Vec<_> = m.iter().collect();

        loop {
            for &co in cos.iter() {
                if let Some(Cell::Paper) = m.get(co)
                    && count_adjacent_paper(&m, co) < 4
                {
                    removed_this_round += 1;
                    m.set(co, Cell::Empty);
                }
            }

            if removed_this_round == 0 {
                break;
            } else {
                removed += removed_this_round;
                removed_this_round = 0;
            }
        }
        removed
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
enum Cell {
    #[default]
    Empty,
    Paper,
}

fn parse(input: &str) -> IntMap<Cell> {
    IntMap::from_raw(input, |c| match c {
        '.' => Cell::Empty,
        '@' => Cell::Paper,
        _ => panic!("bad cell"),
    })
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn adjacent_cos(co: (isize, isize)) -> Vec<(isize, isize)> {
    NEIGHBOURS
        .iter()
        .map(|n| (Coord::from(*n) + Coord::from(co)).into())
        .collect()
}

fn count_adjacent_paper(m: &IntMap<Cell>, co: (isize, isize)) -> isize {
    adjacent_cos(co)
        .iter()
        .map(|co| {
            if m.get(*co) == Some(&Cell::Paper) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        ";

    #[test]
    fn tparse() {
        let m = parse(TEST_INPUT);
        assert_eq!(m.get((0, 0)), Some(&Cell::Empty));
        assert_eq!(m.get((0, 1)), Some(&Cell::Paper));
        assert_eq!(m.get((0, 9)), Some(&Cell::Paper));
    }

    #[test]
    fn tadjacent() {
        assert_eq!(
            adjacent_cos((4, 0)),
            vec![
                (3, -1),
                (3, 0),
                (3, 1),
                (4, -1),
                (4, 1),
                (5, -1),
                (5, 0),
                (5, 1)
            ]
        );
    }

    #[test]
    fn tp1() {
        assert_eq!(Day4::part1(TEST_INPUT).to_string(), 13.to_string());
    }

    #[test]
    fn tp2() {
        assert_eq!(Day4::part2(TEST_INPUT).to_string(), 43.to_string());
    }
}
