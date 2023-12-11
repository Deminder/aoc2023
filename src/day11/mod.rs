use itertools::Itertools;

use crate::PuzzleInput;
use bitvec::prelude::*;

type Galaxy = (usize, usize);

fn expanded_galaxies(
    point_rows: Vec<Vec<usize>>,
    empty_cols: &BitVec,
    factor: usize,
) -> Vec<Galaxy> {
    point_rows
        .into_iter()
        .enumerate()
        .scan(0_usize, |extra_rows, (row, cols)| {
            if cols.is_empty() {
                *extra_rows += factor - 1;
            }
            Some((row + *extra_rows, cols))
        })
        .flat_map(|(row, cols)| {
            cols.into_iter().scan(
                (0, 0),
                move |(extra_cols, last_col): &mut (usize, usize), col| {
                    *extra_cols += (factor - 1) * empty_cols[*last_col..col].count_ones();
                    *last_col = col;
                    Some((row, col + *extra_cols))
                },
            )
        })
        .collect()
}

fn distance(point_a: Galaxy, point_b: Galaxy) -> usize {
    point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
}

fn run(mut input: PuzzleInput, expansion_factor: usize) -> usize {
    let first_line = input.next().unwrap();
    let mut empty_cols = bitvec!(1; first_line.chars().count());

    let point_rows = [first_line]
        .into_iter()
        .chain(input)
        .map(|line| {
            line.char_indices()
                .filter(|(_, c)| *c == '#')
                .map(|(col, _)| {
                    empty_cols.set(col, false);
                    col
                })
                .collect_vec()
        })
        .collect_vec();

    expanded_galaxies(point_rows, &empty_cols, expansion_factor)
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| distance(a, b))
        .sum()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, if part2 { 1_000_000 } else { 2 }).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(run(test_input.into(), 2), 374);
        assert_eq!(run(test_input.into(), 10), 1030);
        assert_eq!(run(test_input.into(), 100), 8410);
    }
}
