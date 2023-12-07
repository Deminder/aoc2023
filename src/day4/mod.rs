use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::PuzzleInput;

struct Card {
    winning_numbers: HashSet<u8>,
    scratch_numbers: HashSet<u8>,
}

impl Card {
    fn new(line: &str) -> Self {
        let (_, numbers) = line.splitn(2, ':').collect_tuple().unwrap();
        let (w, s) = numbers
            .splitn(2, '|')
            .map(|num_list| {
                num_list
                    .split(' ')
                    .filter_map(|num| num.trim().parse::<u8>().ok())
                    .collect()
            })
            .collect_tuple()
            .unwrap();

        Self {
            winning_numbers: w,
            scratch_numbers: s,
        }
    }

    fn match_count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.scratch_numbers)
            .count()
    }
}

fn run(input: PuzzleInput, part2: bool) -> u32 {
    let match_counts = input
        .filter(|line| !line.is_empty())
        .map(|line| Card::new(&line).match_count());

    if part2 {
        match_counts
            .scan(VecDeque::new(), |copies, match_count| {
                let card_count = 1 + copies.pop_front().unwrap_or(0);
                let queue_len = copies.len();
                for offset in 0..match_count {
                    if offset < queue_len {
                        copies[offset] += card_count;
                    } else {
                        copies.push_back(card_count);
                    }
                }
                Some(card_count)
            })
            .sum()
    } else {
        match_counts.filter(|c| *c > 0).map(|c| 1 << (c - 1)).sum()
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
        let test_input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(run(test_input.into(), false), 13);
        assert_eq!(run(test_input.into(), true), 30);
    }
}
