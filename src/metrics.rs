use std::time::Instant;
use std::fmt;

pub struct Metrics {
    input_block_size: usize,
    output_block_size: usize,
    full_blocks_in: usize,
    full_blocks_out: usize,
    partial_block_in: bool,
    partial_block_out: bool,
    bytes_copied: usize,
    start_time: Instant,
    end_time: Instant
}

impl Metrics {
    pub fn init(input_block_size: usize, output_block_size: usize) -> Self {
        Self {
            input_block_size,
            output_block_size,
            full_blocks_in: 0,
            full_blocks_out: 0,
            partial_block_in: false,
            partial_block_out: false,
            bytes_copied: 0,
            start_time: Instant::now(),
            end_time: Instant::now()
        }
    }

    pub fn block_in(&mut self, copied: usize) {
        if copied == self.input_block_size {
            self.full_blocks_in += 1;
            self.bytes_copied += copied;
        } else {
            self.partial_block_in = true;
        }
    }

    pub fn block_out(&mut self, copied: usize) {
        if copied == self.output_block_size {
            self.full_blocks_out += 1;
        } else {
            self.partial_block_out = true;
        }
    }

    pub fn finished(&mut self) {
        self.end_time = Instant::now();
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration = self.end_time - self.start_time;
        let bps = (self.bytes_copied as f64 / duration.as_secs_f64()) as usize;
        
        writeln!(f, "{}+{} records in", self.full_blocks_in, self.partial_block_in as u8)?;
        writeln!(f, "{}+{} records out", self.full_blocks_out, self.partial_block_out as u8)?;
        write!(
            f, "{} bytes transferred in {:0.6} secs ({} bytes/sec)", 
            self.bytes_copied, duration.as_secs_f64(), bps
        )?;
        
        Ok(())
    }
}