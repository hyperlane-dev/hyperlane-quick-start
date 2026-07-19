use super::*;

/// Default implementation for `PerformanceRingBuffer`, using `MAX_HISTORY_SECONDS` as capacity.
impl Default for PerformanceRingBuffer {
    fn default() -> Self {
        Self::new(MAX_HISTORY_SECONDS)
    }
}

/// Ring buffer operations for `PerformanceRingBuffer`, storing performance data points chronologically.
impl PerformanceRingBuffer {
    /// Creates a new ring buffer with the specified capacity.
    ///
    /// # Arguments
    ///
    /// - `usize`: The maximum number of data points the buffer can hold.
    ///
    /// # Returns
    ///
    /// - `PerformanceRingBuffer`: A new empty ring buffer.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            write_index: 0,
            count: 0,
            capacity,
        }
    }

    /// Pushes a new data point into the ring buffer, overwriting the oldest entry when full.
    ///
    /// # Arguments
    ///
    /// - `PerformanceDataPoint`: The data point to insert.
    pub fn push(&mut self, data_point: PerformanceDataPoint) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(data_point);
        } else {
            self.buffer[self.write_index] = data_point;
        }
        self.set_write_index((self.write_index + 1) % self.capacity);
        if self.count < self.capacity {
            self.set_count(self.count + 1);
        }
    }

    /// Returns all data points in chronological order (oldest to newest).
    ///
    /// # Returns
    ///
    /// - `Vec<PerformanceDataPoint>`: The sorted data points.
    pub fn get_all_sorted(&self) -> Vec<PerformanceDataPoint> {
        if self.count == 0 {
            return vec![];
        }
        let mut result: Vec<PerformanceDataPoint> = Vec::with_capacity(self.count);
        if self.count < self.capacity {
            result.extend_from_slice(&self.buffer[..self.count]);
        } else {
            result.extend_from_slice(&self.buffer[self.write_index..]);
            result.extend_from_slice(&self.buffer[..self.write_index]);
        }
        result
    }

    /// Returns the number of data points currently stored in the buffer.
    ///
    /// # Returns
    ///
    /// - `usize`: The count of stored data points.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Checks whether the buffer is empty.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the buffer contains no data points.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns data points whose timestamps fall within the specified range.
    ///
    /// # Arguments
    ///
    /// - `u64`: The start timestamp (inclusive).
    /// - `u64`: The end timestamp (inclusive).
    ///
    /// # Returns
    ///
    /// - `Vec<PerformanceDataPoint>`: The filtered data points.
    pub fn get_range(&self, start_timestamp: u64, end_timestamp: u64) -> Vec<PerformanceDataPoint> {
        self.get_all_sorted()
            .into_iter()
            .filter(|point: &PerformanceDataPoint| {
                let timestamp: u64 = point.get_timestamp();
                timestamp >= start_timestamp && timestamp <= end_timestamp
            })
            .collect()
    }

    /// Returns the most recent `n` data points from the buffer.
    ///
    /// # Arguments
    ///
    /// - `usize`: The number of recent data points to retrieve.
    ///
    /// # Returns
    ///
    /// - `Vec<PerformanceDataPoint>`: The most recent data points.
    pub fn get_recent(&self, n: usize) -> Vec<PerformanceDataPoint> {
        let all: Vec<PerformanceDataPoint> = self.get_all_sorted();
        let skip_count: usize = all.len().saturating_sub(n);
        all.into_iter().skip(skip_count).collect()
    }

    /// Clears all data points and resets the buffer indices.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.set_write_index(0);
        self.set_count(0);
    }
}
