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
    cargo +nightly test --release --lib {{DAY}}
    cargo +nightly bench --bench all-days -- {{DAY}}

bench-all:
    cargo +nightly test --release --lib
    cargo +nightly bench --bench all-days

bench-builtin:
    cargo +nightly run --release --bin bench

flamegraph DAY:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo +nightly flamegraph --bin {{DAY}} -- --bin {{DAY}}

flamegraph-bench DAY:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo +nightly flamegraph --bench all-days -- --bench {{DAY}}
