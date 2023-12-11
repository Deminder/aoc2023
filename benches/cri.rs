mod common;

use criterion::{criterion_group, criterion_main, Criterion};

fn day_benchmark(c: &mut Criterion, day: usize) -> anyhow::Result<()> {
    let (solution, input) = common::bench_day(day)?;

    for part in 1..=2 {
        c.bench_function(&format!("day {day} part{part}"), |b| {
            b.iter(|| solution(input.clone().puzzle_input(), part == 2))
        });
    }
    Ok(())
}

pub fn all_days_benchmark(c: &mut Criterion) {
    for day in 1..=25 {
        if let Err(err) = day_benchmark(c, day) {
            println!("{}", err);
        }
    }
}

criterion_group!(benches, all_days_benchmark);
criterion_main!(benches);
