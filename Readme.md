# RUST CPU Utilization by PID

A Rust application that monitors the CPU utilization of Processes by PIDs

## Compile

Use Cargo to build the project

```bash
cargo build --release
```

## Dependencies 

This implementation of process monitor depends on  [psutil](https://crates.io/crates/psutil) for getting process information, [chrono](https://crates.io/crates/chrono) for local time stamps, [rusqlite](https://crates.io/crates/rusqlite/) to write data to the database and [clap](https://crates.io/crates/clap) to parse command line input.

```bash
psutil = "3.1.0"
chrono = "0.4"
clap = "2.33.2"
rusqlite = "0.23.1"
```

## Usage

```bash
    rust_cpu_pid [FLAGS] [OPTIONS]

FLAGS:
    -i, --interactive    Launches Interactive Mode
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -b, --batch <batch>    List of PIDs separated by commas e.g. -b 646,323,55,665
```
Once the console application starts, Enter a PID to monitor or leave blank and press enter to monitor the current process.

Interactive Mode
```bash
RUST Process CPU utilization
Please Enter Process IDs separated by commas 
        e.g. 646,456,345 (Leave Blank to Monitor this application)
```
## Data Format

Please make sure SQLite 3 is installed on your system.

Data is written to SQLite Database pid_data.db in the table pid_util.
