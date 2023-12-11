use iai_callgrind::{
    black_box, library_benchmark, library_benchmark_group, main, FlamegraphConfig,
    LibraryBenchmarkConfig,
};

mod common;

macro_rules! day_bench {
    ($day_func: ident, $day_num: expr) => {
        #[library_benchmark]
        #[bench::part1(false)]
        #[bench::part2(true)]
        pub fn $day_func(part2: bool) -> Option<String> {
            black_box(
                if let Ok((solution, input)) = black_box(common::bench_day($day_num)) {
                    Some(black_box(solution(input.puzzle_input(), part2)))
                } else {
                    None
                },
            )
        }
    };
}
day_bench!(day1, 1);
day_bench!(day2, 2);
day_bench!(day3, 3);
day_bench!(day4, 4);
day_bench!(day5, 5);
day_bench!(day6, 6);
day_bench!(day7, 7);
day_bench!(day8, 8);
day_bench!(day9, 9);
day_bench!(day10, 10);
day_bench!(day11, 11);
day_bench!(day12, 12);
day_bench!(day13, 13);
day_bench!(day14, 14);
day_bench!(day15, 15);
day_bench!(day16, 16);
day_bench!(day17, 17);
day_bench!(day18, 18);
day_bench!(day19, 19);
day_bench!(day20, 20);
day_bench!(day21, 21);
day_bench!(day22, 22);
day_bench!(day23, 23);
day_bench!(day24, 24);
day_bench!(day25, 25);

library_benchmark_group!(
    name = days;
    benchmarks = day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25);

main!(
    config = LibraryBenchmarkConfig::default().flamegraph(FlamegraphConfig::default());
    library_benchmark_groups = days
);
