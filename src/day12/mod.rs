use itertools::Itertools;

use crate::PuzzleInput;

#[derive(Debug, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
}

/// Find all ending indices of a run of Damaged condition with length `run_length`
fn run_endings(
    conditions: &[Option<Condition>],
    run_length: usize,
) -> impl Iterator<Item = usize> + '_ {
    let last_start_index = conditions.len() - run_length;
    let max_start_index = conditions
        .iter()
        .take(last_start_index)
        .position(|c| matches!(c, Some(Condition::Damaged)))
        .unwrap_or(last_start_index);
    (0..max_start_index + 1)
        .filter(move |start| {
            // Run must end with Unknown or Operational (or end the sequence)
            !matches!(conditions.get(*start + run_length), Some(Some(Condition::Damaged))) &&
            // Run must only contain Unknown and Damaged
            conditions[*start..*start + run_length]
                .iter()
                .all(|c| !matches!(c, Some(Condition::Operational)))
        })
        .map(move |s| s + run_length)
}

fn parse(line: &str, repeats: usize) -> (Vec<Option<Condition>>, Vec<usize>) {
    let (conditions_string, nums_str) = line.splitn(2, ' ').collect_tuple().unwrap();
    let numbers = nums_str
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec()
        .repeat(repeats);
    let conditions = {
        let mut c = conditions_string
            .chars()
            .map(|c| match c {
                '.' => Some(Condition::Operational),
                '#' => Some(Condition::Damaged),
                _ => None,
            })
            .chain([None])
            .collect_vec()
            .repeat(repeats);
        c.pop();
        c
    };
    (conditions, numbers)
}
fn arrangements(line: &str, part2: bool) -> usize {
    let (conditions, run_lengths) = parse(line, if part2 { 5 } else { 1 });

    // All damaged runs have an 1-sized tail (except the last one)
    let total_run_length = run_lengths.iter().sum::<usize>() + run_lengths.len() - 1;
    let remaining_run_lengths = run_lengths
        .iter()
        .scan(total_run_length, |remaining, run_length| {
            if remaining == run_length {
                Some(0)
            } else {
                *remaining -= run_length + 1;
                Some(*remaining)
            }
        })
        .collect_vec();

    // Start with a run of Damaged which ends at index 0
    let empty_run_ending_counts = [0_usize].repeat(conditions.len() + 2);
    let mut run_ending_counts = empty_run_ending_counts.clone();
    run_ending_counts[0] = 1;
    for (run_length, remaining) in run_lengths.into_iter().zip_eq(remaining_run_lengths) {
        let mut next_run_ending_counts = empty_run_ending_counts.clone();
        // Count run endings
        for (start, count) in run_ending_counts
            .into_iter()
            .enumerate()
            .filter(|(start, count)| {
                *count > 0 &&
                // Discard arrangement counts if run_length (+ tail) does not fit in remaining space
                run_length + if remaining == 0 { 0 } else { remaining + 1 }
                    <= conditions.len() - start
            })
            .flat_map(|(start, count)| {
                run_endings(&conditions[start..conditions.len() - remaining], run_length)
                    // Index after tail (e.g. at '?' for '###.?') becomes next start
                    // Each ending can be reached by `count` combinations
                    .map(move |e| (start + e + 1, count))
            })
        {
            // Sum combination counts with the same ending
            next_run_ending_counts[start] += count;
        }
        run_ending_counts = next_run_ending_counts;
    }
    // Ensure that all arrangements include the last Damaged
    let lowest_ending = conditions.len()
        - conditions
            .iter()
            .rev()
            .position(|c| matches!(c, Some(Condition::Damaged)))
            .unwrap_or(conditions.len());

    run_ending_counts
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i >= lowest_ending)
        .map(|(_, c)| c)
        .sum()
}

fn run(input: PuzzleInput, part2: bool) -> usize {
    input.map(|line| arrangements(&line, part2)).sum()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let lines = test_input.lines().collect_vec();

        assert_eq!(arrangements(lines[0], false), 1);
        assert_eq!(arrangements(lines[1], false), 4);
        assert_eq!(arrangements(lines[2], false), 1);
        assert_eq!(arrangements(lines[3], false), 1);
        assert_eq!(arrangements(lines[4], false), 4);
        assert_eq!(arrangements(lines[5], false), 10);
        assert_eq!(run(test_input.into(), false), 21);

        assert_eq!(arrangements(lines[0], true), 1);
        assert_eq!(arrangements(lines[1], true), 16384);
        assert_eq!(arrangements(lines[2], true), 1);
        assert_eq!(arrangements(lines[3], true), 16);
        assert_eq!(arrangements(lines[4], true), 2500);
        assert_eq!(arrangements(lines[5], true), 506250);
        assert_eq!(run(test_input.into(), true), 525152);
    }
}
