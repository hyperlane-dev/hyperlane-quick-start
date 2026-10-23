use super::*;

/// The props of the user management page.
#[derive(Clone, Copy, Debug, Default)]
pub struct UserPageProps {
    /// The state the page renders from.
    pub state: UserState,
}

/// The props of the list of users.
#[derive(Clone, Copy, Debug, Default)]
pub struct UserListProps {
    /// The state the page renders from.
    pub state: UserState,
}

/// The props of one row of the list of users.
#[derive(Clone, Debug, Default)]
pub struct UserRowProps {
    /// The state the page renders from.
    pub state: UserState,
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
}

/// The props of the detail of a user.
#[derive(Clone, Copy, Debug, Default)]
pub struct UserDetailProps {
    /// The state the page renders from.
    pub state: UserState,
}
