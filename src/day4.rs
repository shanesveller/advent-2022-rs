use aoc_runner_derive::{aoc, aoc_generator};
use byte_set::ByteSet;

pub struct SectionRange(ByteSet);

fn parse_range(r: &str) -> SectionRange {
    let (left, right) = r.split_once('-').unwrap();
    let start = left.parse().unwrap();
    let end = right.parse().unwrap();

    SectionRange(ByteSet::from_range(start..end))
}

fn parse_line(l: &str) -> (SectionRange, SectionRange) {
    let (left, right) = l.split_once(',').unwrap();
    (parse_range(left), parse_range(right))
}

#[aoc_generator(day4)]
fn parse_ranges(input: &str) -> Vec<(SectionRange, SectionRange)> {
    input.lines().map(parse_line).collect()
}

#[aoc(day4, part1)]
fn count_containment(input: &[(SectionRange, SectionRange)]) -> usize {
    input.iter().filter(|(l, r)| {
        l.0.is_subset(&r.0) || r.0.is_subset(&l.0)
    }).count()
}

#[aoc(day4, part2)]
fn count_overlap(input: &[(SectionRange, SectionRange)]) -> usize {
    input.iter().filter(|(l, r)| {
        !l.0.is_disjoint(&r.0)
    }).count()
}
