use super::*;

/// The props of the order page.
#[derive(Clone, Copy, Debug, Default)]
pub struct OrderPageProps {
    /// The state the page renders from.
    pub state: OrderState,
}

/// The props of the panel of one tab.
#[derive(Clone, Copy, Debug, Default)]
pub struct PanelProps {
    /// The state the page renders from.
    pub state: OrderState,
}

/// The props of one row of the list of records.
#[derive(Clone, Debug, Default)]
pub struct RecordRowProps {
    /// The record the row shows.
    pub record: RecordItem,
}

/// The props of one row of the list of users.
#[derive(Clone, Debug, Default)]
pub struct UserRowProps {
    /// The user the row shows.
    pub user: UserProfile,
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
    /// The note under the name of the metric.
    pub hint: String,
}

/// The props of one button of a filter.
#[derive(Clone, Copy, Debug, Default)]
pub struct FilterButtonProps {
    /// The state the page renders from.
    pub state: OrderState,
    /// The kind of the filter.
    pub kind: &'static str,
    /// The value of the filter.
    pub value: &'static str,
    /// The label of the button.
    pub label: &'static str,
}

/// The props of the form that writes a record.
#[derive(Clone, Copy, Debug, Default)]
pub struct RecordFormProps {
    /// The state the page renders from.
    pub state: OrderState,
}
