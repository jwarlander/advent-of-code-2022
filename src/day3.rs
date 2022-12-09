use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    use std::collections::HashSet;

    let mut sum = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let matches = left
            .chars()
            .filter(|c| right.contains(*c))
            .collect::<HashSet<char>>();
        sum += matches.iter().map(prio_for).sum::<i32>();
    }

    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    use std::collections::HashSet;

    let mut sum = 0;
    for group in input.lines().collect::<Vec<&str>>().chunks(3) {
        let [first, second, third] = group else { panic!("Invalid input") };
        let matches = first
            .chars()
            .filter(|c| second.contains(*c) && third.contains(*c))
            .collect::<HashSet<char>>();
        sum += matches.iter().map(prio_for).sum::<i32>();
    }

    sum
}

fn prio_for(c: &char) -> i32 {
    static PRIO: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    PRIO.find(*c).unwrap() as i32 + 1
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 70);
    }
}
