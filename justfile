yr YEAR:
    RUSTFLAGS="-C target-cpu=native" cargo run --release --bin=y{{YEAR}} -- data/y{{YEAR}}

clean:
    cargo clean
