use either::Either;
use itertools::Itertools;

use crate::PuzzleInput;

fn next_num_in_sequence(sequence: &[i32], part2: bool) -> i32 {
    let n = sequence.len();
    // Example: n = 4
    // x = (4 * a_4) - (6 * a_3) + (4 * a_2) + (a_1)
    if part2 {
        Either::Left(sequence.iter())
    } else {
        Either::Right(sequence.iter().rev())
    }
    .enumerate()
    .map(|(km1, val)| {
        (
            if km1 % 2 == 0 { 1 } else { -1 },
            num_integer::binomial(n, km1 + 1) as i32,
            val,
        )
    })
    .map(|(sign, coeff, val)| sign * coeff * val)
    .sum()
}

fn run(input: PuzzleInput, part2: bool) -> i32 {
    input
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|sequence| next_num_in_sequence(&sequence, part2))
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
        let test_input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(run(test_input.into(), false), 114);
        assert_eq!(run(test_input.into(), true), 2);
    }
}
