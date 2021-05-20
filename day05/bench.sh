#!/usr/bin/env bash

set -e

cd "$(git rev-parse --show-toplevel)"

if [ -z "$1" ]; then
    echo "USAGE: $0 DAY [HYPERFINE_ARG [HYPERFINE_ARG ...]]"
    echo 1
fi
day="$1"
shift 1

tmpdir="$(mktemp -d)"
np="$tmpdir/$day-no-parallelism"
p="$tmpdir/$day-parallelism"

cargo build --release -p "$day"
cp target/release/"$day" "$np"

cargo build --release -p day05 --features parallelism
cp target/release/"$day" "$p"

hyperfine "$np" "$p" "$@"
hyperfine "$np --part2 --no-part1" "$p --part2 --no-part1" "$@"
