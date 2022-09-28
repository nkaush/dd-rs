use dd_rs::{Metrics, MetricsSnapshot, GenericInput, GenericOutput, Arguments};
use std::io::{Read, Seek, Write, SeekFrom, BufReader, BufWriter};
use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use signal_hook::iterator::{Signals, SignalsInfo};
use signal_hook::consts::{SIGINT, SIGUSR1};
use std::{process, thread};

type ExitCondition = Box<dyn Fn(usize, usize) -> bool>;
type Flag = Arc<AtomicBool>;

fn main() {
    if let Err(e) = dd() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn signal_handler(mut signals: SignalsInfo, sigusr1: Flag, sigint: Flag) {
    for sig in signals.forever() {
        match sig {
            SIGUSR1 => sigusr1.store(true, Ordering::Relaxed),
            SIGINT => sigint.store(true, Ordering::Relaxed),
            _ => ()
        } 
    }
}

fn dd() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse()?;

    if args.help_requested() {
        dd_rs::print_help();
        process::exit(0);
    } else if args.version_requested() {
        dd_rs::print_version();
        process::exit(0);
    }

    // Set up signal handling and atomic states
    let got_sigusr1: Flag = Arc::new(AtomicBool::new(false));
    let got_sigint: Flag = Arc::new(AtomicBool::new(false));
    let got_sigusr1c: Flag = Arc::clone(&got_sigusr1);
    let got_sigintc: Flag = Arc::clone(&got_sigint);

    // Spawn signal handler thread
    let signals = Signals::new(&[SIGINT, SIGUSR1])?;
    thread::spawn(move || signal_handler(signals, got_sigusr1c, got_sigintc));

    // Set up input, output and read buffer
    let input = GenericInput::open(args.get_if_path())?;
    let mut input = BufReader::new(input);

    let output = GenericOutput::open(args.get_of_path())?;
    let mut output = BufWriter::new(output);
    
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
    
    // Define our exit condition depending whether there is a max block limit
    let should_exit: ExitCondition = match args.get_count() {
        Some(max_blocks) => {
            Box::new(move |blocks_read: usize, bytes_read: usize| {
                blocks_read == max_blocks || bytes_read == 0
            })
        },
        None => Box::new(|_: usize, bytes_read: usize| bytes_read == 0)
    };

    let mut metrics = Metrics::init(args.get_ibs(), args.get_obs());
    let mut block_counter: usize = 0;
    let result: MetricsSnapshot = loop {
        let bytes_read = input.read(&mut buffer)?;
        if should_exit(block_counter, bytes_read) || got_sigint.load(Ordering::Relaxed) {
            break metrics.get_snapshot();
        }
        
        if got_sigusr1.load(Ordering::Relaxed) {
            got_sigusr1.store(false, Ordering::Relaxed);
            eprintln!("{}", metrics.get_snapshot());
        }

        metrics.block_in(bytes_read);
        output.write(&buffer[..bytes_read])?;
        block_counter += 1;
    };

    eprintln!("{}", result);

    Ok(())
}
