check: format test
	cargo check

format:
	cargo clippy --fix
	cargo fmt

start:
	RUST_LOG=info,digraph=debug cargo run

test:
	cargo test
