use crate::Arguments;
use std::env;

pub fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn print_help() {
    // REFs: https://doc.rust-lang.org/cargo/reference/environment-variables.html
    print!("\x1b[1m\x1b[33m");
    print_version();
    print!("\x1b[0m");
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!();

    println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Source: {}", env!("CARGO_PKG_REPOSITORY"));
    println!();

    let program_name = env::args().nth(0).unwrap();
    println!("\x1b[1m\x1b[33mUSAGE:\x1b[0m");
    println!("\t{} [OPERAND] ...", program_name);
    println!("\t{} [OPTION]", program_name);
    println!();

    println!("\x1b[1m\x1b[33mOPERANDS:\x1b[0m");
    println!("\tbs=BYTES\tRead and write up to BYTES bytes at a time; overrides ibs and obs [default: {}]", Arguments::DEFAULT_BS);
    println!("\tibs=BYTES\tRead up to BYTES bytes at a time [default: {}]", Arguments::DEFAULT_BS);
    println!("\tobs=BYTES\tWrite BYTES bytes at a time [default: {}]", Arguments::DEFAULT_BS);
    println!("\tcount=N\t\tCopy only N input blocks");
    println!("\tif=FILE\t\tRead from FILE instead of stdin");
    println!("\tof=FILE\t\tWrite to FILE instead of stdout");
    println!("\tseek=N\t\tSkip N obs-sized blocks at start of output");
    println!("\tskip=N\t\tSkip N ibs-sized blocks at start of input");
    println!();

    println!("\x1b[1m\x1b[33mOPTIONS:\x1b[0m");
    println!("\t-h, --help\tPrint help information");
    println!("\t-V, --version\tPrint version information");
}