use std::io::{Read, Seek, Write, SeekFrom, Error};
use dd_rs::{Metrics, Arguments, Input, Output};
use clap::Parser;

fn main() -> Result<(), Error> {
    let mut args = Arguments::parse();
    args.resolve();

    let mut input = Input::open(args.get_if_path())?;
    let mut output = Output::open(args.get_of_path())?;
    let mut buffer: Vec<u8> = vec![0; args.get_ibs()];

    // Skip over the offset requested in the program arguments in the input source
    if let Some(blocks_skipped) = args.get_skip() {
        let bytes_skipped = blocks_skipped * args.get_ibs();
        input.seek(SeekFrom::Start(bytes_skipped as u64))?;
    }

    if let Some(blocks_seeked) = args.get_seek() {
        let bytes_seeked = blocks_seeked * args.get_obs();
        output.seek(SeekFrom::Start(bytes_seeked as u64))?;
    }
    
    let mut metrics = Metrics::init(args.get_ibs(), args.get_obs());
    let result = match args.get_count() {
        Some(max_blocks) => {
            let mut block_counter = 0;
            loop {
                let bytes_read = input.read(&mut buffer)?;
                if max_blocks == block_counter || bytes_read == 0 {
                    break metrics.get_snapshot();
                }

                metrics.block_in(bytes_read);
                output.write(&buffer[..bytes_read])?;
                block_counter += 1;
            }
        }
        None => {
            loop {
                let bytes_read = input.read(&mut buffer)?;
                if bytes_read == 0 {
                    break metrics.get_snapshot();
                }

                metrics.block_in(bytes_read);
                output.write(&buffer[..bytes_read])?;
            }
        }
    };

    println!("{}", result);

    Ok(())
}
