use aoc_runner_derive::{aoc, aoc_generator};

type Crate = char;
type Stack = Vec<Crate>;

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
        .map(|_| Stack::new())
        .collect();

    for line in lines {
        for (index, char) in line.chars().skip(1).step_by(4).enumerate() {
            match char {
                'A'..='Z' => stacks[index].push(char),
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
        let stack = &mut stacks[*from - 1];
        let offset = stack.len() - quantity;
        buffer.extend(stack.drain(offset..));
        let dest = &mut stacks[*to - 1];
        for unit in buffer.drain(..).rev() {
            dest.push(unit);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[aoc(day5, part2)]
fn perform_stackwise_moves((stacks, moves): &(Vec<Stack>, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    let mut buffer: Vec<Crate> = vec![];

    for Move { from, to, quantity } in moves {
        buffer.clear();
        let stack = &mut stacks[*from - 1];
        let offset = stack.len() - quantity;
        buffer.extend(stack.drain(offset..));
        let dest = &mut stacks[*to - 1];
        for unit in buffer.drain(..) {
            dest.push(unit);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}
