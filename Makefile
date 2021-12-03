all: build

clean-data:
	@rm -rf data

download-data:
	@./scripts/download-data.sh

build: download-data
	@cargo build --all-features

release: download-data
	@cargo build --release

install: download-data
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

.PHONY: all doc test format format-check lint download-data clean-data build release
