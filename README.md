# MilliGrep

`Custom simplified implementation of grep`

## What is `grep`?

Classic command line tool `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint).

## What is `milligrep`?

In the simplest use case, `milligrep` searches a specified file for a specified string. To do so, `milligrep` takes as its arguments a filename and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

`This project is built with Rust and uses Cargo as it's package manager.`

## Usage

```bash
cargo run 'search_string' 'path/filename.txt'
```

Default behavior is case-sensitive search, provide 'CASE_INSENSITIVE' environment variable with any value for case-insensitive searches.

```bash
CASE_INSENSITIVE=1 cargo run 'search_string' 'path/filename.txt'
```

## Test

```bash
cargo test
```

## Compile binary

```bash
cargo build
```

`You will find it inside 'target' dir`
