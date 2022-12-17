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

full-migration:
	RUST_LOG=migrate=info cargo run --bin migrate -- --destructive

start:
	RUST_LOG=info,sqlx=warn cargo run --bin api

start-prod:
	RUST_LOG=warn,digraph=info target/release/api

test:
	cargo test
