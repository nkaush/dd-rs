mod args;
mod metrics;

use metrics::Metrics;
use args::Arguments;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prog_args = Arguments::parse();
    prog_args.resolve();

    // println!("{:?}", prog_args);

    let mut input = prog_args.open_if()?;
    let mut output = prog_args.open_of()?;
    let mut buffer: Vec<u8> = vec![0; prog_args.get_ibs()];

    let mut metrics = Metrics::init(prog_args.get_ibs(), prog_args.get_obs());
    let mut counter = 0;

    while let Ok(num_read) = input.read(&mut buffer) {
        if num_read == 0 {
            metrics.finished();
            break;
        } else if let Some(c) = prog_args.get_count() {
            metrics.finished();
            if c == &counter {
                break;
            }
        }

        metrics.block_in(num_read);
        output.write(&buffer[..num_read])?;
        metrics.block_out(num_read);

        counter += 1;
    }

    println!("{}", metrics);

    Ok(())
}
