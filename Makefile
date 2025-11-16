run:
	cargo run

test:
	cargo test

parse:
	cargo run -- parse test.json

get_value:
	cargo run -- get test.json date

validate:
	cargo run -- validate test.json

clippy:
	cargo clippy

fmt:
	cargo fmt

doc:
	cargo doc --open