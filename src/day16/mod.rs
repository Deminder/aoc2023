use std::collections::{HashSet, VecDeque};

use bitvec::prelude::*;
use either::Either;
use itertools::Itertools;

use crate::PuzzleInput;

/// 0 => Right, 1 => Up, 2 => Left, 3 => Down
type Direction = usize;
const RIGHT: Direction = 0;
const UP: Direction = 1;
const LEFT: Direction = 2;
const DOWN: Direction = 3;

type Position = usize;

fn rotate(d: Direction, contraption: Contraption) -> Direction {
    let vertical = d % 2 == 1;
    (d + (match contraption {
        Contraption::CounterClockwise => {
            if vertical {
                3
            } else {
                1
            }
        }
        Contraption::Clockwise => {
            if vertical {
                1
            } else {
                3
            }
        }
        _ => 0,
    })) % 4
}

#[derive(Debug)]
enum Contraption {
    CounterClockwise,
    Clockwise,
    Horizontal,
    Vertical,
}

impl Contraption {
    fn parse(c: char) -> Option<Self> {
        match c {
            '/' => Some(Self::CounterClockwise),
            '\\' => Some(Self::Clockwise),
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    contraption: Contraption,
    /// Neighbors of this node in all four directions.
    /// A missing node implies that the direction reaches to the grid boundary.
    neighbors: [Option<Position>; 4],
}

type Beam = (Position, Direction);

#[derive(Debug)]
enum Step {
    Forward(Beam),
    Split(Beam, Beam),
}

/// Network of beam mirrors and splitters
struct ContraptionNetwork {
    nodes: Vec<Option<Node>>,
    size: usize,
}
fn position(row: usize, col: usize, size: usize) -> Position {
    size * row + col
}

impl ContraptionNetwork {
    fn parse(mut input: impl Iterator<Item = String>) -> Self {
        let first_line = input.next().unwrap();
        let size = first_line.chars().count();
        let mut from_up: Vec<Option<Position>> = (0..size).map(|_| None).collect();
        let mut nodes: Vec<Option<Node>> = (0..size * size).map(|_| None).collect();
        for (row, line) in [first_line].into_iter().chain(input).enumerate() {
            let mut from_left: Option<Position> = None;
            for (col, c) in line.char_indices() {
                if let Some(contraption) = Contraption::parse(c) {
                    let node = Node {
                        contraption,
                        neighbors: [None, from_up[col], from_left, None],
                    };
                    let pos = position(row, col, size);
                    nodes[pos] = Some(node);
                    if let Some(Some(left)) = from_left.and_then(|u| nodes.get_mut(u)) {
                        left.neighbors[RIGHT] = Some(pos)
                    }

                    if let Some(Some(up)) = from_up[col].and_then(|u| nodes.get_mut(u)) {
                        up.neighbors[DOWN] = Some(pos)
                    }

                    from_up[col] = Some(pos);
                    from_left = Some(pos);
                }
            }
        }
        Self { nodes, size }
    }

    fn boundary(&self, node_position: Position, direction: Direction) -> Position {
        position(
            match direction {
                UP => 0,
                DOWN => self.size - 1,
                _ =>
                /*keep row*/
                {
                    node_position / self.size
                }
            },
            match direction {
                RIGHT => self.size - 1,
                LEFT => 0,
                _ =>
                /*keep col*/
                {
                    node_position % self.size
                }
            },
            self.size,
        )
    }

    fn step(&self, beam: Beam) -> Step {
        let (node_position, direction) = beam;
        if let Some(Some(n)) = self.nodes.get(node_position) {
            let forward = |contraption| {
                let out_direction = rotate(direction, contraption);
                (
                    n.neighbors[out_direction]
                        .unwrap_or_else(|| self.boundary(node_position, out_direction)),
                    out_direction,
                )
            };
            match n.contraption {
                Contraption::CounterClockwise => {
                    Step::Forward(forward(Contraption::CounterClockwise))
                }
                Contraption::Clockwise => Step::Forward(forward(Contraption::Clockwise)),
                Contraption::Horizontal if direction % 2 == 0 => {
                    Step::Forward(forward(Contraption::Horizontal))
                }
                Contraption::Vertical if direction % 2 == 1 => {
                    Step::Forward(forward(Contraption::Vertical))
                }
                _ => Step::Split(
                    forward(Contraption::Clockwise),
                    forward(Contraption::CounterClockwise),
                ),
            }
        } else {
            // Find next node hit by beam
            let hit_position = self
                .line(beam, (self.boundary(node_position, direction), direction))
                .find_or_last(|p| self.nodes[*p].is_some())
                .unwrap();
            Step::Forward((hit_position, direction))
        }
    }

    fn line(&self, origin_beam: Beam, target_beam: Beam) -> impl Iterator<Item = Position> {
        let (origin, _) = origin_beam;
        let (target, input_direction) = target_beam;
        debug_assert!({
            let vertical = input_direction % 2 == 1;
            if vertical {
                (target % self.size) == (origin % self.size)
            } else {
                (target / self.size) == (origin / self.size)
            }
        });
        match input_direction {
            d @ (RIGHT | LEFT) => Either::Left(match d {
                RIGHT => Either::Left(origin..=target),
                _ => Either::Right((target..=origin).rev()),
            }),
            d => Either::Right(match d {
                UP => Either::Left((target..=origin).rev().step_by(self.size)),
                _ => Either::Right((origin..=target).step_by(self.size)),
            }),
        }
        .into_iter()
    }
}

fn energize_with_beam(grid: &ContraptionNetwork, start_beam: Beam) -> usize {
    let mut energized = bitvec!(0; grid.size * grid.size);
    let mut beams: VecDeque<Beam> = [start_beam].into_iter().collect();
    let mut visited_beams: HashSet<Beam> = HashSet::new();
    let mut energize = |beam, next_beam| {
        let mut count = 0;
        for p in grid.line(beam, next_beam) {
            energized.set(p, true);
            count += 1;
        }
        // `next_beam` and `beam` must have distinct positions.
        // (This is not the case if the line is a single dot.)
        count > 1
    };
    while let Some(beam) = beams.pop_back() {
        if visited_beams.insert(beam) {
            match grid.step(beam) {
                Step::Forward(next_beam) => {
                    if energize(beam, next_beam) {
                        beams.push_front(next_beam);
                    }
                }
                Step::Split(next_beam1, next_beam2) => {
                    if energize(beam, next_beam1) {
                        beams.push_front(next_beam1);
                    }
                    if energize(beam, next_beam2) {
                        beams.push_front(next_beam2);
                    }
                }
            }
        }
    }
    /*
    for row in 0..grid.size {
        println!(
            "{}",
            grid.nodes[row * grid.size..(row + 1) * grid.size]
                .iter()
                .map(|n| if let Some(node) = n {
                    match node.contraption {
                        Contraption::CounterClockwise => '/',
                        Contraption::Clockwise => '\\',
                        Contraption::Horizontal => '-',
                        Contraption::Vertical => '|',
                    }
                } else {
                    '.'
                })
                .collect::<String>()
        );
    }
    for row in 0..grid.size {
        println!(
            "{}",
            energized[row * grid.size..(row + 1) * grid.size]
                .into_iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        );
    }
    */
    energized.count_ones()
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let grid = ContraptionNetwork::parse(input);
    if part2 {
        let e = grid.size - 1;
        [
            (0..=0, 0..=e, DOWN),
            (e..=e, 0..=e, UP),
            (1..=e - 1, 0..=0, RIGHT),
            (1..=e - 1, e..=e, LEFT),
        ]
        .into_iter()
        .flat_map(|(rows, cols, dir)| {
            rows.cartesian_product(cols)
                .map(move |(r, c)| (position(r, c, grid.size), dir))
        })
        .inspect(|v| println!("{v:?}"))
        .map(|(p, dir)| energize_with_beam(&grid, (p, dir)))
        .inspect(|value| println!("{value}"))
        .max()
        .unwrap()
    } else {
        energize_with_beam(&grid, (0, RIGHT))
    }
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rotate() {
        // "-" / "|"
        assert_eq!(rotate(RIGHT, Contraption::Horizontal), RIGHT);
        assert_eq!(rotate(UP, Contraption::Vertical), UP);
        assert_eq!(rotate(LEFT, Contraption::Horizontal), LEFT);
        assert_eq!(rotate(DOWN, Contraption::Vertical), DOWN);

        // "/"
        assert_eq!(rotate(RIGHT, Contraption::CounterClockwise), UP);
        assert_eq!(rotate(UP, Contraption::CounterClockwise), RIGHT);
        assert_eq!(rotate(LEFT, Contraption::CounterClockwise), DOWN);
        assert_eq!(rotate(DOWN, Contraption::CounterClockwise), LEFT);

        // "\"
        assert_eq!(rotate(RIGHT, Contraption::Clockwise), DOWN);
        assert_eq!(rotate(UP, Contraption::Clockwise), LEFT);
        assert_eq!(rotate(LEFT, Contraption::Clockwise), UP);
        assert_eq!(rotate(DOWN, Contraption::Clockwise), RIGHT);
    }

    #[test]
    fn test_run() {
        let test_input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(run(test_input.into(), false), 46);
        assert_eq!(run(test_input.into(), true), 51);
    }
}
