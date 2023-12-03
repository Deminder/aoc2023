use std::{ops::Range, rc::Rc};

use itertools::Itertools;

use crate::PuzzleInput;

fn engine_at<const N: usize>(lines: [&Vec<char>; N], position: usize) -> bool {
    lines.into_iter().any(|line| {
        line.get(position)
            .is_some_and(|c| *c != '.' && !c.is_digit(10))
    })
}

fn has_adjacent_engine(
    range: Range<usize>,
    line_up: &Vec<char>,
    line_mid: &Vec<char>,
    line_down: &Vec<char>,
) -> bool {
    let all_lines = [line_up, line_mid, line_down];
    (range.start > 0 && engine_at(all_lines, range.start - 1))
        || engine_at(all_lines, range.end)
        || range
            .into_iter()
            .any(|p| engine_at([line_up, line_down], p))
}

fn line_number_ranges<'a>(line: &'a Vec<char>) -> impl Iterator<Item = Range<usize>> + 'a {
    line.iter()
        .chain([&'.'].into_iter())
        .enumerate()
        .scan(None as Option<usize>, |start, (pos, c)| {
            let is_num = c.is_digit(10);
            if start.is_some() {
                if !is_num {
                    // End of number range
                    let s = start.unwrap();
                    *start = None;
                    Some(Some(s..pos))
                } else {
                    Some(None)
                }
            } else {
                if is_num {
                    // Start of number range
                    *start = Some(pos);
                }
                Some(None)
            }
        })
        .filter_map(|v| v)
}

fn line_range_number(line: &Vec<char>, range: Range<usize>) -> u32 {
    line[range]
        .into_iter()
        .flat_map(|c| c.to_digit(10))
        .reduce(|acc, d| 10 * acc + d)
        .unwrap()
}

fn range_intersect(range1: &Range<usize>, range2: &Range<usize>) -> bool {
    let max_start = range1.start.max(range2.start);
    let min_end = range1.end.min(range2.end);
    max_start < min_end
}

fn gear_ratio(
    gear_position: usize,
    lines_and_ranges_cache: &[(Rc<Vec<char>>, Vec<Range<usize>>); 3],
) -> Option<u32> {
    // If exaclty two numbers are adjacent to '*', return their product
    let adjacent_range = if gear_position > 0 {
        gear_position - 1
    } else {
        gear_position
    }..gear_position + 2;
    lines_and_ranges_cache
        .iter()
        .flat_map(|(line, ranges)| {
            ranges
                .iter()
                .filter(|r| range_intersect(r, &adjacent_range))
                .map(|r| line_range_number(line, r.clone()))
        })
        .collect_tuple()
        // Product of the two numbers is the "gear ratio"
        .map(|(a, b)| a * b)
}

fn run(input: PuzzleInput, part2: bool) -> u32 {
    // Pad start and end of grid with an empty line
    let empty_line = Rc::new(vec![]);
    [empty_line.clone()]
        .into_iter()
        .chain(input.map(|line| Rc::new(line.chars().collect_vec())))
        .chain([empty_line].into_iter())
        .tuple_windows()
        .map(|(prev, cur, next)| {
            if part2 {
                let lines_and_ranges_cache = [&prev, &cur, &next]
                    .map(|line| (line.clone(), line_number_ranges(line).collect_vec()));
                cur.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == '*')
                    .filter_map(|(gear_position, _)| {
                        gear_ratio(gear_position, &lines_and_ranges_cache)
                    })
                    .sum::<u32>()
            } else {
                line_number_ranges(&cur)
                    .filter(|r| has_adjacent_engine(r.clone(), &prev, &cur, &next))
                    .map(|r| line_range_number(&cur, r.clone()))
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
