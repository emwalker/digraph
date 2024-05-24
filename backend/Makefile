build:
	cargo build --release


check: format test
	cargo check

check-pre-push: check

clean:
	cargo clean

dev:
	RUST_LOG=info,sqlx=warn cargo run --bin api

export:
	RUST_LOG=warn,export=info target/release/export --data-dir ~/data/digraph-data

format:
	cargo fmt
	cargo clippy --fix --allow-dirty -- -D warnings

migrate:
	cargo run --bin migrate

full-migration:
	RUST_LOG=migrate=info cargo run --bin migrate -- --destructive

prod:
	RUST_LOG=warn,digraph=info target/release/api

test:
	cargo test
