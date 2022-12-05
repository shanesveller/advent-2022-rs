use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

type Crate = char;
type Stack = VecDeque<Crate>;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    quantity: usize,
}

fn parse_diagram(input: &str) -> Vec<Stack> {
    let mut lines = input.lines().rev();

    let labels = lines.next().unwrap();

    let mut stacks: Vec<Stack> = labels
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|_| VecDeque::new())
        .collect();

    for line in lines {
        for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
            match char {
                'A'..='Z' => stacks[index].push_front(char),
                ' ' => {}
                _ => unreachable!(),
            }
        }
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let mut nums = l.split(' ').filter_map(|s| s.parse::<usize>().ok());

            let quantity = nums.next().unwrap();
            let from = nums.next().unwrap();
            let to = nums.next().unwrap();

            Move { from, to, quantity }
        })
        .collect()
}

#[aoc_generator(day5)]
fn parse_instructions(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let (diagram, moves) = input.split_once("\n\n").expect("could not find separator");

    (parse_diagram(diagram), parse_moves(moves))
}

#[aoc(day5, part1)]
fn perform_moves((stacks, moves): &(Vec<Stack>, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    let mut buffer: Vec<Crate> = vec![];

    for Move { from, to, quantity } in moves {
        buffer.clear();
        buffer.extend(stacks[*from - 1].drain(0..(*quantity)));
        let dest = &mut stacks[*to - 1];
        for unit in buffer.drain(..) {
            dest.push_front(unit);
        }
    }

    stacks.iter().map(|stack| stack.front().unwrap()).collect()
}

#[aoc(day5, part2)]
fn perform_stackwise_moves((stacks, moves): &(Vec<Stack>, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    let mut buffer: Vec<Crate> = vec![];

    for Move { from, to, quantity } in moves {
        buffer.clear();
        buffer.extend(stacks[*from - 1].drain(0..(*quantity)));
        let dest = &mut stacks[*to - 1];
        for unit in buffer.drain(..) {
            dest.push_front(unit);
        }
    }

    stacks.iter().map(|stack| stack.front().unwrap()).collect()
}
