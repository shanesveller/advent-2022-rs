use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut groups = Vec::new();
    let mut group = Vec::new();

    for line in input.lines() {
        match line {
            "" => {
                groups.push(group.clone());
                group.clear();
            }
            n => {
                group.push(str::parse::<usize>(n).unwrap());
            }
        }
    }

    groups.into_iter().map(|ns| ns.into_iter().sum()).collect()
}

#[aoc(day1, part1)]
pub fn max(input: &[usize]) -> usize {
    *input.into_iter().max().unwrap_or(&0)
}

#[aoc(day1, part2)]
pub fn max_three(input: &[usize]) -> usize {
    let mut groups: Vec<_> = input.to_vec();
    groups.sort();
    groups.into_iter().rev().take(3).sum()
}
