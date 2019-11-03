# len

Command that reads stdin and prints its length in bytes, with human-friendly
formatting if the output is a terminal.

## Installation

[Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html),
the package manager for the Rust programming language. Then run:

```sh
$ cargo install len
```

## Usage

```sh
$ cat /usr/share/dict/words | len

2.4 MiB
```

`len` will print its output in human-readable format if the output is a terminal,
or a plain number otherwise, in which case it is the same as using `wc -c`.
