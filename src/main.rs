use signal_hook::iterator::SignalsInfo;
use signal_hook::{consts::SIGINT, consts::SIGUSR1, iterator::Signals};
use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use std::io::{Read, Seek, Write, SeekFrom, Error};
use dd_rs::{Metrics, Arguments, Input, Output};
use clap::Parser;
use std::thread;

type ExitCondition = Box<dyn Fn(usize, usize) -> bool>;

fn signal_handler(mut signals: SignalsInfo, got_sigusr1: Arc<AtomicBool>, got_sigint: Arc<AtomicBool>) {
    for sig in signals.forever() {
        match sig {
            SIGUSR1 => got_sigusr1.store(true, Ordering::Relaxed),
            SIGINT => got_sigint.store(true, Ordering::Relaxed),
            _ => ()
        } 
    }
}

fn main() -> Result<(), Error> {
    let mut args = Arguments::parse();
    args.resolve();

    // Set up signal handling and atomic states
    let got_sigusr1: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let got_sigint: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let got_sigusr1c: Arc<AtomicBool> = Arc::clone(&got_sigusr1);
    let got_sigintc: Arc<AtomicBool> = Arc::clone(&got_sigint);

    // Spawn signal handler thread
    let signals = Signals::new(&[SIGINT, SIGUSR1])?;
    thread::spawn(move || signal_handler(signals, got_sigusr1c, got_sigintc));

    // Set up input, output and read buffer
    let mut input = Input::open(args.get_if_path())?;
    let mut output = Output::open(args.get_of_path())?;
    let mut buffer: Vec<u8> = vec![0; args.get_ibs()];

    // Skip over the offset requested in the program arguments in the input
    if let Some(blocks_skipped) = args.get_skip() {
        let bytes_skipped = blocks_skipped * args.get_ibs();
        input.seek(SeekFrom::Start(bytes_skipped as u64))?;
    }

    // Skip over the offset requested in the program arguments in the output
    if let Some(blocks_seeked) = args.get_seek() {
        let bytes_seeked = blocks_seeked * args.get_obs();
        output.seek(SeekFrom::Start(bytes_seeked as u64))?;
    }
    
    // Define out exit condition depending if there is a max block limit
    let should_exit: ExitCondition = match args.get_count() {
        Some(max_blocks) => {
            Box::new(move |blocks_read: usize, bytes_read: usize| {
                blocks_read == max_blocks || bytes_read == 0
            })
        },
        None => Box::new(|_: usize, bytes_read: usize| bytes_read == 0)
    };

    let mut metrics = Metrics::init(args.get_ibs(), args.get_obs());
    let mut block_counter = 0;
    let result = loop {
        let bytes_read = input.read(&mut buffer)?;
        if should_exit(block_counter, bytes_read) || got_sigint.load(Ordering::Relaxed) {
            break metrics.get_snapshot();
        }
        
        if got_sigusr1.load(Ordering::Relaxed) {
            got_sigusr1.store(false, Ordering::Relaxed);
            println!("{}", metrics.get_snapshot());
        }

        metrics.block_in(bytes_read);
        output.write(&buffer[..bytes_read])?;
        block_counter += 1;
    };

    println!("{}", result);

    Ok(())
}
