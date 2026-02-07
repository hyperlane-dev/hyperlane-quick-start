use super::*;

impl Default for PerformanceRingBuffer {
    fn default() -> Self {
        Self::new(MAX_HISTORY_SECONDS)
    }
}

impl PerformanceRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            write_index: 0,
            count: 0,
            capacity,
        }
    }

    pub fn push(&mut self, data_point: PerformanceDataPoint) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(data_point);
        } else {
            self.buffer[self.write_index] = data_point;
        }
        self.write_index = (self.write_index + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    pub fn get_all_sorted(&self) -> Vec<PerformanceDataPoint> {
        if self.count == 0 {
            return Vec::new();
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

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn get_range(&self, start_timestamp: u64, end_timestamp: u64) -> Vec<PerformanceDataPoint> {
        self.get_all_sorted()
            .into_iter()
            .filter(|point| {
                let timestamp: u64 = point.get_timestamp();
                timestamp >= start_timestamp && timestamp <= end_timestamp
            })
            .collect()
    }

    pub fn get_recent(&self, n: usize) -> Vec<PerformanceDataPoint> {
        let all: Vec<PerformanceDataPoint> = self.get_all_sorted();
        let skip_count: usize = all.len().saturating_sub(n);
        all.into_iter().skip(skip_count).collect()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.write_index = 0;
        self.count = 0;
    }
}
