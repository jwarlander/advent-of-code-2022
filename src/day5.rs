use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(_input: &str) -> String {
    "X".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    static INPUT: &str = "\
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), "CMZ");
    }
}
