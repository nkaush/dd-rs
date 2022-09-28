# dd-rs
A Rust implementation of the Linux dd utility.

## Usage

```
dd 0.1.0
Neil Kaushikkar <neil.kaushikkar@gmail.com>
A Rust implementation of the Linux dd utility.

USAGE:
    dd-rs [OPTIONS]

OPTIONS:
        --bs <BS>          Read and write up to BS bytes at a time; overrides ibs and obs [default:
                           512]
        --count <COUNT>    Copy only COUNT input blocks
    -h, --help             Print help information
        --ibs <IBS>        Read up to IBS bytes at a time [default: 512]
        --if <IF>          Read from IF instead of stdin
        --obs <OBS>        Write OBS bytes at a time [default: 512]
        --of <OF>          Write to OF instead of stdout
        --seek <SEEK>      Skip SEEK obs-sized blocks at start of output
        --skip <SKIP>      Skip SKIP ibs-sized blocks at start of input
    -V, --version          Print version information
```