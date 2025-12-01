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
