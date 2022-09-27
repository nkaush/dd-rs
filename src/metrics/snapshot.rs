use std::time::Duration;
use std::fmt;

pub struct MetricsSnapshot {
    pub(in crate::metrics) full_blocks_in: usize,
    pub(in crate::metrics) full_blocks_out: usize,
    pub(in crate::metrics) partial_block_in: bool,
    pub(in crate::metrics) partial_block_out: bool,
    pub(in crate::metrics) bytes_copied: usize,
    pub(in crate::metrics) duration: Duration
}

impl fmt::Display for MetricsSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bps = self.bytes_copied as f64 / self.duration.as_secs_f64();
        let bps = bps as usize;
        
        writeln!(f, "{}+{} records in", self.full_blocks_in, self.partial_block_in as u8)?;
        writeln!(f, "{}+{} records out", self.full_blocks_out, self.partial_block_out as u8)?;
        write!(
            f, "{} bytes transferred in {:0.6} secs ({} bytes/sec)", 
            self.bytes_copied, self.duration.as_secs_f64(), bps
        )?;
        
        Ok(())
    }
}