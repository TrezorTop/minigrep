# Minigrep

Minigrep is a simple command-line application written in Rust. It allows you to search for a specific query within a text file. The application supports both case-sensitive and case-insensitive searches.

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

- Rust programming language installed on your machine. You can download it from [here](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository
```bash
git clone https://github.com/TrezorTop/rust-minigrep.git
```
2. Navigate to the project directory
```bash
cd rust-minigrep
```
3. Build the project
```bash
cargo build
```

## Usage

You can run the application using the `cargo run` command followed by your search query and the path to the file you want to search in.

```bash
cargo run <query> <file_path>
```

For example, to search for the word "frog" in a file named "sample.txt", you would run:

```bash
cargo run frog sample.txt
```

By default, the search is case-sensitive. If you want to perform a case-insensitive search, you can set the `IGNORE_CASE` environment variable.

```bash
IGNORE_CASE=1 cargo run frog sample.txt
```

## Running Tests

You can run the tests using the `cargo test` command.

```bash
cargo test
```
