use std::fmt::Display;

use bitvec::prelude::*;

use crate::PuzzleInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right = 0,
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

impl Direction {
    fn left(&self) -> Direction {
        self.right().inverse()
    }

    fn right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    fn inverse(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }

    fn next(&self) -> impl Iterator<Item = Direction> {
        let inverse_direction = self.inverse();
        DIRS.into_iter().filter(move |d| inverse_direction != *d)
    }
}

type Pos = usize;

struct PipeGrid {
    directions: BitVec,
    width: usize,
    start_pos: usize,
}

struct PipeGridWalk<'a> {
    grid: &'a PipeGrid,
    walk_direction: Direction,
    pos: Pos,
}

impl<'a> Iterator for PipeGridWalk<'a> {
    type Item = (Pos, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = self.grid.move_pos(self.pos, self.walk_direction);
        if self.pos == self.grid.start_pos {
            None
        } else {
            self.walk_direction = self.grid.output_direction(self.pos, self.walk_direction);
            Some((self.pos, self.walk_direction))
        }
    }
}
fn pipe_char_to_directions(pipe_char: char) -> [bool; 4] {
    let f = false;
    match pipe_char {
        'L' => [true, true, f, f],
        'J' => [f, true, true, f],
        '7' => [f, f, true, true],
        '-' => [true, f, true, f],
        '|' => [f, true, f, true],
        'F' => [true, f, f, true],
        _ => [f, f, f, f],
    }
}

fn directions_to_pipe_char(has_directions: [bool; 4]) -> char {
    match has_directions {
        [true, true, _, _] => '┗',
        [_, true, true, _] => '┛',
        [_, _, true, true] => '┓',
        [true, _, true, _] => '━',
        [_, true, _, true] => '┃',
        [true, _, _, true] => '┏',
        _ => ' ',
    }
}
fn direction_to_walk_char(dir: Direction) -> char {
    match dir {
        Direction::Right => '▷',
        Direction::Up => '△',
        Direction::Left => '◁',
        Direction::Down => '▽',
    }
}
impl<'a> Display for PipeGridWalk<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid.width {
            for col in 0..self.grid.width {
                let pos = row * self.grid.width + col;
                let cur_pos = pos == self.pos;
                write!(
                    f,
                    "{}",
                    if cur_pos {
                        direction_to_walk_char(self.walk_direction)
                    } else if pos == self.grid.start_pos {
                        '○'
                    } else {
                        directions_to_pipe_char(DIRS.map(|d| self.grid.has_direction(pos, d)))
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PipeGrid {
    fn parse(mut input: PuzzleInput) -> Self {
        let first_line = input.next().unwrap();
        let width = first_line.chars().count();
        // Parse a square grid into a `(row, col, direction) => bool` collection
        let bitcount = 4 * width * width;
        let mut directions = BitVec::with_capacity(bitcount);
        let mut start_pos = 0;
        for (row, line) in [first_line].into_iter().chain(input).enumerate() {
            if start_pos == 0 {
                if let Some(col) = line.find('S') {
                    start_pos = row * width + col;
                }
            }
            directions.extend(line.chars().flat_map(pipe_char_to_directions))
        }
        directions.extend(bitvec!(0; bitcount - directions.len()));
        Self {
            directions,
            start_pos,
            width,
        }
    }

    fn output_direction(&self, pos: Pos, walk_direction: Direction) -> Direction {
        walk_direction
            .next()
            .find(|d| self.has_direction(pos, *d))
            .expect("pos should be connected")
    }

    fn has_direction(&self, pos: Pos, direction: Direction) -> bool {
        self.directions[4 * pos + direction as usize]
    }

    fn move_pos(&self, pos: Pos, move_direction: Direction) -> Pos {
        match move_direction {
            Direction::Right => pos + 1,
            Direction::Up => pos - self.width,
            Direction::Left => pos - 1,
            Direction::Down => pos + self.width,
        }
    }

    fn row_col(&self, pos: Pos) -> (usize, usize) {
        num_integer::div_rem(pos, self.width)
    }

    fn walk(&self) -> PipeGridWalk<'_> {
        PipeGridWalk {
            grid: self,
            walk_direction: DIRS
                .into_iter()
                .find(|d| self.has_direction(self.move_pos(self.start_pos, *d), d.inverse()))
                .expect("start should be connected"),
            pos: self.start_pos,
        }
    }

    fn move_possible(&self, pos: Pos, dir: Direction) -> bool {
        let (row, col) = self.row_col(pos);
        match dir {
            Direction::Right => col < self.width - 1,
            Direction::Up => row > 0,
            Direction::Left => col > 0,
            Direction::Down => row < self.width - 1,
        }
    }
}

fn flood_fill_empty(field: &mut BitVec, start_pos: Pos, empty: &BitVec, grid: &PipeGrid) {
    let mut next_positions = vec![start_pos];
    while !next_positions.is_empty() {
        next_positions = next_positions
            .into_iter()
            .filter(|pos| {
                let p = *pos;
                let is_new = empty[p] && !field[p];
                if is_new {
                    field.set(p, true);
                }
                is_new
            })
            .flat_map(|pos| {
                DIRS.into_iter().filter_map(move |dir| {
                    if grid.move_possible(pos, dir) {
                        Some(grid.move_pos(pos, dir))
                    } else {
                        None
                    }
                })
            })
            .collect();
    }
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let grid = PipeGrid::parse(input);
    if part2 {
        // Mark non-pipes by walking the pipe loop
        let size = grid.width * grid.width;
        let mut no_pipes = bitvec!(1; size);
        let walk_iter = grid.walk();
        let mut last_forward = walk_iter.walk_direction;
        let mut right_turns: isize = 0;
        let mut moves = vec![];
        for (pos, forward) in walk_iter {
            right_turns += if last_forward == forward {
                0
            } else if last_forward.right() == forward {
                1
            } else {
                -1
            };
            last_forward = forward;
            no_pipes.set(pos, false);
            moves.push((pos, forward));
        }

        // Fill inner non-pipe fields
        // -> Expect inner on the right if more right turns than left turns
        let mut inner_field = bitvec!(0; size);
        for (pos, forward) in moves {
            let dir = if right_turns > 0 {
                forward.right()
            } else {
                forward.left()
            };
            if grid.move_possible(pos, dir) {
                flood_fill_empty(&mut inner_field, grid.move_pos(pos, dir), &no_pipes, &grid)
            }
        }
        inner_field.count_ones()
    } else {
        (grid.walk().count() + 1) / 2
    }
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let grid = PipeGrid::parse(
            r".....
.S-7.
.|.|.
.L-J.
....."
                .into(),
        );
        println!("{:?}", grid.directions);
        assert!(grid.has_direction(5 + 2, Direction::Right));
        assert!(!grid.has_direction(5 + 2, Direction::Up));
    }

    #[test]
    fn test_run() {
        let test_input = r".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(run(test_input.into(), false), 4);
        let test_input2 = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(run(test_input2.into(), false), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            run(
                r"...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O....."
                    .into(),
                true
            ),
            4
        );
        assert_eq!(
            run(
                r"..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."
                    .into(),
                true
            ),
            4
        );
        assert_eq!(
            run(
                r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
                    .into(),
                true
            ),
            8
        );
    }
}
