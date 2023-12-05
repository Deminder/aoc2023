use std::ops::Range;

use itertools::Itertools;

use crate::PuzzleInput;

fn range_intersect(range1: &Range<u64>, range2: &Range<u64>) -> Option<Range<u64>> {
    let max_start = range1.start.max(range2.start);
    let min_end = range1.end.min(range2.end);
    if max_start < min_end {
        Some(max_start..min_end)
    } else {
        None
    }
}

#[derive(Debug)]
struct RangeMapping {
    ranges: Vec<(Range<u64>, u64)>,
}

impl RangeMapping {
    fn new<T>(lines: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        Self {
            ranges: lines
                .into_iter()
                .filter_map(|line| {
                    line.splitn(3, ' ')
                        .filter_map(|n| n.parse().ok())
                        .collect_tuple()
                })
                .map(|(destination, start, len)| (start..start + len, destination))
                .collect_vec(),
        }
    }

    fn transform(&self, range: Range<u64>) -> impl Iterator<Item = Range<u64>> {
        self.ranges
            .iter()
            .filter_map(|(r, destination)| {
                range_intersect(r, &range).map(|i| {
                    let intersect_offset = i.start - r.start;
                    (i, *destination, intersect_offset)
                })
            })
            .sorted_by_key(|(i, _, _)| i.start)
            // Transform intersecting ranges, otherwise leave unchanged
            // Each intersecting range produces an unchanged and transformed range
            // A dummy intersection ensures that the last unchanged region is also included:
            .chain([(range.end..range.end, 0, 0)].into_iter())
            .scan(
                range.start,
                |unchanged_start, (intersection, destination, intersect_offset)| {
                    let unchanged_range = *unchanged_start..intersection.start;
                    *unchanged_start = intersection.end;
                    let mapped_start = intersect_offset + destination;
                    Some(
                        [
                            unchanged_range,
                            mapped_start..mapped_start + (intersection.end - intersection.start),
                        ]
                        .into_iter(),
                    )
                },
            )
            .flatten()
            // Filter out empty ranges
            .filter(|r| r.start < r.end)
    }
}

fn run(mut input: PuzzleInput, part2: bool) -> u64 {
    let first_line = input.next().unwrap();
    let (_, seeds_list) = first_line.splitn(2, ':').collect_tuple().unwrap();
    let seed_nums = seeds_list
        .split(' ')
        .filter_map(|num| num.parse::<u64>().ok());

    let seeds = if part2 {
        seed_nums
            .chunks(2)
            .into_iter()
            .filter_map(|nums| nums.collect_tuple())
            .map(|(start, len)| start..start + len)
            .collect_vec()
    } else {
        seed_nums.map(|start| start..start + 1).collect_vec()
    };

    let ranges = input
        .scan(0, |step, line| {
            if line.ends_with("map:") {
                *step += 1;
            }
            Some((*step, line))
        })
        .group_by(|(step, _)| *step)
        .into_iter()
        .filter(|(step, _)| *step > 0)
        .map(|(_, lines)| RangeMapping::new(lines.map(|(_, l)| l)))
        .collect_vec();

    let locations = ranges
        .into_iter()
        .fold(seeds, |mapped_values, range_mapping| {
            let values = mapped_values
                .into_iter()
                .flat_map(|range| range_mapping.transform(range))
                .collect_vec();
            values
        });
    locations.into_iter().map(|r| r.start).min().unwrap()
}

pub fn solution(input: PuzzleInput, part2: bool) -> String {
    run(input, part2).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let test_input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(run(test_input.into(), false), 35);
        assert_eq!(run(test_input.into(), true), 46);
    }
}