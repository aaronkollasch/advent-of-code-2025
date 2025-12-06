# Advent of Code 2025 in Rust

My [Advent of Code 2025](https://adventofcode.com/2025) solutions in the Rust programming language.

Three years ago I solved every problem in Advent of Code 2022 in under a second:
https://github.com/aaronkollasch/advent-of-code-2022, inspired by Tim Visee's blog:
https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/.

## Timings

Timing code modified from https://github.com/timvisee/advent-of-code-2022

## How to Run

First copy the input text files into the `inputs/` directory, and name them by day, e.g. `day01.txt`.

To run an individual solution in debug mode:
```shell
cargo +nightly run --bin day01a
```

To run everything in parallel:
```shell
cargo +nightly run --release --bin runner-par
```

To benchmark every day:
```shell
cargo +nightly run --release --bin bench
```

or using criterion:
```shell
cargo +nightly bench --bench all-days
```

To run tests:
```shell
cargo +nightly test --lib
```

## Other years

- [2025](https://github.com/aaronkollasch/advent-of-code-2025) _(current)_
- [2022](https://github.com/aaronkollasch/advent-of-code-2022)

## License
This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
