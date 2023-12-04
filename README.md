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

To get solutions of part 1+2 for day `n`, given the puzzle input in `src/day{n}/input.txt`, run:
```sh
./runday.sh $n
```

The tests of day `n` may be run by:
```shell
cargo test day$n
```
# References
- https://adventofcode.com/2023/
