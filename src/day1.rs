use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|ls| ls.split('\n'))
        .map(|ns| ns.into_iter().map(|n| n.parse::<usize>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn max(input: &[usize]) -> usize {
    *input.iter().max().unwrap_or(&0)
}

#[aoc(day1, part2)]
pub fn max_three(input: &[usize]) -> usize {
    let mut groups: Vec<_> = input.to_vec();
    groups.sort();
    groups.into_iter().rev().take(3).sum()
}
