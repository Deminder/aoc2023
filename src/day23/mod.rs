use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

use itertools::Itertools;
use num_integer::{Integer, Roots};

use crate::PuzzleInput;

type Position = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Direction {
    fn backward(&self, d: &Direction) -> bool {
        match self {
            Direction::Right => matches!(d, Direction::Left),
            Direction::Up => matches!(d, Direction::Down),
            Direction::Left => matches!(d, Direction::Right),
            Direction::Down => matches!(d, Direction::Up),
        }
    }
    fn forward(&self) -> impl Iterator<Item = Direction> + '_ {
        DIRS.into_iter().filter(|d| !self.backward(d))
    }
}

#[derive(Debug)]
enum Slot {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Slot {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::Slope(Direction::Right),
            '^' => Self::Slope(Direction::Up),
            '<' => Self::Slope(Direction::Left),
            'v' => Self::Slope(Direction::Down),
            _ => panic!("unexpected slot: {}", value),
        }
    }
}

type Edge = (Position, Position);

#[derive(Debug, Clone)]
struct PathCandidate {
    head: Position,
    tail: HashSet<Position>,
    weight: usize,
    residual_edge_weights: Rc<HashMap<Edge, usize>>,
}

impl PathCandidate {
    fn new(head: usize, edge_weights: Rc<HashMap<Edge, usize>>) -> Self {
        Self {
            head,
            tail: HashSet::new(),
            weight: 0,
            residual_edge_weights: edge_weights.clone(),
        }
    }

    fn upper_bound_length(&self) -> usize {
        self.weight + self.residual_edge_weights.values().sum::<usize>()
    }

    fn advance(&self, weight_edges: Vec<(Position, usize)>) -> impl Iterator<Item = Self> + '_ {
        let residual_edge_weights = {
            let mut weights = (*self.residual_edge_weights).clone();
            for w in weight_edges.iter() {
                weights.remove(w);
            }
            Rc::new(weights)
        };

        weight_edges.into_iter().filter_map(move |(head, weight)| {
            let mut tail = self.tail.clone();
            if tail.insert(head) {
                Some(Self {
                    head,
                    tail,
                    weight: self.weight + weight,
                    residual_edge_weights: residual_edge_weights.clone(),
                })
            } else {
                None
            }
        })
    }
}

impl Ord for PathCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight
            .cmp(&other.weight)
            .then_with(|| self.head.cmp(&other.head))
    }
}

impl PartialOrd for PathCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.upper_bound_length() == other.upper_bound_length() && self.head == other.head
    }
}
impl Eq for PathCandidate {}

struct Graph {
    edges: HashMap<Position, Vec<Position>>,
    edge_weights: HashMap<Edge, usize>,
    start_pos: Position,
    end_pos: Position,
}

impl Graph {
    fn parse(lines: impl Iterator<Item = String>, dry: bool) -> Self {
        let mut slots = vec![];
        for line in lines {
            slots.extend(line.chars().map(|c| c.into()));
        }
        let size = slots.len().sqrt();

        let start_pos = 1;
        let end_pos = size * size - 2;
        let mut edges: HashMap<Position, Vec<Position>> = HashMap::new();
        let mut edge_weights: HashMap<Edge, usize> = HashMap::new();
        let mut add_edge = |p1: Position, p2: Position, weight: usize| {
            let edge = (p1.min(p2), p1.max(p2));

            let old_weight = edge_weights.get(&edge).unwrap_or(&0);
            let longer = old_weight < &weight;
            if longer && edge_weights.insert(edge, weight).is_none() {
                edges
                    .entry(p1)
                    .and_modify(|ends| ends.push(p2))
                    .or_insert_with(|| vec![p2]);
                if dry {
                    // Add backwards edge for undirected graph
                    edges
                        .entry(p2)
                        .and_modify(|ends| ends.push(p1))
                        .or_insert_with(|| vec![p1]);
                }
            }
            longer
        };
        let mut candidates: VecDeque<(Position, Direction)> =
            [(start_pos, Direction::Down)].into_iter().collect();
        while let Some((edge_start, first_direction)) = candidates.pop_front() {
            // Find next heads
            let advance = |d, p| {
                let next_end = match d {
                    Direction::Right => p + 1,
                    Direction::Up => p - size,
                    Direction::Left => p - 1,
                    Direction::Down => p + size,
                };
                if match &slots[next_end] {
                    Slot::Path => true,
                    Slot::Forest => false,
                    Slot::Slope(sd) => dry || &d == sd,
                } {
                    Some((next_end, d))
                } else {
                    None
                }
            };
            let (mut pos, mut from_direction) = advance(first_direction, edge_start).unwrap();
            let mut step = 1;
            loop {
                let heads = from_direction
                    .forward()
                    .filter_map(|d| advance(d, pos))
                    .collect_vec();
                if heads.len() == 1 {
                    step += 1;
                    (pos, from_direction) = heads[0];
                }
                if pos == start_pos {
                    break;
                }

                assert!(
                    !heads.is_empty(),
                    "should have no dead ends from {:?} at {:?}",
                    edge_start.div_rem(&size),
                    pos.div_rem(&size),
                );

                if pos == end_pos || heads.len() > 1 {
                    if add_edge(edge_start, pos, step) && pos != end_pos {
                        candidates.extend(heads.into_iter().map(|(_, dir)| (pos, dir)));
                    }
                    break;
                }
            }
        }
        Self {
            start_pos,
            edges,
            edge_weights,
            end_pos,
        }
    }

    fn advance_candidate<'a>(
        &'a self,
        cand: &'a PathCandidate,
    ) -> impl Iterator<Item = PathCandidate> + 'a {
        cand.advance(
            self.edges[&cand.head]
                .iter()
                .map(|end| {
                    let edge = (cand.head.min(*end), cand.head.max(*end));
                    (*end, self.edge_weights[&edge])
                })
                .collect_vec(),
        )
    }

    // Get longest path length of an acyclic graph
    fn longest_greedy_path_length(&self, initial: &PathCandidate) -> Option<usize> {
        if initial.head != self.end_pos {
            let mut candidates: BinaryHeap<PathCandidate> = [initial.clone()].into_iter().collect();
            while let Some(cand) = candidates.pop() {
                if cand.head == self.end_pos {
                    return Some(cand.weight);
                }
                candidates.extend(self.advance_candidate(&cand));
            }
            None
        } else {
            Some(initial.weight)
        }
    }

    fn upper_bound_path_length(&self, initial: &PathCandidate) -> usize {
        if initial.tail.len() > initial.residual_edge_weights.len() {
            // Path length by sum of all edges which can reach the end
            let mut visits = vec![initial.head];
            let mut reachable_edges: HashSet<Edge> = HashSet::new();
            while let Some(visit) = visits.pop() {
                visits.extend(
                    self.edges[&visit]
                        .iter()
                        .filter(|&&end| {
                            let edge = (visit.min(end), visit.max(end));
                            initial.residual_edge_weights.contains_key(&edge)
                                && reachable_edges.insert(edge)
                        })
                        .filter(|&&end| end != self.end_pos),
                );
            }
            reachable_edges
                .into_iter()
                .map(|edge| self.edge_weights[&edge])
                .sum::<usize>()
        } else {
            // Early in the search:
            // Length as if all residual edges are reachable
            initial.upper_bound_length()
        }
    }

    /// Exhaustively search for longest paths:
    /// Visiting best greedy paths first.
    /// (For polynomial-time solution https://en.wikipedia.org/wiki/Longest_path_problem
    /// says it has to be a specific graph, possibly, a cactus graph?)
    fn longest_path_length(&self) -> usize {
        let mut candidates: BinaryHeap<(usize, usize, PathCandidate)> = [(
            usize::MIN,
            usize::MAX,
            PathCandidate::new(self.start_pos, Rc::new(self.edge_weights.clone())),
        )]
        .into_iter()
        .collect();

        let mut longest_known_path_length = 0;
        while let Some((_, upper_bound, cand)) = candidates.pop() {
            if cand.head == self.end_pos || longest_known_path_length >= upper_bound {
                continue;
            }
            let next_candidates = self
                .advance_candidate(&cand)
                .filter_map(|cand| {
                    self.longest_greedy_path_length(&cand)
                        .map(|length| (cand, length))
                })
                .map(|(cand, greedy_length)| {
                    longest_known_path_length = longest_known_path_length.max(greedy_length);
                    (greedy_length, self.upper_bound_path_length(&cand), cand)
                })
                .filter(|(lower_bound, upper_bound, _)| lower_bound < upper_bound);
            candidates.extend(next_candidates);
        }
        longest_known_path_length
    }
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let graph = Graph::parse(input, part2);
    graph.longest_path_length()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
        assert_eq!(run(test_input.into(), false), 94);
        assert_eq!(run(test_input.into(), true), 154);
    }
}
