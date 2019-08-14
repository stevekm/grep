SHELL:=/bin/bash
rustup:
	curl https://sh.rustup.rs -sSf | sh

update:
	rustup update
	rustup self update

build:
	cargo build

run:
	cargo build
	printf 'foo\nbar\nbaz\nbuz\nfuzz\n' | target/debug/grep bar
	printf 'foo\nbar\nbaz\nbuz\nfuzz\n' | target/debug/grep -A 1 -B 1 bar
	target/debug/grep 15 data.txt
	target/debug/grep -A 1 15 data.txt
	target/debug/grep -B 2 15 data.txt

test:
	cargo test
