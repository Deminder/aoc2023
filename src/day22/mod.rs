use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

use crate::PuzzleInput;

#[derive(Debug, Clone)]
struct Brick {
    lateral: Range<usize>,
    colateral: Range<usize>,
    elevation: Range<usize>,
}

impl Brick {
    fn parse(line: &str) -> Self {
        let ((x, y, z), (x2, y2, z2)) = line
            .splitn(2, '~')
            .map(|s| {
                s.split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Self {
            lateral: x..x2 + 1,
            colateral: y..y2 + 1,
            elevation: z..z2 + 1,
        }
    }

    fn height(&self) -> usize {
        self.elevation.end - self.elevation.start
    }

    fn laterals(&self) -> impl Iterator<Item = (usize, usize)> {
        self.lateral
            .clone()
            .cartesian_product(self.colateral.clone())
    }
}

fn foundations_of_brick(bricks: &[Brick]) -> Vec<HashSet<usize>> {
    let bricks_sorted = bricks
        .iter()
        .enumerate()
        .sorted_by_key(|(_, b)| b.elevation.start);

    let lateral_max = bricks
        .iter()
        .flat_map(|b| {
            [
                b.colateral.start,
                b.colateral.end,
                b.lateral.start,
                b.lateral.end,
            ]
        })
        .max()
        .unwrap();
    let idx = |(colat, lat)| colat * lateral_max + lat;
    let mut elevation_map = [0].repeat(lateral_max * lateral_max);
    let mut brick_map: Vec<Option<usize>> = (0..elevation_map.len()).map(|_| None).collect();
    let mut foundations: Vec<HashSet<usize>> = (0..bricks.len()).map(|_| HashSet::new()).collect();
    for (index, brick) in bricks_sorted {
        let height = brick.height();
        let elevation = brick
            .laterals()
            .map(|p| elevation_map[idx(p)])
            .max()
            .unwrap();
        for map_idx in brick.laterals().map(idx) {
            let map_elevation = &mut elevation_map[map_idx];
            let map_brick = &mut brick_map[map_idx];

            if *map_elevation == elevation {
                if let Some(foundation_index) = *map_brick {
                    foundations[index].insert(foundation_index);
                }
            }
            *map_brick = Some(index);
            *map_elevation = elevation + height;
        }
    }
    foundations
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let bricks = input.map(|line| Brick::parse(&line)).collect_vec();

    let foundations = foundations_of_brick(&bricks);
    let mut brick_loads = (0..foundations.len()).map(|_| vec![]).collect_vec();
    for (index, foundations) in foundations.iter().enumerate() {
        for foundation in foundations {
            brick_loads[*foundation].push(index);
        }
    }
    if part2 {
        brick_loads
            .iter()
            .enumerate()
            .map(|(start_brick, loads)| {
                // Find bricks which fall/(get removed) when removing the start brick
                let mut removed_bricks: HashSet<usize> = [start_brick].into_iter().collect();
                let mut stack = loads.clone();
                while let Some(brick) = stack.pop() {
                    // A brick falls if it has no foundation
                    let brick_falls = foundations[brick]
                        .iter()
                        .all(|f| removed_bricks.contains(f));
                    if brick_falls && removed_bricks.insert(brick) {
                        stack.extend(&brick_loads[brick]);
                    }
                }
                removed_bricks.len() - 1
            })
            .sum()
    } else {
        // A brick can be removed if all its loads have more than one foundation
        brick_loads
            .into_iter()
            .filter(|loads| loads.iter().all(|&l| foundations[l].len() > 1))
            .count()
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
        let test_input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(run(test_input.into(), false), 5);
        assert_eq!(run(test_input.into(), true), 7);
    }
}
