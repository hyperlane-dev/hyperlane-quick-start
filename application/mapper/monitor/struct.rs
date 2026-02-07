use super::*;

#[derive(Clone, Debug)]
pub struct PerformanceRingBuffer {
    pub buffer: Vec<PerformanceDataPoint>,
    pub write_index: usize,
    pub count: usize,
    pub capacity: usize,
}
