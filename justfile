run DAY:
    cargo +nightly run --bin {{DAY}}

run-all:
    cargo +nightly run --release --bin runner

test DAY:
    cargo +nightly test --lib {{DAY}}

test-all:
    cargo +nightly test --lib

watch:
    bacon test

bench DAY:
    cargo +nightly bench --bench all-days -- {{DAY}}

bench-all:
    cargo +nightly bench --bench all-days

bench-builtin:
    cargo +nightly run --release --bin bench
