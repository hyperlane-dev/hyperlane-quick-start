use super::*;

#[derive(Clone, Data, Debug)]
pub struct PerformanceRingBuffer {
    pub(super) buffer: Vec<PerformanceDataPoint>,
    #[get(type(copy))]
    pub(super) write_index: usize,
    #[get(type(copy))]
    pub(super) count: usize,
    #[get(type(copy))]
    pub(super) capacity: usize,
}
