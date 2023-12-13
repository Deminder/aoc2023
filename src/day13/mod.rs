use itertools::Itertools;

use crate::{split_by_empty_line, PuzzleInput};

use bitvec::prelude::*;

#[derive(Debug, Clone)]
struct GridPattern {
    rocks: BitVec,
    width: usize,
    height: usize,
}

impl GridPattern {
    fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let first_line = lines.next().unwrap();
        let width = first_line.chars().count();
        let rocks: BitVec = [first_line]
            .into_iter()
            .chain(lines)
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut v = bitvec![0; width];
                for p in line.chars().positions(|c| c == '#') {
                    v.set(p, true);
                }
                v
            })
            .concat();
        Self {
            width,
            height: rocks.len() / width,
            rocks,
        }
    }
    fn row(&self, row: usize) -> &BitSlice {
        &self.rocks[row * self.width..(row + 1) * self.width]
    }

    /// Find row indices above reflection centers
    fn reflected_rows(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.height - 1).filter(|&row| {
            // Confirm complete row reflection
            let radius = row.min(self.height - 2 - row);
            (0..=radius).all(|r| self.row(row - r) == self.row(row + 1 + r))
        })
    }

    /// Find column indices to the left of reflection centers
    fn reflected_cols(&self) -> Vec<usize> {
        // Check row by row (for better cpu-cache utilization)
        let mut reflection_candidates = bitvec!(1; self.width-1);
        for h in 0..self.height {
            let row = self.row(h);
            for col in 0..self.width - 1 {
                if reflection_candidates[col]
                    && (0..=/*scan radius of reflection*/col.min(self.width - 2 - col))
                        .any(|r| row[col - r] != row[col + 1 + r])
                {
                    // Eliminate single col reflection candidate
                    reflection_candidates.set(col, false);
                }
            }
        }
        reflection_candidates.iter_ones().collect()
    }
}

fn summaries(g: &GridPattern) -> impl Iterator<Item = usize> + '_ {
    g.reflected_cols()
        .into_iter()
        .map(|col| col + 1)
        .chain(g.reflected_rows().map(|row| (row + 1) * 100))
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    split_by_empty_line!(input)
        .map(|lines| GridPattern::parse(lines))
        .map(|mut gp| {
            let sum = summaries(&gp).next().expect("either row or col reflection");
            if part2 {
                (0..gp.rocks.len())
                    .find_map(|index| {
                        let val = gp.rocks[index];
                        gp.rocks.set(index, !val);
                        let s = summaries(&gp).find(|&s| s != sum);
                        gp.rocks.set(index, val);
                        s
                    })
                    .expect("different row or col reflection after removing smudge")
            } else {
                sum
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
        let test_input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(run(test_input.into(), false), 405);
        assert_eq!(run(test_input.into(), true), 400);
    }
}
