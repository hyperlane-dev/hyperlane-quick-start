use super::*;

/// The props of the diff page.
#[derive(Clone, Copy, Debug, Default)]
pub struct DiffPageProps {
    /// The state the page renders from.
    pub state: DiffState,
}

/// The props of one metric of the result.
#[derive(Clone, Debug, Default)]
pub struct DiffStatProps {
    /// The name of the metric.
    pub label: &'static str,
    /// The value of the metric.
    pub value: String,
}

/// The props of one row of the split view.
#[derive(Clone, Debug, Default)]
pub struct DiffRowProps {
    /// The row the view shows.
    pub row: DiffRow,
    /// The side of the result the cell shows.
    pub side: DiffSide,
}

/// The props of the segments of one line.
#[derive(Clone, Debug, Default)]
pub struct DiffSegmentsProps {
    /// The segments the view shows.
    pub parts: Vec<DiffPart>,
}
