y15:
    RUSTFLAGS="-C target-cpu=native" cargo run --release --bin=y15 -- data/y15

y15-flame:
    RUSTFLAGS="-C target-cpu=native" cargo flamegraph -F 999 --release --bin=y15 -- data/y15