use aoc2025::jobs;
use std::io::{self, Write};

use took::{Timer, Took};

const RUNS: usize = 100;

fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);

    let times: Vec<_> = jobs()
        .iter()
        .map(|j| {
            (
                j.1,
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        j.0();
                        let result = took.took().into_std();
                        io::stdout().flush().expect("Could not flush stdout");
                        result
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();

    println!();
    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("everything");
}
