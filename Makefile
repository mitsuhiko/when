all: build

build:
	@(cd cli; cargo build --all-features)

release:
	@(cd cli; cargo build --release)

install:
	@cargo install --path=./cli

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy

.PHONY: all test format format-check lint clean-data build release
