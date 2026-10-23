use super::*;

/// The props of the blog page.
#[derive(Clone, Copy, Debug, Default)]
pub struct BlogPageProps {
    /// The state the page renders from.
    pub state: BlogState,
}

/// The props of the post list panel.
#[derive(Clone, Copy, Debug, Default)]
pub struct PostListPanelProps {
    /// The state the page renders from.
    pub state: BlogState,
}

/// The props of one row of the post list.
#[derive(Clone, Debug, Default)]
pub struct PostRowProps {
    /// The state the page renders from.
    pub state: BlogState,
    /// The post the row shows.
    pub post: BlogPost,
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

/// The props of the editor panel.
#[derive(Clone, Copy, Debug, Default)]
pub struct WritePanelProps {
    /// The state the page renders from.
    pub state: BlogState,
}

/// The props of the post detail.
#[derive(Clone, Copy, Debug, Default)]
pub struct PostDetailProps {
    /// The state the page renders from.
    pub state: BlogState,
}
