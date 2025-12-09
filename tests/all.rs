macro_rules! check {
    ($name:ident, $struct:ty, $p1:literal, $p2:literal) => {
        mod $name {

            use aoc25_rs::*;

            #[test]
            fn part1() {
                let input = fetch_input_s(stringify!($struct)).unwrap();
                let res = <$struct>::part1(&input);
                assert_eq!($p1.to_string(), res.to_string());
            }

            #[test]
            fn part2() {
                let input = fetch_input_s(stringify!($struct)).unwrap();
                let res = <$struct>::part2(&input);
                assert_eq!($p2.to_string(), res.to_string());
            }
        }
    };
}

check!(day01, Day1, 1177, 6768);
check!(day02, Day2, 12850231731u64, 24774350322u64);
check!(day03, Day3, 17554, 175053592950232u64);
check!(day04, Day4, 1428, 8936);
check!(day05, Day5, 558, 344813017450467u64);
check!(day06, Day6, 4805473544166u64, 8907730960817u64);
check!(day07, Day7, 1543, 3223365367809u64);
check!(day08, Day8, 26400, 8199963486u64);
check!(day09, Day9, 4777409595u64, 1473551379u64);
// check!(day10, Day10, "👻", "👻");
// check!(day11, Day11, "👻", "👻");
// check!(day12, Day12, "👻", "👻");
