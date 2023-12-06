use std::ops::Range;

use itertools::Itertools;

use crate::PuzzleInput;

/// When pressing the button for t time units the velocity increases linearly: v(t) = t
/// Distance(t) = v(t) * (race_duration - t)
fn record_beating_range(race_duration: f64, record_distance: f64) -> Option<Range<u64>> {
    let sqrt_part = ((race_duration * race_duration / 4.0) - record_distance).sqrt();
    if sqrt_part.is_nan() {
        None
    } else {
        let t2 = race_duration / 2.0;
        let start = 0.max((t2 - sqrt_part).trunc() as i32 + 1) as u64;
        let end = 0.max((t2 + sqrt_part).ceil() as u64);
        Some(start..end)
    }
}

fn run(input: PuzzleInput, part2: bool) -> u64 {
    let (time, distance) = input
        .map(|line| {
            let (_, num_list) = line.splitn(2, ':').collect_tuple().unwrap();
            if part2 {
                vec![num_list
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()]
            } else {
                num_list
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect_vec()
            }
        })
        .collect_tuple()
        .unwrap();
    time.into_iter()
        .zip(distance)
        .flat_map(|(t, d)| record_beating_range(t as f64, d as f64))
        // Number of ways to beat the record
        .map(|r| r.end - r.start)
        .product()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(run(test_input.into(), false), 288);
        assert_eq!(run(test_input.into(), true), 71503);
    }
}
