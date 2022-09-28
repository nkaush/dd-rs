mod snapshot;
pub use snapshot::*;

use std::time::Instant;

pub struct Metrics {
    input_block_size: usize,
    output_block_size: usize,
    bytes_copied: usize,
    start_time: Instant,
}

impl Metrics {
    pub fn init(input_block_size: usize, output_block_size: usize) -> Self {
        Self {
            input_block_size,
            output_block_size,
            bytes_copied: 0,
            start_time: Instant::now()
        }
    }

    #[inline(always)]
    pub fn block_in(&mut self, copied: usize) {
        self.bytes_copied += copied;
    }

    pub fn get_snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            full_blocks_in: self.bytes_copied / self.input_block_size,
            full_blocks_out: self.bytes_copied / self.output_block_size,
            partial_block_in: self.bytes_copied % self.input_block_size != 0,
            partial_block_out: self.bytes_copied % self.output_block_size != 0,
            bytes_copied: self.bytes_copied,
            duration: Instant::now() - self.start_time
        }
    }
}