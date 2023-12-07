use std::cmp::Ordering;

use itertools::Itertools;

use crate::PuzzleInput;

const CARDS: usize = 5;

fn char_to_strength(c: char) -> u8 {
    ['T', 'J', 'Q', 'K', 'A']
        .into_iter()
        .position(|t| t == c)
        .map_or_else(|| c.to_digit(10).unwrap() as u8 - 2, |p| p as u8 + 8)
}

fn char_to_joke_strength(c: char) -> u8 {
    ['T', 'Q', 'K', 'A']
        .into_iter()
        .position(|t| t == c)
        .map_or_else(|| c.to_digit(10).unwrap_or(1) as u8 - 1, |p| p as u8 + 9)
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    card_strengths: [u8; CARDS],
    hand_type: u8,
}

impl Hand {
    fn parse(hand: &str, part2: bool) -> Self {
        let card_strengths = array_init::from_iter(hand.chars()).unwrap().map(if part2 {
            char_to_joke_strength
        } else {
            char_to_strength
        });
        Self {
            hand_type: Self::strengths_to_type(&card_strengths, part2),
            card_strengths,
        }
    }

    fn strengths_to_type(card_strengths: &[u8; CARDS], joker_wildcard: bool) -> u8 {
        let mut strength_counts = [0 as u8; 13];
        for strength in card_strengths {
            strength_counts[*strength as usize] += 1;
        }
        let wildcard_count = if joker_wildcard {
            let jokers = strength_counts[0];
            strength_counts[0] = 0;
            jokers
        } else {
            0
        };
        let inverse = |c| CARDS as u8 - c;
        strength_counts
            .into_iter()
            .map(inverse)
            .k_smallest(2)
            .map(inverse)
            .enumerate()
            .map(|(i, c)| if i == 0 { c + wildcard_count } else { c })
            .map(|c| c * c)
            .sum()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(|| {
            self.card_strengths
                .iter()
                .zip_eq(other.card_strengths)
                .map(|(a, b)| a.cmp(&b))
                .find(|o| o.is_ne())
                .unwrap_or(Ordering::Equal)
        })
    }
}

fn run(input: PuzzleInput, part2: bool) -> u32 {
    input
        .flat_map(|line| {
            line.splitn(2, ' ')
                .collect_tuple()
                .map(|(hand, bet)| (Hand::parse(hand, part2), bet.parse::<u32>().unwrap()))
        })
        .sorted_by(|(a, _), (b, _)| a.cmp(&b))
        .enumerate()
        //.inspect(|(rank, (hand, bet))| println!("{rank}: {hand:?} {bet}"))
        .map(|(rank, (_, bet))| (rank as u32 + 1) * bet)
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
        let test_input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(run(test_input.into(), false), 6440);
        assert_eq!(run(test_input.into(), true), 5905);
    }
}
