#!/bin/bash
day=${1:?"Missing day argument"}
for part in 1 2
do
	cargo run -rq -- -i src/day${day}/input.txt ${day} ${part}
done
