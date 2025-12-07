use std::collections::{HashMap, HashSet};

use crate::{Day, intmap::IntMap};

pub struct Day7 {}

impl Day for Day7 {
    fn part1(input: &str) -> impl crate::Answer {
        let m = parse(input);
        let mut actives = active(&m, 0);
        let mut splits = 0;
        for row in 0..m.height as isize {
            for col in 0..m.width as isize {
                let c = *m.get((col, row)).unwrap();
                if c == Cell::Splitter && actives.contains(&col) {
                    splits += 1;
                    actives.remove(&col);
                    if m.in_bounds((col + 1, row)) {
                        actives.insert(col + 1);
                    }
                    if m.in_bounds((col - 1, row)) {
                        actives.insert(col - 1);
                    }
                }
            }
        }

        splits
    }

    fn part2(input: &str) -> impl crate::Answer {
        let m = parse(input);
        let mut actives: HashMap<isize, isize> = active(&m, 0).iter().map(|a| (*a, 1)).collect();
        // sweep rows

        let rows = 0..m.height as isize;
        let cols = 0..m.width as isize;
        for (col, row) in rows.flat_map(|y| cols.clone().map(move |x| (x, y))) {
            // Beams pass through Beams and Empties so only need to worry about
            // splitters
            let c = *m.get((col, row)).unwrap();
            let col_active = *actives.get(&col).unwrap_or(&0);
            if c != Cell::Splitter || col_active == 0 {
                continue;
            }

            actives.remove(&col);
            for offset in [col + 1, col - 1] {
                if !m.in_bounds((offset, row)) {
                    continue;
                }

                let existing_beams = actives.get_mut(&offset);
                if let Some(v) = existing_beams {
                    *v += col_active;
                } else {
                    actives.insert(offset, col_active);
                }
                // actives.insert(col + 1);
            }
        }

        actives.values().sum::<isize>()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
enum Cell {
    #[default]
    Empty,
    Splitter,
    Beam,
}

fn parse(input: &str) -> IntMap<Cell> {
    IntMap::from_raw(input, |c| match c {
        '.' => Cell::Empty,
        '^' => Cell::Splitter,
        '|' | 'S' => Cell::Beam,
        _ => {
            eprintln!("bad char in input {}", c);
            Cell::Empty
        }
    })
}

fn active(m: &IntMap<Cell>, row: isize) -> HashSet<isize> {
    (0..(m.width as isize))
        .map(|x| m.get((x, row)))
        .enumerate()
        .filter(|(_, c)| *c == Some(&Cell::Beam))
        .map(|(i, _)| i as isize)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &'static str = "
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
    ";

    #[test]
    fn tparse() {
        let m = parse(TEST_INPUT);
        dbg!(&m);
        assert_eq!(m.get((7, 0)), Some(&Cell::Beam));
    }

    #[test]
    fn t1() {
        let r = Day7::part1(TEST_INPUT);
        assert_eq!(r.to_string(), "21");
    }

    #[test]
    fn t2() {
        let r = Day7::part2(TEST_INPUT);
        assert_eq!(r.to_string(), "40");
    }
}
