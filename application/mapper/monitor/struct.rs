use super::*;

/// performance ring buffer.
#[derive(Clone, Data, Debug)]
pub struct PerformanceRingBuffer {
    /// The buffer.
    pub(super) buffer: Vec<PerformanceDataPoint>,
    /// The write index.
    #[get(type(copy))]
    pub(super) write_index: usize,
    /// The count.
    #[get(type(copy))]
    pub(super) count: usize,
    /// The capacity.
    #[get(type(copy))]
    pub(super) capacity: usize,
}
