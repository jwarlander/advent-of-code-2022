use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    fn parse_code(code: &str) -> Move {
        match code {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid code"),
        }
    }

    let mut score = 0;

    for line in input.lines() {
        match line
            .split_ascii_whitespace()
            .map(parse_code)
            .collect::<Vec<Move>>()
            .as_slice()
        {
            [theirs, ours] if theirs < ours => score += ours.score() + 6,
            [theirs, ours] if theirs == ours => score += ours.score() + 3,
            [theirs, ours] if theirs > ours => score += ours.score(),
            _ => panic!("Invalid input"),
        }
    }

    score
}

#[derive(Clone, PartialEq)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        self.to_owned() as i32
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Paper, Move::Scissors) => Some(Ordering::Less),
            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    fn parse_code_pair(pair: &[&str]) -> (Move, Move) {
        let their_move = match pair[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid code"),
        };

        let game_result = match pair[1] {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("Invalid code"),
        };

        let all_moves = vec![Move::Rock, Move::Paper, Move::Scissors];

        let our_move = match game_result {
            GameResult::Lose => all_moves.into_iter().find(|m| m < &their_move),
            GameResult::Draw => all_moves.into_iter().find(|m| m == &their_move),
            GameResult::Win => all_moves.into_iter().find(|m| m > &their_move),
        };

        (their_move, our_move.unwrap())
    }

    let mut score = 0;

    for line in input.lines() {
        let moves = line.split_ascii_whitespace().collect::<Vec<&str>>();

        match parse_code_pair(&moves) {
            (theirs, ours) if theirs < ours => score += ours.score() + 6,
            (theirs, ours) if theirs == ours => score += ours.score() + 3,
            (theirs, ours) if theirs > ours => score += ours.score(),
            _ => panic!("Invalid input"),
        }
    }

    score
}

enum GameResult {
    Lose,
    Draw,
    Win,
}
