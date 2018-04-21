build:
	cargo build

run:
	cargo run

help: build
	target/debug/ocean --help

.PHONY: build run
