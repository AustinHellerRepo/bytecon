export RUST_BACKTRACE=1
cargo test --all-features --release -- --show-output
read -n 1 -s