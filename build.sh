cargo clean
cargo build --release --target x86_64-unknown-linux-gnu
cp -r scripts/ target/x86_64-unknown-linux-gnu/release/scripts/