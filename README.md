# ripnode

Simple tool for deleting node_modules and other folders

*Inspired by [`npkill`](https://www.npmjs.com/package/npkill)*

![Continuous integration](https://github.com/JorgeMayoral/ripnode/workflows/Continuous%20integration/badge.svg)
[![](https://img.shields.io/crates/v/ripnode.svg)](https://crates.io/crates/ripnode)

## Installation

### Cargo

```sh
cargo install ripnode
```

## Usage

By default, it will run in interactive mode, showing a TUI with the folders to delete.

Directory to delete defaults to node_modules, but can be changed with the -n or --name option.
Directory search starts from the current directory.

```sh
ripnode [OPTIONS]
```

### Options

```sh
  -d, --dry-run          See what would be deleted without actually deleting anything
  -n, --name <NAME>      The name of the folder to delete [default: node_modules]
      --non-interactive  Run as CLI, without TUI
  -v, --verbose...       More output per occurrence
  -q, --quiet...         Less output per occurrence
  -h, --help             Print help
  -V, --version          Print version
```

## Purpose

This is a learning exercise for me to learn Rust and keeping up with its ecosystem.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
