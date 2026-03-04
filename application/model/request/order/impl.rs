use super::*;

impl Direction {
    #[instrument_trace]
    pub fn is_prev(&self) -> bool {
        matches!(self, Direction::Prev)
    }

    #[instrument_trace]
    pub fn is_next(&self) -> bool {
        matches!(self, Direction::Next)
    }
}
