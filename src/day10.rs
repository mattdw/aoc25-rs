use std::collections::HashSet;

use crate::Day;

pub struct Day10 {}

impl Day for Day10 {
    fn part1(input: &str) -> impl crate::Answer {
        let ms = parse(input);

        ms.iter().map(solve_machine_toggle).sum::<usize>()
    }

    fn part2(input: &str) -> impl crate::Answer {
        return 0;

        let ms = parse(input);

        ms.iter().map(solve_machine_inc).sum::<usize>()
    }
}

fn solve_machine_toggle(m: &Machine) -> usize {
    let mut stacks = vec![MachineState {
        steps: 0,
        indicators: m.indicators.clone(),
    }];

    let mut seen_states = HashSet::new();
    loop {
        let mut new_stacks = vec![];
        for stack in stacks {
            for combo in &m.button_combos {
                if seen_states.contains(&(stack.indicators.clone(), combo)) {
                    continue;
                }
                seen_states.insert((stack.indicators.clone(), combo));
                let mut stack = stack.clone();
                apply_combo_toggle(&mut stack.indicators, combo);
                // stack.indicators ^= combo;
                stack.steps += 1;

                if stack.indicators == m.target_indicators {
                    return dbg!(stack.steps);
                }

                new_stacks.push(stack);
            }
        }

        stacks = new_stacks
    }
}

fn solve_machine_inc(m: &Machine) -> usize {
    let mut stacks = vec![MachineState {
        steps: 0,
        indicators: m.indicators.clone(),
    }];

    let mut seen_states = HashSet::new();
    loop {
        let mut new_stacks = vec![];
        for stack in stacks {
            // dbg!(&stack);
            for combo in &m.button_combos {
                if seen_states.contains(&(stack.indicators.clone(), combo)) {
                    continue;
                }
                seen_states.insert((stack.indicators.clone(), combo));
                let mut stack = stack.clone();
                stack.steps += 1;
                let mut matched = true;
                let mut overflowed = false;
                for &i in combo {
                    stack.indicators[i] += 1;
                    if stack.indicators[i] > m.joltages[i] {
                        overflowed = true;
                        break;
                    }
                }
                if overflowed {
                    continue;
                }
                for i in 0..m.joltages.len() {
                    if stack.indicators[i] != m.joltages[i] {
                        matched = false;
                    }
                    if stack.indicators[i] > m.joltages[i] {
                        overflowed = true;
                        break;
                    }
                }
                if matched {
                    return stack.steps;
                }
                if overflowed {
                    continue;
                }
                new_stacks.push(stack);
            }
        }

        stacks = new_stacks
    }
}

fn apply_combo_toggle(indicators: &mut [usize], indexes: &[usize]) {
    for &i in indexes {
        indicators[i] = (indicators[i] + 1) % 2;
    }
}

fn apply_combo_inc(indicators: &mut [usize], indexes: &[usize]) {
    for &i in indexes {
        indicators[i] += 1;
    }
}

///- Types and utils

#[derive(Debug, Clone)]
struct Machine {
    target_indicators: Vec<usize>,
    indicators: Vec<usize>,
    button_combos: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

#[derive(Debug, Clone)]
struct MachineState {
    steps: usize,
    indicators: Vec<usize>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|l| {
            let pieces = l.trim().split(" ");
            let mut target_indicators = vec![];
            let mut button_combos = vec![];
            let mut joltages = vec![];

            for piece in pieces {
                match &piece.as_bytes()[0..1] {
                    b"[" => {
                        target_indicators = parse_targets(piece);
                    }
                    b"(" => {
                        button_combos.push(parse_num_list(&['(', ')'], piece));
                    }
                    b"{" => {
                        joltages = parse_num_list(&['{', '}'], piece)
                            .iter()
                            .map(|j| *j as usize)
                            .collect();
                    }
                    _ => {
                        dbg!(piece);
                        panic!("unexpected input")
                    }
                }
            }

            let mut indicators = vec![];
            indicators.resize(target_indicators.len(), 0);
            Machine {
                target_indicators,
                indicators,
                button_combos,
                joltages,
            }
        })
        .collect()
}

fn parse_targets(s: &str) -> Vec<usize> {
    s.trim_matches(&['[', ']'])
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => {
                dbg!(c);
                panic!("bad char")
            }
        })
        .collect::<Vec<_>>()
}

fn parse_num_list(strip: &[char; 2], s: &str) -> Vec<usize> {
    s.trim_matches(strip)
        .split(',')
        .map(|n| n.parse().expect("should be a number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";

    #[test]
    fn tparse() {
        let ms = parse(TEST_INPUT);
        assert_eq!(ms.len(), 3);

        let m = &ms[1];

        assert_eq!(m.target_indicators, &[0, 0, 0, 1, 0]);
        assert_eq!(m.joltages, &[7, 5, 12, 7, 2]);
        assert_eq!(m.button_combos[0], &[0, 2, 3, 4]);
    }

    #[test]
    fn tlogic() {
        let mut ms = parse(TEST_INPUT);
        let m = ms.get_mut(1).unwrap();

        apply_combo_toggle(&mut m.indicators, &m.button_combos[0]);
        assert_eq!(m.indicators, &[1, 0, 1, 1, 1]);
        apply_combo_toggle(&mut m.target_indicators, &m.button_combos[0]);
        assert_eq!(m.target_indicators, &[1, 0, 1, 0, 1]);
    }

    #[test]
    fn tsolve_machine() {
        let ms = parse(TEST_INPUT);
        let m = &ms[1];

        assert_eq!(solve_machine_toggle(m), 3);
    }

    #[test]
    fn tp1() {
        assert_eq!(Day10::part1(TEST_INPUT).to_string(), "7");
    }

    #[test]
    fn tp2() {
        assert_eq!(Day10::part2(TEST_INPUT).to_string(), "33");
    }
}
