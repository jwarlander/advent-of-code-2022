use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut v = vec![0];

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(num) => *v.last_mut().unwrap() += num,
            Err(_) => v.push(0),
        }
    }

    *v.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut v = vec![0];

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(num) => *v.last_mut().unwrap() += num,
            Err(_) => v.push(0),
        }
    }

    v.sort_by(|a, b| b.cmp(a));
    v.iter().take(3).sum()
}
