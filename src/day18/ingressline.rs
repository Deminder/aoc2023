use std::ops::Range;

use crate::range_intersect;

#[derive(Debug)]
pub struct IngressLineIdx {
    pub row: i32,
    pub col: i32,
}

impl PartialEq for IngressLineIdx {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}
impl Eq for IngressLineIdx {}

impl Ord for IngressLineIdx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row
            .cmp(&other.row)
            .then_with(|| self.col.cmp(&other.col))
    }
}
impl PartialOrd for IngressLineIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl IngressLineIdx {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn range(start: i32, end: i32) -> Range<Self> {
        Self {
            row: start,
            col: i32::MIN,
        }..Self {
            row: end,
            col: i32::MAX,
        }
    }
}

#[derive(Debug)]
pub struct IngressLine {
    row_range: Range<i32>,
    occlusion_ranges: Vec<Range<i32>>,
}

impl IngressLine {
    pub fn new(row_range: Range<i32>) -> Self {
        Self {
            occlusion_ranges: vec![],
            row_range,
        }
    }

    /// Occlude a range of this line
    /// Returns the length of the newly occluded region
    pub fn occlude(&mut self, range: &Range<i32>) -> usize {
        let mut newly_occluded = 0;
        if let Some(mut intersection) = range_intersect(&self.row_range, range) {
            newly_occluded = (intersection.end - intersection.start) as usize;
            let mut insertion_index = None;
            // Accumulate (sorted) intersecting occlusion ranges into a single range
            for (index, occlusion_range) in self.occlusion_ranges.iter_mut().enumerate() {
                if occlusion_range.end < intersection.start {
                    continue;
                }
                if insertion_index.is_none() {
                    insertion_index = Some(index);
                }
                let already_occluded = range_intersect(&intersection, occlusion_range)
                    .map(|r| (r.end - r.start) as usize)
                    .unwrap_or(0);
                if already_occluded > 0
                    || intersection.start == occlusion_range.end
                    || intersection.end == occlusion_range.start
                {
                    newly_occluded -= already_occluded;
                    // Extend intersection to include occlusion range
                    intersection.start = occlusion_range.start.min(intersection.start);
                    intersection.end = occlusion_range.end.max(intersection.end);
                    // Mark `occlusion_range` for removal
                    occlusion_range.start = occlusion_range.end;
                } else {
                    break;
                }
            }
            // Replace intersecting occlusion ranges with `intersection`
            if let Some(index) = insertion_index {
                self.occlusion_ranges.insert(index, intersection);
                self.occlusion_ranges.retain(|r| r.end > r.start)
            } else {
                self.occlusion_ranges.push(intersection);
            }
        }
        newly_occluded
    }

    pub fn fully_occluded(&self) -> bool {
        self.occlusion_ranges
            .first()
            .is_some_and(|r| r.start == self.row_range.start && r.end == self.row_range.end)
    }

    #[cfg(feature = "plot")]
    #[allow(clippy::single_range_in_vec_init)]
    pub fn remaining(&self) -> impl Iterator<Item = Range<i32>> + '_ {
        self.occlusion_ranges
            .iter()
            .cloned()
            .chain([self.row_range.end..self.row_range.end])
            .scan(self.row_range.start, |start, occ_range| {
                let range = *start..occ_range.start;
                *start = occ_range.end;
                Some(range)
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_occlude() {
        let mut ingressline = IngressLine::new(-10..10);
        let occluded = ingressline.occlude(&(0..1));
        assert_eq!(ingressline.occlusion_ranges, vec![0..1]);
        assert_eq!(occluded, 1);
        //          0
        // -3 -2 -1 0 1 2
        // = 5 new occluded
        let occluded = ingressline.occlude(&(-3..3));
        assert_eq!(ingressline.occlusion_ranges, vec![-3..3]);
        assert_eq!(occluded, 5);

        // -3 -2 -1 0 1 2
        //                  4 5
        // = 2 new occluded
        assert_eq!(ingressline.occlude(&(4..6)), 2);
        assert_eq!(ingressline.occlusion_ranges, vec![-3..3, 4..6]);

        // -3 -2 -1 0 1 2   4 5
        //                3
        // = 1 new occluded
        assert_eq!(ingressline.occlude(&(3..4)), 1);
        assert_eq!(ingressline.occlusion_ranges, vec![-3..6]);
    }

    #[test]
    fn test_occlude2() {
        let mut ingressline = IngressLine::new(-10..10);
        assert_eq!(ingressline.occlude(&(-15..1)), 11);
        assert_eq!(ingressline.occlusion_ranges, vec![-10..1]);

        assert_eq!(ingressline.occlude(&(0..5)), 4);
        assert_eq!(ingressline.occlusion_ranges, vec![-10..5]);
    }
}
