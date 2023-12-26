use std::collections::{HashSet, VecDeque};

use num_integer::{Integer, Roots};

use crate::PuzzleInput;

type Position = usize;
#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}
const DIRS: [Direction; 4] = [
    Direction::Right,
    Direction::Up,
    Direction::Left,
    Direction::Down,
];

struct Garden {
    rocks: Vec<bool>,
    size: usize,
}

impl Garden {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut rocks: Vec<bool> = vec![];

        for line in lines {
            for c in line.chars() {
                rocks.push(c == '#');
            }
        }
        Self {
            size: rocks.len().sqrt(),
            rocks,
        }
    }

    fn step(&self, position: Position, direciton: Direction) -> Option<Position> {
        let (row, col) = position.div_rem(&self.size);
        if match direciton {
            Direction::Right => col < self.size - 1,
            Direction::Up => row > 0,
            Direction::Left => col > 0,
            Direction::Down => row < self.size - 1,
        } {
            let next_pos = match direciton {
                Direction::Right => position + 1,
                Direction::Up => position - self.size,
                Direction::Left => position - 1,
                Direction::Down => position + self.size,
            };
            if !self.rocks[next_pos] {
                Some(next_pos)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn center(&self) -> Position {
        let c = self.size / 2;
        c * self.size + c
    }

    #[allow(unused)]
    /// Inefficient endless garden plot count for testing
    fn count_reachable_plots_endless(&self, steps: usize) -> usize {
        // Reachable plots are in a checkerboard pattern
        let even = steps % 2;
        let (row, col) = {
            let (r, c) = self.center().div_rem(&self.size);
            (r as isize, c as isize)
        };

        let mut positions: VecDeque<((isize, isize), usize)> = [((0, 0), 0)].into_iter().collect();
        let mut visited: HashSet<(isize, isize)> = [(0, 0)].into_iter().collect();
        let mut plots = 0;
        while let Some(((y, x), step)) = positions.pop_front() {
            if step % 2 == even {
                plots += 1;
            }

            if step < steps {
                positions.extend(
                    DIRS.into_iter()
                        .filter_map(|d| {
                            let (yn, xn) = match d {
                                Direction::Right => (y, x + 1),
                                Direction::Up => (y - 1, x),
                                Direction::Left => (y, x - 1),
                                Direction::Down => (y + 1, x),
                            };
                            let s = self.size as isize;
                            let (r, c) = ((row + yn).mod_floor(&s), (col + xn).mod_floor(&s));
                            if !self.rocks[(r * s + c) as usize] {
                                Some((yn, xn))
                            } else {
                                None
                            }
                        })
                        .filter(|p| visited.insert(*p))
                        .map(|p| (p, step + 1)),
                );
            }
        }
        plots
    }

    fn count_reachable_plots(&self, start: Position, steps: usize) -> usize {
        // Reachable plots are in a checkerboard pattern
        let mut plots = 0;
        let even = steps % 2;

        // Find all reachable positions
        let mut positions: VecDeque<(Position, usize)> = [(start, 0)].into_iter().collect();
        let mut visited = [false].repeat(self.rocks.len());
        visited[start] = true;
        while let Some((pos, step)) = positions.pop_front() {
            plots += (step % 2 == even) as usize;

            if step < steps {
                positions.extend(
                    DIRS.into_iter()
                        .filter_map(|d| self.step(pos, d))
                        .filter(|p| {
                            let v = &mut visited[*p];
                            let is_new = !*v;
                            if is_new {
                                *v = true;
                            }
                            is_new
                        })
                        .map(|p| (p, step + 1)),
                );
            }
        }

        plots
    }
}

fn quadrant_odd_count(length: usize) -> usize {
    let l = (length + 1) / 2;
    l * l
}

fn quadrant_even_count(length: usize) -> usize {
    let l = length / 2;
    l * (l + 1)
}

fn run(input: PuzzleInput, steps: usize) -> usize {
    let garden = Garden::parse(input);
    let start_border_distance = garden.size / 2;

    let center_start = garden.center();
    if steps < start_border_distance {
        garden.count_reachable_plots(center_start, steps)
    } else {
        // Assumptions which are not specified in puzzle text:
        // - Start is at the center of the square garden (with odd side lengths)
        // - Start has unobstructed vertical and horizontal lines
        let full_count = garden.count_reachable_plots(center_start, steps);
        let full_odd_count = garden.count_reachable_plots(center_start, steps - 1);
        let (garden_repeats, residual_steps) =
            (steps - start_border_distance).div_rem(&garden.size);
        assert!(
            residual_steps == 0,
            "required to simplify (for garden_repeats-1 to be full & exactly one peak garden)"
        );
        let peak_steps = garden.size - 1;
        let full_gardens = garden_repeats - 1;
        let quadrant_full_even_gardens = quadrant_even_count(full_gardens);
        let quadrant_full_odd_gardens = quadrant_odd_count(full_gardens);
        let quadrant_full_plots = (full_count * quadrant_full_even_gardens)
            + (full_odd_count * quadrant_full_odd_gardens);

        let quadrant_border_plots = DIRS
            .into_iter()
            .map(|d| {
                let peak_garden_plots = {
                    let (row, col) = match d {
                        Direction::Right => (start_border_distance, 0),
                        Direction::Up => (garden.size - 1, start_border_distance),
                        Direction::Left => (start_border_distance, garden.size - 1),
                        Direction::Down => (0, start_border_distance),
                    };
                    let peak_start = row * garden.size + col;
                    garden.count_reachable_plots(peak_start, peak_steps)
                };

                // Count diagonal border of quadrant (the hypotenuse)
                let diagonal_plots = {
                    let (row, col) = match d {
                        // Top left
                        Direction::Right => (0, 0),
                        // Bottom left
                        Direction::Up => (garden.size - 1, 0),
                        // Bottom right
                        Direction::Left => (garden.size - 1, garden.size - 1),
                        // Top right
                        Direction::Down => (0, garden.size - 1),
                    };
                    let corner_start = row * garden.size + col;
                    // Moving diagonally (origin is to the back, peak to the front):
                    // Move left from center of peak
                    let minor_diagonal = garden.count_reachable_plots(
                        corner_start,
                        peak_steps - start_border_distance - 1,
                    );
                    // Then move one garden backward from peak
                    let major_diagonal = garden.count_reachable_plots(
                        corner_start,
                        peak_steps - start_border_distance - 1 + garden.size,
                    );

                    minor_diagonal + (minor_diagonal + major_diagonal) * full_gardens
                };

                peak_garden_plots + diagonal_plots
            })
            .sum::<usize>();
        // Count middle + 4 triangular quadrants + diagonal borders
        full_count + 4 * quadrant_full_plots + quadrant_border_plots
    }
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, if part2 { 26501365 } else { 64 }).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    #[test]
    fn test_quadrants() {
        assert_eq!(quadrant_odd_count(0), 0);
        assert_eq!(quadrant_odd_count(1), 1);
        assert_eq!(quadrant_odd_count(2), 1);
        assert_eq!(quadrant_odd_count(3), 1 + 3);
        assert_eq!(quadrant_odd_count(4), 1 + 3);
        assert_eq!(quadrant_odd_count(5), 1 + 3 + 5);
        assert_eq!(quadrant_odd_count(6), 1 + 3 + 5);
        assert_eq!(quadrant_odd_count(7), 1 + 3 + 5 + 7);
        assert_eq!(quadrant_odd_count(8), 1 + 3 + 5 + 7);
        assert_eq!(quadrant_odd_count(9), 1 + 3 + 5 + 7 + 9);

        assert_eq!(quadrant_even_count(0), 0);
        assert_eq!(quadrant_even_count(1), 0);
        assert_eq!(quadrant_even_count(2), 2);
        assert_eq!(quadrant_even_count(3), 2);
        assert_eq!(quadrant_even_count(4), 2 + 4);
        assert_eq!(quadrant_even_count(5), 2 + 4);
        assert_eq!(quadrant_even_count(6), 2 + 4 + 6);
        assert_eq!(quadrant_even_count(7), 2 + 4 + 6);
        assert_eq!(quadrant_even_count(8), 2 + 4 + 6 + 8);
        assert_eq!(quadrant_even_count(9), 2 + 4 + 6 + 8);
    }

    #[test]
    fn test_endless() {
        let garden = Garden::parse(PuzzleInput::from(EXAMPLE));
        assert_eq!(garden.count_reachable_plots_endless(6), 16);
        assert_eq!(garden.count_reachable_plots_endless(10), 50);
        assert_eq!(garden.count_reachable_plots_endless(100), 6536);
        assert_eq!(garden.count_reachable_plots_endless(500), 167004);
        //assert_eq!(garden.count_reachable_plots_endless(1000), 668697);
        //assert_eq!(garden.count_reachable_plots_endless(5000), 16733044);
    }

    macro_rules! run_compare {
        ($example:expr,$steps:expr) => {
            println!("___STEPS: {}", $steps);
            let steps = $steps;
            let garden = Garden::parse(PuzzleInput::from($example));
            assert_eq!(
                run($example.into(), steps),
                garden.count_reachable_plots_endless(steps)
            );
        };
    }

    #[test]
    fn test_clear_endless() {
        let clear_example = r"...........
...........
...........
...........
...........
.....S.....
...........
...........
...........
...........
...........";
        run_compare!(clear_example, 5 + 11);
        run_compare!(clear_example, 5 + 11 * 2);
        run_compare!(clear_example, 5 + 11 * 3);
        run_compare!(clear_example, 5 + 11 * 4);
        run_compare!(clear_example, 5 + 11 * 5);
    }

    #[test]
    fn test_simplify_endless() {
        let simplify_example = r"...........
......##.#.
.###..#..#.
..#.#...#..
....#.#....
.....S.....
.##......#.
.......##..
.##.#.####.
.##...#.##.
...........";
        run_compare!(simplify_example, 5 + 11);
        run_compare!(simplify_example, 5 + 11 * 2);
        run_compare!(simplify_example, 5 + 11 * 3);
        run_compare!(simplify_example, 5 + 11 * 4);
        run_compare!(simplify_example, 5 + 11 * 5);
    }
}
