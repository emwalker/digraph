build:
	cargo build --release


check: format test
	cargo check

clean:
	cargo clean

format:
	cargo clippy --fix
	cargo fmt

start:
	RUST_LOG=info,digraph=debug cargo run --bin frontend

start-prod:
	RUST_LOG=info,digraph=debug target/release/frontend

test:
	cargo test
