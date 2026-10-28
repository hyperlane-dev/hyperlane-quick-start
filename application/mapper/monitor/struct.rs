use super::*;

/// performance ring buffer.
#[derive(Clone, Data, Debug)]
pub struct PerformanceRingBuffer {
    /// The buffer.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) buffer: Vec<PerformanceDataPoint>,
    /// The write index.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) write_index: usize,
    /// The count.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) count: usize,
    /// The capacity.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) capacity: usize,
}
