# Solutions for Advent of Code 2023 in Rust

```
Usage: aoc2023 [OPTIONS] <DAY> [PART]

Arguments:
  <DAY>   Day of puzzle
  [PART]  Part of puzzle to run [default: 1]

Options:
  -i, --input-file <FILE>  Puzzle input file, otherwise reads from stdin
  -h, --help               Print help
  -V, --version            Print version
```

You get answers for part `1` and `2` by running:
```sh
./runday.sh $n
```
if you place your puzzle input for day `n` in `src/day{n}/input.txt` or put a AOC session key into `session.txt`.

The tests of day `n` may be run by:
```shell
cargo test day$n
```
# References
- https://adventofcode.com/2023/
