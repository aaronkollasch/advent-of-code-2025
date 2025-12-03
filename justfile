run DAY:
    cargo +nightly run --bin {{DAY}}

run-all:
    cargo +nightly run --release --bin runner

test:
    cargo +nightly test --lib

watch:
    bacon test

bench-all:
    cargo +nightly bench --bench all-days

bench DAY:
    cargo +nightly bench --bench all-days -- {{DAY}}
