use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

type Item = char;
type Compartment = HashSet<Item>;
type Rucksack = (Compartment, Compartment);
type Priority = u16;

fn priority(c: Item) -> Priority {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => unreachable!(),
    }
}

fn find_missorted_item(left: &Compartment, right: &Compartment) -> Item {
    left.intersection(right)
        .copied()
        .next()
        .expect("no common item")
}

fn find_common_item(rucksacks: &[HashSet<Item>]) -> Item {
    LETTERS
        .chars()
        .find(|c| rucksacks.iter().all(|r| r.contains(c)))
        .expect("no common item found")
}

fn parse_rucksack(l: &str) -> Rucksack {
    let midpoint = l.len() / 2;
    let items = parse_items(l);
    let (left, right) = items.split_at(midpoint);

    (parse_compartment(left), parse_compartment(right))
}

fn parse_items(l: &str) -> Vec<Item> {
    l.chars().collect()
}

fn parse_compartment(items: &[Item]) -> Compartment {
    Compartment::from_iter(items.iter().copied())
}

#[aoc_generator(day3, part1)]
pub fn rucksack_generator(input: &str) -> Vec<Rucksack> {
    input.lines().map(parse_rucksack).collect()
}

#[aoc_generator(day3, part2)]
pub fn items_generator(input: &str) -> Vec<HashSet<Item>> {
    input
        .lines()
        .map(|l| HashSet::from_iter(l.chars()))
        .collect()
}

#[aoc(day3, part1)]
pub fn misplaced_items(input: &[Rucksack]) -> Priority {
    input
        .iter()
        .map(|(l, r)| find_missorted_item(l, r))
        .map(priority)
        .sum()
}

#[aoc(day3, part2)]
pub fn badge_groups(input: &[HashSet<Item>]) -> Priority {
    input.chunks(3).map(find_common_item).map(priority).sum()
}
