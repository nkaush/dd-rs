use dd_rs::{Metrics, Arguments, Input, Output};
use std::io::{Read, Seek, Write, SeekFrom, Error};
use clap::Parser;

fn main() -> Result<(), Error> {
    let mut args = Arguments::parse();
    args.resolve();

    let mut input = Input::open(args.get_if_path())?;
    let mut output = Output::open(args.get_of_path())?;
    let mut buffer: Vec<u8> = vec![0; args.get_ibs()];

    let mut metrics = Metrics::init(args.get_ibs(), args.get_obs());
    let mut counter = 0;

    let are_blocks_capped = args.get_count().is_some();
    let max_blocks = args.get_count().unwrap_or_default();

    // Skip over the offset requested in the program arguments in the input source
    if let Some(skip) = args.get_skip() {
        input.seek(SeekFrom::Start(skip))?;
    }

    if let Some(seek) = args.get_seek() {
        output.seek(SeekFrom::Start(seek))?;
    }

    while let Ok(num_read) = input.read(&mut buffer) {
        if num_read == 0 || (are_blocks_capped && max_blocks == counter) {
            metrics.finished();
            break;
        }

        metrics.block_in(num_read);
        output.write(&buffer[..num_read])?;
        metrics.block_out(num_read);

        counter += 1;
    }

    println!("{}", metrics);

    Ok(())
}
