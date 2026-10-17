use super::*;

fn point(timestamp: u64) -> PerformanceDataPoint {
    let mut data_point: PerformanceDataPoint = PerformanceDataPoint::default();
    data_point
        .set_timestamp(timestamp)
        .set_cpu_usage(1.5)
        .set_memory_usage(2.5)
        .set_memory_used(3)
        .set_disk_usage(4.5)
        .set_disk_used(6)
        .set_network_rx(7)
        .set_network_tx(8)
        .set_load_average(0.5)
        .set_active_connections(2)
        .set_process_count(3);
    data_point
}

fn timestamps(buffer: &PerformanceRingBuffer) -> Vec<u64> {
    buffer
        .get_all_sorted()
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect()
}

#[test]
fn a_new_ring_buffer_is_empty_with_the_requested_capacity() {
    let buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(4);
    assert_eq!(buffer.get_capacity(), 4);
    assert_eq!(buffer.get_write_index(), 0);
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert!(buffer.get_all_sorted().is_empty());
    assert!(buffer.get_recent(1).is_empty());
    assert!(buffer.get_range(0, u64::MAX).is_empty());
}

#[test]
fn the_default_ring_buffer_holds_the_maximum_history_window() {
    let buffer: PerformanceRingBuffer = PerformanceRingBuffer::default();
    assert_eq!(buffer.get_capacity(), MAX_HISTORY_SECONDS);
    assert_eq!(buffer.get_capacity(), 60);
    assert!(buffer.is_empty());
}

#[test]
fn push_below_capacity_keeps_insertion_order() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(4);
    buffer.push(point(10));
    buffer.push(point(20));
    buffer.push(point(30));
    assert_eq!(buffer.len(), 3);
    assert!(!buffer.is_empty());
    assert_eq!(buffer.get_write_index(), 3);
    assert_eq!(buffer.get_buffer().len(), 3);
    assert_eq!(timestamps(&buffer), vec![10, 20, 30]);
}

#[test]
fn push_past_capacity_overwrites_the_oldest_entry() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(3);
    buffer.push(point(10));
    buffer.push(point(20));
    buffer.push(point(30));
    buffer.push(point(40));
    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer.get_buffer().len(), 3);
    assert_eq!(buffer.get_write_index(), 1);
    assert_eq!(timestamps(&buffer), vec![20, 30, 40]);
    buffer.push(point(50));
    assert_eq!(timestamps(&buffer), vec![30, 40, 50]);
}

#[test]
fn get_all_sorted_wraps_from_the_write_index_once_full() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(2);
    buffer.push(point(1));
    buffer.push(point(2));
    assert_eq!(timestamps(&buffer), vec![1, 2]);
    buffer.push(point(3));
    assert_eq!(timestamps(&buffer), vec![2, 3]);
    buffer.push(point(4));
    assert_eq!(timestamps(&buffer), vec![3, 4]);
    assert_eq!(buffer.get_write_index(), 0);
    assert_eq!(timestamps(&buffer), vec![3, 4]);
}

#[test]
fn get_range_filters_inclusively_on_both_ends() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(8);
    for timestamp in [100u64, 200, 300, 400, 500] {
        buffer.push(point(timestamp));
    }
    let inside: Vec<u64> = buffer
        .get_range(200, 400)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(inside, vec![200, 300, 400]);
    let single: Vec<u64> = buffer
        .get_range(300, 300)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(single, vec![300]);
    let above: Vec<u64> = buffer
        .get_range(6_000, 7_000)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert!(above.is_empty());
    let below: Vec<u64> = buffer
        .get_range(0, 50)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert!(below.is_empty());
    let all: Vec<u64> = buffer
        .get_range(0, u64::MAX)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(all, vec![100, 200, 300, 400, 500]);
}

#[test]
fn get_range_filters_a_wrapped_buffer_in_chronological_order() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(3);
    for timestamp in [1u64, 2, 3, 4, 5] {
        buffer.push(point(timestamp));
    }
    assert_eq!(timestamps(&buffer), vec![3, 4, 5]);
    let inside: Vec<u64> = buffer
        .get_range(3, 5)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(inside, vec![3, 4, 5]);
}

#[test]
fn get_recent_returns_the_tail_in_chronological_order() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(5);
    for timestamp in [10u64, 20, 30, 40, 50] {
        buffer.push(point(timestamp));
    }
    let two: Vec<u64> = buffer
        .get_recent(2)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(two, vec![40, 50]);
    let one: Vec<u64> = buffer
        .get_recent(1)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(one, vec![50]);
}

#[test]
fn get_recent_returns_everything_when_asked_for_more_than_capacity() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(2);
    buffer.push(point(7));
    buffer.push(point(8));
    buffer.push(point(9));
    let recent: Vec<u64> = buffer
        .get_recent(50)
        .iter()
        .map(|data_point: &PerformanceDataPoint| data_point.get_timestamp())
        .collect();
    assert_eq!(recent, vec![8, 9]);
    assert_eq!(recent.len(), buffer.len());
    let zero: Vec<PerformanceDataPoint> = buffer.get_recent(0);
    assert!(zero.is_empty());
}

#[test]
fn clear_resets_the_buffer_and_its_indices() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(3);
    for timestamp in [1u64, 2, 3, 4] {
        buffer.push(point(timestamp));
    }
    assert_eq!(buffer.len(), 3);
    buffer.clear();
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.get_write_index(), 0);
    assert_eq!(buffer.get_capacity(), 3);
    assert!(buffer.get_buffer().is_empty());
    assert!(buffer.get_all_sorted().is_empty());
    buffer.push(point(5));
    assert_eq!(timestamps(&buffer), vec![5]);
}

#[test]
fn a_zero_capacity_buffer_drops_every_push_and_stays_empty() {
    let mut buffer: PerformanceRingBuffer = PerformanceRingBuffer::new(0);
    assert_eq!(buffer.get_capacity(), 0);
    assert!(buffer.is_empty());
    buffer.push(point(1));
    buffer.push(point(2));
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.get_write_index(), 0);
    assert!(buffer.get_buffer().is_empty());
    assert!(buffer.get_all_sorted().is_empty());
    assert!(buffer.get_recent(3).is_empty());
    assert!(buffer.get_range(0, u64::MAX).is_empty());
    buffer.clear();
    assert!(buffer.is_empty());
    buffer.push(point(3));
    assert!(buffer.is_empty());
    assert!(buffer.get_all_sorted().is_empty());
}
