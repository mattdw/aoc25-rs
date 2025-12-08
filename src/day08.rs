use std::{collections::HashSet, fmt::Debug};

use crate::Day;

pub struct Day8 {}

impl Day for Day8 {
    fn part1(input: &str) -> impl crate::Answer {
        solve1(input, 1000)
    }

    fn part2(input: &str) -> impl crate::Answer {
        solve1(input, -1)
    }
}

fn solve1(input: &str, rounds: isize) -> isize {
    let vs = parse(input);
    let mut pair_dists = vs
        .iter()
        .enumerate()
        .flat_map(|(idx, v)| {
            vs[idx + 1..].iter().cloned().map(|w| {
                let dist = v.dist_sq(&w);
                (dist, v.clone(), w)
            })
        })
        .filter(|(d, _a, _b)| *d > 0)
        .collect::<Vec<(Int, V3, V3)>>();

    pair_dists.sort_by_cached_key(|v| v.0);
    let mut circuits: Vec<HashSet<V3>> = vec![];

    for (round, (_, a, b)) in pair_dists.into_iter().enumerate() {
        if rounds - (round as isize) == 0 {
            break;
        }

        let a_loc = circuits.iter().position(|s| s.contains(&a));
        let b_loc = circuits.iter().position(|s| s.contains(&b));

        if let Some(a_loc) = a_loc
            && let Some(b_loc) = b_loc
        {
            if a_loc != b_loc {
                // currently in different circuits
                // copy b into a and clear b
                circuits[a_loc] = circuits[a_loc].union(&circuits[b_loc]).cloned().collect();
                circuits.remove(b_loc);
            }
        } else if let Some(a_loc) = a_loc {
            circuits[a_loc].insert(b.clone());
        } else if let Some(b_loc) = b_loc {
            circuits[b_loc].insert(a.clone());
        } else {
            let new = HashSet::from([a.clone(), b.clone()]);
            circuits.push(new);
        }

        // exit condition - everything is connected
        if circuits.len() == 1 && circuits[0].len() == vs.len() {
            return a.x * b.x;
        }
    }

    // find our 3 biggest circuits and * them
    let mut circuit_sizes: Vec<isize> = circuits.into_iter().map(|c| -(c.len() as isize)).collect();
    circuit_sizes.sort();

    -circuit_sizes.into_iter().take(3).product::<isize>()
}

type Int = isize;
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct V3 {
    x: Int,
    y: Int,
    z: Int,
}

impl Debug for V3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V_{},{},{}", self.x, self.y, self.z)
    }
}

impl From<(Int, Int, Int)> for V3 {
    fn from((x, y, z): (Int, Int, Int)) -> V3 {
        V3 { x, y, z }
    }
}

impl V3 {
    fn dist_sq(&self, other: &Self) -> Int {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

fn parse(input: &str) -> Vec<V3> {
    input
        .trim()
        .lines()
        .map(|l| {
            let nums = l
                .trim()
                .split(",")
                .map(|el| el.parse::<Int>().expect("bad num"))
                .collect::<Vec<Int>>();

            V3::from((nums[0], nums[1], nums[2]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
        ";

    #[test]
    fn tparse() {
        let r = parse(TEST_INPUT);
        assert_eq!(r.len(), 20);
        assert_eq!(*r.last().unwrap(), V3::from((425, 690, 689)));
    }

    #[test]
    fn tsolve1() {
        assert_eq!(solve1(TEST_INPUT, 10), 40);
    }

    #[test]
    fn tsolve2() {
        assert_eq!(solve1(TEST_INPUT, -1), 25272);
    }
}
