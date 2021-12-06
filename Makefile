all: build

build:
	@cargo build --all-features

release:
	@cargo build --release

install:
	@cargo install --path=.

doc:
	@cargo doc --all-features

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy

.PHONY: all doc test format format-check lint clean-data build release
