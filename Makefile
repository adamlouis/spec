
all: build fmt test

build:
	cargo build --release

fmt:
	cargo fmt --all -- --check

test:
	cargo test