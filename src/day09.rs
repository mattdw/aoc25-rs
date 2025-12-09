use std::cmp::minmax;

use crate::{Day, intmap::Coord};

pub struct Day9 {}

impl Day for Day9 {
    fn part1(input: &str) -> impl crate::Answer {
        let vs = parse(input);
        all_pairs(&vs).map(rect_size).max().unwrap()
    }

    fn part2(input: &str) -> impl crate::Answer {
        let mut vs = parse(input);

        let mut candidates = all_pairs(&vs)
            .map(|p| (rect_size(p), p.0, p.1))
            .collect::<Vec<_>>();

        candidates.sort_by_cached_key(|t| -t.0);

        // make it loop
        vs.push(vs[0]);

        // let mut m = make_map(&vs);
        let segments: Vec<[Coord<Int>; 2]> = vs
            .windows(2)
            .map(|w| [w[0], w[1]] as [Coord<Int>; 2])
            .collect();

        // dbg!(&segments);

        candidates
            .into_iter()
            .filter(|(_, a, b)| valid_rect(&segments, a, b))
            .map(|(d, _, _)| d)
            .take(1)
            .sum::<isize>()
    }
}

type Int = isize;
fn parse(input: &str) -> Vec<Coord<Int>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let nums: Vec<Int> = l
                .trim()
                .split(",")
                .map(|n| n.parse::<Int>().unwrap())
                .collect();

            Coord::from((nums[0], nums[1]))
        })
        .collect()
}

fn all_pairs(vs: &[Coord<Int>]) -> impl Iterator<Item = (Coord<Int>, Coord<Int>)> {
    vs.iter()
        .enumerate()
        .flat_map(|(idx, a)| vs[(idx + 1)..].iter().map(|b| (*a, *b)))
}

fn rect_size(pair: (Coord<Int>, Coord<Int>)) -> Int {
    let (a, b) = pair;
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn valid_rect(lines: &[[Coord<Int>; 2]], a: &Coord<Int>, b: &Coord<Int>) -> bool {
    let [xmin, xmax] = minmax(a.x, b.x);
    let [ymin, ymax] = minmax(a.y, b.y);

    let insidex = xmin + 1..=xmax - 1;
    let insidey = ymin + 1..=ymax - 1;
    let outsidex = xmin..=xmax;
    let outsidey = ymin..=ymax;

    for seg in lines {
        let [l, r] = seg;
        let horz = l.y == r.y;

        let (insideperp, insidepar, outside, shared, lmin, lmax) = if horz {
            let [lmin, lmax] = minmax(l.x, r.x);
            (&insidey, &insidex, &outsidex, l.y, lmin, lmax)
        } else {
            let [lmin, lmax] = minmax(l.y, r.y);
            (&insidex, &insidey, &outsidey, l.x, lmin, lmax)
        };

        if !insideperp.contains(&shared) {
            continue;
        }

        if insidepar.contains(&lmin) || insidepar.contains(&lmax) {
            return false;
        }

        if lmin <= *outside.start() && lmax >= *outside.end() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
        ";

    #[test]
    fn tparse() {
        let r = parse(TEST_INPUT);
        assert_eq!(r.len(), 8);
        assert_eq!(r[0], Coord::from((7, 1)));
        assert_eq!(r[7], Coord::from((7, 3)));
    }

    #[test]
    fn trect() {
        assert_eq!(rect_size((Coord::from((2, 5)), Coord::from((9, 7)))), 24);
    }

    #[test]
    fn p1() {
        assert_eq!(Day9::part1(TEST_INPUT).to_string(), "50");
    }

    #[test]
    fn vrect() {
        assert_eq!(
            valid_rect(
                &[[Coord::from((9, 7)), Coord::from((9, 5))]],
                &Coord::from((7, 1)),
                &Coord::from((11, 7))
            ),
            false
        );
    }

    #[test]
    fn p2() {
        assert_eq!(Day9::part2(TEST_INPUT).to_string(), "24");
    }
}
