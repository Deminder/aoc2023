#!/bin/bash
day=${1:?"Missing day argument"}
INPUT_FILE="src/day${day}/input.txt"
SESSION_KEY_FILE=session.txt
if [ ! -f "$INPUT_FILE" ] && [ -f "$SESSION_KEY_FILE" ]
then
	curl "https://adventofcode.com/2023/day/$day/input" \
		--compressed \
		-A "curl/$(curl --version | head -n 1 | cut -f2-3 -d' ') ($(cargo read-manifest | jq -r '.homepage,.authors[0]' | tr '\n' ' '))" \
		-H "Cookie: session=$(cat "$SESSION_KEY_FILE")" \
		--create-dirs \
		--output "$INPUT_FILE"
fi
for part in 1 2
do
	cargo run -rq -- -i "$INPUT_FILE" ${day} ${part}
done
