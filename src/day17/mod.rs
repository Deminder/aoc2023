use std::collections::{BinaryHeap, HashMap};

use either::Either;
use itertools::Itertools;
use num_integer::Roots;

use crate::PuzzleInput;

type Position = usize;

type Direction = u8;
type HeatGrid = Vec<u8>;

const RIGHT: Direction = 0;
const UP: Direction = 1;
const LEFT: Direction = 2;
const DOWN: Direction = 3;

fn left(d: Direction) -> Direction {
    (d + 1) % 4
}

fn right(d: Direction) -> Direction {
    (d + 3) % 4
}

fn step(step: usize, direction: Direction, position: Position, size: usize) -> Option<Position> {
    let (row, col) = num_integer::div_rem(position, size);
    if match direction {
        RIGHT => col + step <= size - 1,
        UP => row >= step,
        LEFT => col >= step,
        _ => row + step <= size - 1,
    } {
        Some(match direction {
            RIGHT => position + step,
            UP => position - (step * size),
            LEFT => position - step,
            _ => position + (step * size),
        })
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Crucible {
    direction: Direction,
    position: Position,
}

impl Crucible {
    fn maneuver(&self, size: usize, ultra: bool) -> impl Iterator<Item = Self> + '_ {
        let (min_steps, max_steps) = if ultra { (4, 10) } else { (1, 3) };
        [left(self.direction), right(self.direction)]
            .into_iter()
            .flat_map(move |direction| {
                (min_steps..=max_steps).filter_map(move |s| {
                    step(s, direction, self.position, size).map(|position| Crucible {
                        direction,
                        position,
                    })
                })
            })
    }

    fn line(&self, next_crucible: &Crucible, size: usize) -> impl Iterator<Item = Position> {
        match next_crucible.direction {
            RIGHT => Either::Left(self.position + 1..=next_crucible.position),
            UP => Either::Right((next_crucible.position..=self.position - size).step_by(size)),
            LEFT => Either::Left(next_crucible.position..=self.position - 1),
            _ => Either::Right((self.position + size..=next_crucible.position).step_by(size)),
        }
        .into_iter()
    }
}

#[derive(Debug)]
struct State {
    crucible: Crucible,
    heat: usize,
    distance: usize,
    vertical: bool,
}

fn goal_distance(position: Position, size: usize) -> usize {
    let (row, col) = num_integer::div_rem(position, size);
    let goal = size - 1;
    (goal - row) + (goal - col)
}
impl State {
    fn new(crucible: Crucible, heat: usize, size: usize) -> Self {
        Self {
            heat,
            distance: goal_distance(crucible.position, size),
            vertical: crucible.direction % 2 == 1,
            crucible,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.heat == other.heat
            && self.crucible.position == other.crucible.position
            && self.vertical == other.vertical
    }
}
impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.heat + self.distance)
            .cmp(&(other.heat + other.distance))
            .reverse()
            .then_with(|| self.vertical.cmp(&other.vertical))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let heat_grid: HeatGrid = input
        .flat_map(|line| {
            line.chars()
                .map(move |c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect();
    let size = heat_grid.len().sqrt();

    // Approach: Find the shortest/coolest path on a weighted graph/heat-map
    let mut min_heat_by_crucible: HashMap<Crucible, usize> = HashMap::new();
    // Start in top-left with goal at bottom-right
    let mut frontier: BinaryHeap<State> = [RIGHT, DOWN]
        .into_iter()
        .map(|direction| Crucible {
            position: 0,
            direction,
        })
        .map(|crucible| State::new(crucible, 0, size))
        .collect();
    while let Some(State {
        crucible,
        heat,
        distance,
        vertical: _,
    }) = frontier.pop()
    {
        if min_heat_by_crucible
            .get(&crucible)
            .is_some_and(|min_heat| heat >= *min_heat)
        {
            // Do not re-walk non-imporoving heat-paths
            continue;
        } else {
            min_heat_by_crucible.insert(crucible.clone(), heat);
        }

        if distance == 0 {
            // Min-Heat path has been found.
            // (This requires no 0-heat paths, i.e., all `heat_grid` numbers must be >= 1)
            break;
        }

        for next_crucible in crucible.maneuver(size, part2) {
            let next_heat = heat
                + crucible
                    .line(&next_crucible, size)
                    .map(|p| heat_grid[p])
                    .sum::<u8>() as usize;
            if !min_heat_by_crucible
                .get(&next_crucible)
                .is_some_and(|min_heat| next_heat >= *min_heat)
            {
                // Add candidate with `next_crucible` to frontier
                frontier.push(State::new(next_crucible, next_heat, size));
            }
        }
        if cfg!(test) {
            // Print state of search
            println!("Frontier: {}", frontier.len());
            for row in 0..size {
                let overview = (0..size)
                    .map(|col| {
                        [RIGHT, UP, LEFT, DOWN]
                            .into_iter()
                            .filter_map(|d| {
                                min_heat_by_crucible.get(&Crucible {
                                    position: row * size + col,
                                    direction: d,
                                })
                            })
                            .min()
                            .map(|n| n.to_string())
                            .unwrap_or("?".into())
                    })
                    .map(|s| format!("{:3}", s))
                    .collect_vec();
                println!("{:?}", overview);
            }
        }
    }

    // Get minimum heat for bottom-right goal
    [RIGHT, DOWN]
        .into_iter()
        .filter_map(|direction| {
            min_heat_by_crucible.get(&Crucible {
                position: (size * size) - 1,
                direction,
            })
        })
        .min()
        .copied()
        .unwrap()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        assert_eq!(run(test_input.into(), false), 102);
        assert_eq!(run(test_input.into(), true), 94);
    }
}
