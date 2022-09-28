# dd-rs
A Rust implementation of the Linux dd utility.

## Usage

```
dd-rs 0.1.0
A Rust implementation of the Linux dd utility.

Author: Neil Kaushikkar <neil.kaushikkar@gmail.com>
Source: https://github.com/nkaush/dd-rs/

USAGE:
        ./dd-rs [OPERAND] ...
        ./dd-rs [OPTION]

OPERANDS:
        bs=BYTES        Read and write up to BYTES bytes at a time; overrides ibs and obs [default: 512]
        ibs=BYTES       Read up to BYTES bytes at a time [default: 512]
        obs=BYTES       Write BYTES bytes at a time [default: 512]
        count=N         Copy only N input blocks
        if=FILE         Read from FILE instead of stdin
        of=FILE         Write to FILE instead of stdout
        seek=N          Skip N obs-sized blocks at start of output
        skip=N          Skip N ibs-sized blocks at start of input

OPTIONS:
        -h, --help      Print help information
        -V, --version   Print version information
```