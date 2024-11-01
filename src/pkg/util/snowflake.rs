use std::time::{SystemTime, UNIX_EPOCH};

// Constants
const EPOCH: u64 = 1314220000000; // Epoch
const WORKER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;

pub struct SnowflakeGen {
    worker_id: u64,
    last_timestamp: u64,
    sequence: u64,
}

impl SnowflakeGen {
    /// Ctor
    ///
    /// * `worker_id` - The ID of the worker
    pub fn new(worker_id: u64) -> Self {
        SnowflakeGen {
            worker_id,
            last_timestamp: 0,
            sequence: 0,
        }
    }

    /// Generates a new ID
    pub fn next(&mut self) -> u64 {
        let mut timestamp = Self::get_timestamp();

        // Handle clock drift by waiting until the next ms
        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & ((1 << SEQUENCE_BITS) - 1);
            if self.sequence == 0 {
                timestamp = Self::wait_next_millis(timestamp);
            }
        } else {
            self.sequence = 0;
        }

        // Update the last timestamp
        self.last_timestamp = timestamp;

        // Generate the ID
        ((timestamp - EPOCH) << (WORKER_ID_BITS + SEQUENCE_BITS))
            | (self.worker_id << SEQUENCE_BITS)
            | self.sequence
    }

    // Get the current timestamp in ms
    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("failed to generate ID: SystemTime before UNIX epoch")
            .as_millis() as u64
    }

    // Wait until the next ms
    fn wait_next_millis(current_timestamp: u64) -> u64 {
        let mut timestamp = Self::get_timestamp();
        while timestamp <= current_timestamp {
            timestamp = Self::get_timestamp();
        }
        timestamp
    }
}
