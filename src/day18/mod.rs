use std::{collections::BTreeMap, ops::Range};

mod ingressline;

#[cfg(feature = "plot")]
mod plot;
#[cfg(feature = "plot")]
use crate::range_intersect;

use ingressline::IngressLine;

use itertools::Itertools;

use crate::PuzzleInput;

use self::ingressline::IngressLineIdx;

type Direction = u8;
const RIGHT: Direction = 0;
const UP: Direction = 1;
const LEFT: Direction = 2;
const DOWN: Direction = 3;

#[derive(Debug, Clone)]
struct Color(u32);

#[derive(Debug, Clone)]
pub struct VerticalTrenchLine {
    row: i32,
    col: i32,
    length: i32,
    direction: Direction,
}

impl VerticalTrenchLine {
    /// Range of vertical rows. (The includes an additional tail.)
    fn row_range(&self) -> Range<i32> {
        match self.direction {
            DOWN => self.row..self.row + self.length + 1,
            UP => self.row - self.length..self.row + 1,
            _ => panic!("VerticalTrenchLine must be vertical"),
        }
    }
}

fn area(vertical_trench_lines: &[VerticalTrenchLine]) -> usize {
    let col_sorted_vertical_lines = vertical_trench_lines
        .iter()
        .sorted_by_key(|l| l.col)
        .collect_vec();
    let ingress_direction = col_sorted_vertical_lines[0].direction;
    #[cfg(feature = "plot")]
    #[allow(clippy::type_complexity)]
    let (mut trench_points, mut areas): (Vec<(i32, i32)>, Vec<(i32, i32, usize, usize)>) =
        (vec![], vec![]);

    // Sum horizontal lines which are to the right of an egress line
    // since these are missed by the following area scan.
    let egress_and_col =
        |line: &VerticalTrenchLine| (line.direction != ingress_direction, line.col);
    let mut area = vertical_trench_lines
        .iter()
        .scan(
            egress_and_col(vertical_trench_lines.last().unwrap()),
            |prev, line| {
                #[cfg(feature = "plot")]
                {
                    let row_range = line.row_range();
                    if line.direction == DOWN {
                        trench_points.push((row_range.start, line.col + 1));
                        trench_points.push((row_range.end, line.col + 1));
                    } else {
                        trench_points.push((row_range.end, line.col));
                        trench_points.push((row_range.start, line.col));
                    }
                }
                let hidden_length = {
                    let (start_egress, start_col) = *prev;
                    let (end_egress, end_col) = egress_and_col(line);
                    let length = end_col - start_col;
                    // Check if length is hidden behind an egress line
                    let both = start_egress && end_egress;
                    if both || (start_egress && length > 0) || (end_egress && length < 0) {
                        #[cfg(feature = "plot")]
                        areas.push((
                            line.row,
                            start_col.min(end_col) + 1,
                            1,
                            length.unsigned_abs() as usize - if both { 0 } else { 1 },
                        ));
                        length.unsigned_abs() as usize - if both { 0 } else { 1 }
                    } else {
                        0
                    }
                };
                *prev = egress_and_col(line);
                Some(hidden_length)
            },
        )
        .sum();

    let max_ingress_line_length = col_sorted_vertical_lines
        .iter()
        .filter(|l| l.direction == ingress_direction)
        .map(|l| l.length)
        .max()
        .unwrap();
    // Ingress lines indexable by their row
    let mut ingress_lines: BTreeMap<IngressLineIdx, IngressLine> = BTreeMap::new();

    // Add inner area by scanning all trench lines left to right
    // To produce an area product:
    // A vertical line must be to the right of an ingress line.
    for line in col_sorted_vertical_lines {
        let row_range = line.row_range();
        let previous_area = area;
        // Find ingress lines which intersect with this vertical line
        for (ingress, ingress_line) in ingress_lines.range_mut(IngressLineIdx::range(
            row_range.start - max_ingress_line_length - 1,
            row_range.end + 1,
        )) {
            #[cfg(feature = "plot")]
            {
                let span = line.col - ingress.col
                    + if line.direction == ingress_direction {
                        0
                    } else {
                        1
                    };
                for r in ingress_line.remaining() {
                    if let Some(range) = range_intersect(&r, &row_range) {
                        areas.push((
                            range.start,
                            ingress.col,
                            (range.end - range.start) as usize,
                            span as usize,
                        ));
                    }
                }
            }
            let height = ingress_line.occlude(&row_range);
            if height > 0 {
                // Area span from inclusive ingress line to:
                // inclusive for egress line, exclusive for ingress line
                let span = line.col - ingress.col
                    + if line.direction == ingress_direction {
                        0
                    } else {
                        1
                    };
                area += height * span as usize;
            }
        }
        if area > previous_area {
            ingress_lines.retain(|_, l| !l.fully_occluded());
        }
        if line.direction == ingress_direction {
            ingress_lines.insert(
                IngressLineIdx::new(row_range.start, line.col),
                IngressLine::new(row_range),
            );
        }
    }

    #[cfg(feature = "plot")]
    plot::plot_svg(&trench_points, &areas);

    area
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let vertical_trench_lines = input
        .map(|line| {
            let (direction, steps, color) = line.split_whitespace().collect_tuple().unwrap();

            if part2 {
                (
                    match color[7..8].parse::<u8>().unwrap() {
                        0 => RIGHT,
                        1 => DOWN,
                        2 => LEFT,
                        _ => UP,
                    },
                    color[2..7]
                        .chars()
                        .flat_map(|c| c.to_digit(16))
                        .map(|digit| digit as i32)
                        .reduce(|n, digit| 16 * n + digit)
                        .unwrap(),
                )
            } else {
                (
                    "RUL".find(direction).map(|d| d as u8).unwrap_or(DOWN),
                    steps.parse::<i32>().unwrap(),
                )
            }
        })
        .scan((0_i32, 0_i32), |(row, col), (d, s)| {
            let vertical = d % 2 == 1;
            let line = if vertical {
                Some(VerticalTrenchLine {
                    row: *row,
                    col: *col,
                    length: s,
                    direction: d,
                })
            } else {
                None
            };

            *row = match d {
                UP => *row - s,
                DOWN => *row + s,
                _ => *row,
            };
            *col = match d {
                RIGHT => *col + s,
                LEFT => *col - s,
                _ => *col,
            };
            Some(line)
        })
        .flatten()
        .collect_vec();

    area(&vertical_trench_lines)
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(run(test_input.into(), false), 62);
        assert_eq!(run(test_input.into(), true), 952408144115);
    }
}
