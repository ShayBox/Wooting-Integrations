use std::time::Duration;

const BUFFER_SIZE: usize = 10;

pub struct Latency {
    buffer: [Duration; BUFFER_SIZE],
    index: usize,
    total: Duration,
}

impl Default for Latency {
    fn default() -> Self {
        Self {
            buffer: [Duration::default(); BUFFER_SIZE],
            index: usize::default(),
            total: Duration::default(),
        }
    }
}

impl Latency {
    pub fn add(&mut self, value: Duration) {
        // Subtract the old value from the total and replace it with the new one
        self.total -= self.buffer[self.index];
        self.buffer[self.index] = value;
        self.total += value;

        // Move the index to the next position in the ring
        self.index = (self.index + 1) % BUFFER_SIZE;
    }

    /// # Returns the average latency of the last `BUFFER_SIZE` samples
    ///
    /// # Panics
    /// if `BUFFER_SIZE` is larger than the size of the buffer
    #[must_use]
    pub fn average(&self) -> Duration {
        self.total / u32::try_from(BUFFER_SIZE).expect("BUFFER_SIZE too large")
    }
}
