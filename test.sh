export RUST_BACKTRACE=1
cargo test --all-features -- --show-output
read -n 1 -s