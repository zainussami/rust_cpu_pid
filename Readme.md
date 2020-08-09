# RUST CPU Utilization by PID

A Rust application that monitors the CPU utilization of a Process by PID

## Compile

Use Cargo to build the project

```bash
cargo build --release
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
Once the console application starts, Enter a PID to monitor or leave blank and press enter to monitor the current process.

```bash
RUST Process CPU utilization
Please Enter a Process ID (Leave Blank to Monitor this application): 
```

Data is written to file data/pid_time_stamp.txt

## Data Format

CPU utilization Data for process_name <br/>
CPU%: utilization_percentage  TimeStamp: time_stamp