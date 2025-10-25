default: fmt lint test

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy --tests
