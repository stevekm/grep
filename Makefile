SHELL:=/bin/bash
rustup:
	curl https://sh.rustup.rs -sSf | sh

update:
	rustup update
	rustup self update

build:
	cargo build

run:
	printf 'foo\nbar\nbaz\nbuz\nfuzz\n' | cargo run -- bar
	cargo run 158647522 data.txt

test:
	cargo test
