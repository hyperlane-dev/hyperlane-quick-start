use super::*;

/// performance ring buffer.
#[derive(Clone, Data, Debug)]
pub struct PerformanceRingBuffer {
    /// The buffer.
    pub buffer: Vec<PerformanceDataPoint>,
    /// The write index.
    #[get(type(copy))]
    pub write_index: usize,
    /// The count.
    #[get(type(copy))]
    pub count: usize,
    /// The capacity.
    #[get(type(copy))]
    pub capacity: usize,
}
