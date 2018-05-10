# Overview

This utility reads the output produced by the complimentary [search-recorder](https://github.com/cp-profiler/search-logger) utility and redirects it to CP-Profiler.

Debug mode is available (`-d`, `--debug`) that lets the user send messages one at a time.

# Building

- make sure that Rust and Cargo are installed (can be installed using [rustup](https://www.rustup.rs))
- execute `cargo build --release` in the same directory as `Cargo.toml`
