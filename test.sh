export RUST_BACKTRACE=1

if [ -z "$1" ]; then
    cargo test --all-features --release -- --show-output
else
    cargo test --all-features --release -- --show-output "$1"
fi

read -n 1 -s