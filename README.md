# grep

GNU grep implemented in Rust

# Installation

## Build From Source

First, clone this repository:

```
git clone https://github.com/stevekm/grep.git
cd grep
```

If `rustup` is not already installed, install it with `curl https://sh.rustup.rs -sSf | sh`.

Build the program with `cargo build`.

# Usage

The `Makefile` includes some example methods to run the program; `make run`.

By default the program reads from `stdin`.

```
$ printf 'foo\nbar\nbaz\n' | target/debug/grep bar
bar
```

A file can also be passed

```
$ target/debug/grep 5 data.txt
5
15
```

Lines preceeding and following a match can be printed

```
$ target/debug/grep -A 1 -B 2 5 data.txt
--
3
4
5
6
--
--
13
14
15
16
--
```
