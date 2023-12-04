use std::{ops::Range, rc::Rc};

use itertools::Itertools;
use regex::Regex;

use crate::PuzzleInput;

struct GridLine {
    symbol_ranges: Vec<Range<usize>>,
    number_ranges: Vec<Range<usize>>,
    line: String,
}

impl GridLine {
    fn new(line: String, symbol_regex: &Regex, number_regex: &Regex) -> Self {
        let [symbol_ranges, number_ranges] = [symbol_regex, number_regex]
            .map(|regex| regex.find_iter(&line).map(|m| m.range()).collect_vec());
        Self {
            line,
            symbol_ranges,
            number_ranges,
        }
    }
}

fn range_intersect(range1: &Range<usize>, range2: &Range<usize>) -> bool {
    let max_start = range1.start.max(range2.start);
    let min_end = range1.end.min(range2.end);
    max_start < min_end
}

fn part_number(number_str: &str, range: &Range<usize>, lines: [&GridLine; 3]) -> Option<u32> {
    let adjacent_range = if range.start > 0 {
        range.start - 1
    } else {
        range.start
    }..range.end + 1;
    if lines.iter().any(|line| {
        line.symbol_ranges
            .iter()
            .any(|r| range_intersect(r, &adjacent_range))
    }) {
        Some(number_str.parse().unwrap())
    } else {
        None
    }
}

fn gear_ratio(gear_position: usize, lines: [&GridLine; 3]) -> Option<u32> {
    // If exaclty two numbers are adjacent to '*', return their product
    let adjacent_range = if gear_position > 0 {
        gear_position - 1
    } else {
        gear_position
    }..gear_position + 2;
    lines
        .iter()
        .flat_map(|line| {
            line.number_ranges
                .iter()
                .filter(|r| range_intersect(r, &adjacent_range))
                .map(|r| line.line[r.clone()].parse::<u32>().unwrap())
        })
        .collect_tuple()
        // Product of the two numbers is the "gear ratio"
        .map(|(a, b)| a * b)
}

fn run(input: PuzzleInput, part2: bool) -> u32 {
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(if part2 { r"\*" } else { r"[^\.\d]" }).unwrap();
    // Pad start and end of grid with an empty line
    let empty_line = Rc::new(GridLine::new("".into(), &symbol_regex, &number_regex));
    [empty_line.clone()]
        .into_iter()
        .chain(input.map(|line| Rc::new(GridLine::new(line, &symbol_regex, &number_regex))))
        .chain([empty_line].into_iter())
        .tuple_windows()
        .map(|(prev, cur, next)| {
            if part2 {
                cur.symbol_ranges
                    .iter()
                    .filter_map(|r| gear_ratio(r.start, [&prev, &cur, &next]))
                    .sum::<u32>()
            } else {
                cur.number_ranges
                    .iter()
                    .filter_map(|r| part_number(&cur.line[r.clone()], r, [&prev, &cur, &next]))
                    .sum()
            }
        })
        .sum()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(run(test_input.into(), false), 4361);
        assert_eq!(run(test_input.into(), true), 467835);
    }
}
