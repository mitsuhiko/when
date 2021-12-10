all: build

build:
	@(cd cli; cargo build --all-features)

test:
	@cargo test --all

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

web-dev:
	@cd web; wasm-pack build
	@cd web/www; npm run start

web-dist:
	@rm -rf web/www/dist
	@cd web; wasm-pack build --release
	@cd web/www; npm run build

.PHONY: all test format format-check lint clean-data build release web-dev web-dist
