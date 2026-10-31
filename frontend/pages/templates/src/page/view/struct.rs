use super::*;

/// The props of the landing page.
#[derive(Clone, Copy, Debug, Default)]
pub struct TemplatesPageProps {
    /// The state the page renders from.
    pub state: TemplatesState,
}

/// The props of one metric tile of the landing page.
#[derive(Clone, Copy, Debug, Default)]
pub struct StatTileProps {
    /// The glyph of the metric.
    pub icon: &'static str,
    /// The value of the metric.
    pub value: &'static str,
    /// The name of the metric.
    pub label: &'static str,
}
