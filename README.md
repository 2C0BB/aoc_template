# Advent of Code Template

## About
Simple Rust template for me to use in advent of code

## Setup
Use [cargo generate](https://github.com/cargo-generate/cargo-generate) to clone this template

```
cargo generate --git https://github.com/2C0BB/aoc_template --name my-project
cd my-project
```

## Usage

1. Place the challenge input in `inputs/input.txt` and sample inputs in `inputs/test1.txt` and `inputs/test2.txt`

2. Edit the `src/parts.rs` file to fill in the code for parts 1 and 2 for the challenge along with expected outputs for sample tests

3. Run `cargo test` to test code against sample inputs

4. Run `cargo run` to determine the output of the code for parts 1 and 2 with the actual input
