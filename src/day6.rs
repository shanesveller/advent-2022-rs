use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn find_unique_sequence_offset(input: &[char], target_length: usize) -> Option<usize> {
    input
        .windows(target_length)
        .enumerate()
        .find_map(|(offset, chars)| {
            let set: HashSet<char> = chars.iter().copied().collect();
            if set.len() == target_length {
                Some(offset + target_length)
            } else {
                None
            }
        })
}

#[aoc(day6, part1)]
pub fn find_start_of_packet(input: &[char]) -> usize {
    find_unique_sequence_offset(input, 4).unwrap()
}

#[aoc(day6, part2)]
pub fn max_three(input: &[char]) -> usize {
    find_unique_sequence_offset(input, 14).unwrap()
}
