# Advent of Code 2022 in Rust

My [Advent of Code 2022][aoc-2022] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

## Run solutions

Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo +nightly run --release

# or run everything in parallel
cd ../runner
cargo +nightly run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo +nightly run --release --bin bench
```

Some solutions require Rust Nightly, that's why `+nightly` is included.

[aoc-2022]: https://adventofcode.com/2022
