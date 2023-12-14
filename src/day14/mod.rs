use std::collections::hash_map::Entry;
use std::{collections::HashMap, fmt::Display};

use bitvec::prelude::*;

use crate::PuzzleInput;

enum Orientation {
    North,
    West,
    South,
    East,
}

impl Orientation {
    fn couter_clockwise() -> [Orientation; 4] {
        [
            Orientation::North,
            Orientation::West,
            Orientation::South,
            Orientation::East,
        ]
    }

    /// Transform a position from this orientation to north and return its index
    fn index(&self, row: usize, col: usize, size: usize) -> usize {
        let (r, c) = match self {
            Orientation::North => (row, col),
            Orientation::West => (size - 1 - col, row),
            Orientation::South => (size - 1 - row, size - 1 - col),
            Orientation::East => (col, size - 1 - row),
        };
        r * size + c
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RockGrid {
    round_rocks: BitVec,
    cube_rocks: Vec<bool>,
    size: usize,
}

impl RockGrid {
    fn parse(mut input: impl Iterator<Item = String>) -> Self {
        let first_line = input.next().unwrap();
        let size = first_line.chars().count();
        let mut round_rocks = bitvec!(0; size * size);
        let mut cube_rocks = round_rocks.clone();
        for (row, line) in [first_line].into_iter().chain(input).enumerate() {
            for (col, c) in line.char_indices() {
                let index = row * size + col;
                match c {
                    '#' => cube_rocks.set(index, true),
                    'O' => round_rocks.set(index, true),
                    _ => (),
                }
            }
        }
        Self {
            round_rocks,
            // Column access is faster for Vec<bool> than for BitVec.
            // (Precomputing BitVec rotations is about as fast as Vec<bool>.)
            cube_rocks: cube_rocks.into_iter().collect(),
            size,
        }
    }

    fn row<'a>(&self, v: &'a BitVec, row: usize) -> &'a BitSlice {
        &v[row * self.size..(row + 1) * self.size]
    }

    fn roll_up_and_west_to_up(&self, cube_rock_orientation: Orientation) -> BitVec {
        let mut block_height = [0].repeat(self.size);
        let mut rolled_round_rocks = bitvec!(0; self.round_rocks.len());
        for row in 0..self.size {
            for col in (0..self.size)
                .filter(|&col| self.cube_rocks[cube_rock_orientation.index(row, col, self.size)])
            {
                // Column has a blocking cube rock
                block_height[col] = row + 1;
            }
            for col in self.row(&self.round_rocks, row).iter_ones() {
                // Column has falling round rock
                rolled_round_rocks.set(
                    // Rotate left by transforming index East to North
                    Orientation::East.index(block_height[col], col, self.size),
                    true,
                );
                block_height[col] += 1;
            }
        }
        rolled_round_rocks
    }

    fn cycle(&mut self) {
        for cube_rock_orientation in Orientation::couter_clockwise() {
            // Rotate target direction to up/north
            self.round_rocks = self.roll_up_and_west_to_up(cube_rock_orientation);
        }
    }

    fn north_load(&self) -> usize {
        (0..self.size)
            .map(|row| self.row(&self.round_rocks, row).count_ones() * (self.size - row))
            .sum()
    }

    /// Part 1 compute north load after rolling up without mutating grid
    fn north_load_after_roll_up(&self) -> usize {
        let mut load_by_column = [0].repeat(self.size);
        let mut block_height = load_by_column.clone();

        for row in 0..self.size {
            for col in (0..self.size)
                .filter(|&col| self.cube_rocks[Orientation::North.index(row, col, self.size)])
            {
                // Column has a blocking cube rock
                block_height[col] = row + 1;
            }
            for col in self.row(&self.round_rocks, row).iter_ones() {
                // Column has falling round rock
                load_by_column[col] += self.size - block_height[col];
                block_height[col] += 1;
            }
        }
        load_by_column.into_iter().sum()
    }
}

impl Display for RockGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.size {
            let rrow = self.row(&self.round_rocks, r);
            writeln!(
                f,
                "{}",
                (0..self.size)
                    .map(
                        |col| if self.cube_rocks[Orientation::North.index(r, col, self.size)] {
                            '#'
                        } else if rrow[col] {
                            'O'
                        } else {
                            '.'
                        }
                    )
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

struct History(HashMap<usize, Vec<(usize, BitVec)>>);

impl History {
    fn new() -> Self {
        Self(HashMap::new())
    }

    // Insert a (load => (cycle, rocks)) entry.
    // If rocks has already been inserted, return its cycle instead.
    fn insert(&mut self, load: usize, cycle: usize, rocks: &BitVec) -> Option<usize> {
        let history = match self.0.entry(load) {
            Entry::Vacant(entry) => entry.insert(vec![]),
            Entry::Occupied(entry) => entry.into_mut(),
        };
        if let Some(cycle) = history
            .iter()
            .find(|(_, history_rocks)| history_rocks == rocks)
            .map(|(cycle, _)| *cycle)
        {
            Some(cycle)
        } else {
            history.push((cycle, rocks.clone()));
            None
        }
    }
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let mut grid = RockGrid::parse(input);
    if part2 {
        let mut cycle = 0;
        let mut cycle_history = History::new();
        let cycle_target = 1000000000;
        while cycle < cycle_target {
            grid.cycle();
            cycle += 1;
            let load = grid.north_load();
            if let Some(prev_cycle) = cycle_history.insert(load, cycle, &grid.round_rocks) {
                // Use a repeating cycle to skip cycles until near target
                let cycle_len = cycle - prev_cycle;
                let remaining = cycle_target - cycle;
                cycle += cycle_len * (remaining / cycle_len);
            }
        }
        grid.north_load()
    } else {
        grid.north_load_after_roll_up()
    }
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(run(test_input.into(), false), 136);
        let mut grid = RockGrid::parse(PuzzleInput::from(test_input));
        grid.cycle();
        assert_eq!(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
            grid.to_string().trim_end()
        );
        grid.cycle();
        assert_eq!(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
            grid.to_string().trim_end()
        );
        grid.cycle();
        assert_eq!(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
            grid.to_string().trim_end()
        );
        assert_eq!(grid.north_load(), 69);
        assert_eq!(run(test_input.into(), true), 64);
    }
}
