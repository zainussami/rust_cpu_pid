# RUST CPU Utilization by PID

A Rust application that monitors the CPU utilization of a Process by PID

## Compile

Use Cargo to build the project

```bash
cargo build
```

## Dependencies 

This implementation of process monitor depends on  [psutil](https://crates.io/crates/psutil) for getting process information and [chrono](https://crates.io/crates/chrono) for local time stamps.

```bash
psutil = "3.1.0"
chrono = "0.4"
```

## Usage

```bash
cargo run
```
