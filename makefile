.PHONY: clean format clippy test audit build

all: clean format clippy test audit build

clean:
	cargo clean

fotmat:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

audit:
	cargo audit

build:
	cargo build --release