use super::*;

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "next" => Ok(Direction::Next),
            "prev" => Ok(Direction::Prev),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

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
