#!/bin/bash
day=${1:?"Missing day argument"}
part=${2:?"Missing part argument"}
cargo build --profile profiling
LONG_INPUT_FILE=src/day${day}/longinput.txt
flatpak-spawn --host \
	samply record ./target/profiling/aoc2023 \
	-i "$([ -f "$LONG_INPUT_FILE" ] && echo "$LONG_INPUT_FILE" || echo src/day${day}/input.txt)" \
	${day} ${part}
