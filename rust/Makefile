build:
	cargo build --release


check: format test
	cargo check

check-pre-push: check

clean:
	cargo clean

export:
	RUST_LOG=warn,export=info target/release/export --data-dir ~/data/digraph-data

format:
	cargo clippy -- -D warnings
	cargo fmt

migrate:
	cargo run --bin migrate

start:
	RUST_LOG=info,digraph=debug cargo run --bin frontend

start-prod:
	RUST_LOG=warn,digraph=info target/release/frontend

test:
	cargo test
