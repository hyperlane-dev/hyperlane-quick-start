use super::*;

/// The props of the monitor page.
#[derive(Clone, Copy, Debug, Default)]
pub struct MonitorPageProps {
    /// The state the page renders from.
    pub state: MonitorState,
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

/// The props of one entry of the system overview.
#[derive(Clone, Debug, Default)]
pub struct InfoEntryProps {
    /// The name of the entry.
    pub label: &'static str,
    /// The value of the entry.
    pub value: String,
}
