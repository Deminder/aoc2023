use std::collections::HashMap;

use itertools::Itertools;

use crate::PuzzleInput;

struct Bag(HashMap<String, usize>);

impl Bag {
    fn new(list: &str) -> Self {
        Bag(list
            .split(',')
            .map(|c| {
                let (count, cube_type) = c.trim().splitn(2, ' ').collect_tuple().unwrap();
                (cube_type.into(), count.parse().unwrap())
            })
            .collect())
    }

    fn subset_of(&self, bag: &Bag) -> bool {
        self.0.iter().all(|(k, v)| v <= bag.0.get(k).unwrap_or(&0))
    }

    fn grow_to(&self, bag: &Bag) -> Bag {
        Self(
            self.0
                .keys()
                .chain(bag.0.keys())
                .unique()
                .map(|k| {
                    (
                        k.clone(),
                        *self.0.get(k).unwrap_or(&0).max(bag.0.get(k).unwrap_or(&0)),
                    )
                })
                .collect(),
        )
    }

    fn power(&self) -> usize {
        self.0.values().product()
    }
}

struct Game {
    id: usize,
    minimum_bag: Bag,
}

impl Game {
    fn new(line: &str) -> Self {
        let (label, listing) = line.splitn(2, ":").collect_tuple().unwrap();
        let hands = listing.split(';').map(|r| Bag::new(r));
        Self {
            id: label.splitn(2, ' ').last().unwrap().parse().unwrap(),
            minimum_bag: hands.reduce(|min_bag, bag| min_bag.grow_to(&bag)).unwrap(),
        }
    }

    fn possible_by(&self, bag: &Bag) -> bool {
        self.minimum_bag.subset_of(bag)
    }
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    let games = input
        .filter(|line| !line.is_empty())
        .map(|line| Game::new(&line));
    if part2 {
        games.map(|g| g.minimum_bag.power()).sum()
    } else {
        let bag = Bag::new("12 red, 13 green, 14 blue");
        games
            .filter(|game| game.possible_by(&bag))
            .map(|game| game.id)
            .sum()
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
        let test_input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(run(test_input.into(), false), 8);
        assert_eq!(run(test_input.into(), true), 2286);
    }
}
