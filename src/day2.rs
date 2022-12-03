use std::convert::{Infallible, TryFrom};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Play {
    fn outcome_against(&self, opponent: &Self) -> Outcome {
        use Outcome::*;
        use Play::*;

        match (self, opponent) {
            (Rock, Scissors) => Win,
            (Rock, Paper) => Loss,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Rock) => Loss,
            _ => Draw,
        }
    }

    fn play_for_outcome(&self, outcome: &Outcome) -> Play {
        use Outcome::*;
        use Play::*;

        match (self, outcome) {
            (Rock, Win) => Paper,
            (Rock, Loss) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Loss) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Loss) => Paper,
            (play, Draw) => *play,
        }
    }
}

trait Score {
    fn score(&self) -> u16;
}

impl Score for Outcome {
    fn score(&self) -> u16 {
        use Outcome::*;
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

impl Score for Play {
    fn score(&self) -> u16 {
        use Play::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl TryFrom<&'_ str> for Play {
    type Error = Infallible;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Play::*;

        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&'_ str> for Outcome {
    type Error = Infallible;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Outcome::*;

        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Play, String)> {
    input
        .lines()
        .map(|l| l.split_once(' ').expect("whitespace not found"))
        .map(|(l, r)| {
            (
                Play::try_from(l).expect("could not parse play"),
                String::from(r),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn decode_play_and_sum_score(input: &[(Play, String)]) -> u16 {
    input
        .iter()
        .map(|(opponent, me)| {
            (
                opponent,
                Play::try_from(&me[..]).expect("could not parse play"),
            )
        })
        .map(|(opponent, me)| me.outcome_against(opponent).score() + me.score())
        .sum()
}

#[aoc(day2, part2)]
pub fn decode_outcome_and_sum_score(input: &[(Play, String)]) -> u16 {
    input
        .iter()
        .map(|(opponent, me)| {
            (
                opponent,
                Outcome::try_from(&me[..]).expect("could not parse outcome"),
            )
        })
        .map(|(opponent, outcome)| {
            let play = opponent.play_for_outcome(&outcome);
            outcome.score() + play.score()
        })
        .sum()
}
