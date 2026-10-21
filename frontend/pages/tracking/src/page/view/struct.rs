use super::*;

/// The props of the tracking page.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrackingPageProps {
    /// The state the page renders from.
    pub state: TrackingState,
}

/// The props of one metric tile.
#[derive(Clone, Debug, Default)]
pub struct StatTileProps {
    /// The name of the metric.
    pub label: &'static str,
    /// The value of the metric.
    pub value: String,
    /// The glyph of the metric.
    pub icon: &'static str,
}
