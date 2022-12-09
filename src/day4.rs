use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let mut n = 0;

    for line in input.lines() {
        let [ref first, ref second] = line.split(',')
            .map(parse_range)
            .collect::<Vec<_>>()[..] else { panic!("Invalid input, must have even pairs of ranges in each line: {}", line)};

        if first.is_subset(second) || second.is_subset(first) {
            n += 1;
        }
    }

    n
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let mut n = 0;

    for line in input.lines() {
        let [ref first, ref second] = line.split(',')
            .map(parse_range)
            .collect::<Vec<_>>()[..] else { panic!("Invalid input, must have even pairs of ranges in each line: {}", line)};

        if first.intersection(second).count() > 0 {
            n += 1;
        }
    }

    n
}

fn parse_range(input: &str) -> HashSet<i32> {
    let [start, stop] = input
        .split('-')
        .map(|w| w.parse().unwrap())
        .collect::<Vec<i32>>()[..] else { panic!("Invalid range: {}", input)};

    HashSet::from_iter(start..=stop)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 2);
    }
}
