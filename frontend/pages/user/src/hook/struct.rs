use super::*;

/// The action the visitor has to confirm.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PendingAction {
    /// The kind of the action.
    pub kind: String,
    /// The identifier of the user the action works on.
    pub id: String,
}

/// The reactive state the user management page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct UserState {
    /// The users of the list being displayed.
    #[get(type(copy))]
    pub users: Signal<Vec<UserProfile>>,
    /// The number of users the server holds.
    #[get(type(copy))]
    pub total: Signal<i64>,
    /// Whether the server holds more users than the list shows.
    #[get(type(copy))]
    pub has_more: Signal<bool>,
    /// The identifiers the pages of the list start after.
    #[get(type(copy))]
    pub cursors: Signal<Vec<String>>,
    /// The page being displayed.
    #[get(type(copy))]
    pub page: Signal<i32>,
    /// Whether the avatar of a user stays a square.
    #[get(type(copy))]
    pub square: Signal<bool>,
    /// The keyword the list is filtered by.
    #[get(type(copy))]
    pub keyword: Signal<String>,
    /// Whether the keyword waits for the search to be sent.
    #[get(type(copy))]
    pub pending: Signal<bool>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
    /// The user whose detail is open.
    #[get(type(copy))]
    pub detail: Signal<Option<UserProfile>>,
    /// The view the detail of the user shows.
    #[get(type(copy))]
    pub detail_mode: Signal<String>,
    /// The address the detail of the user shows.
    #[get(type(copy))]
    pub edit_email: Signal<String>,
    /// The phone number the detail of the user shows.
    #[get(type(copy))]
    pub edit_phone: Signal<String>,
    /// The current password the detail of the user holds.
    #[get(type(copy))]
    pub old_password: Signal<String>,
    /// The new password the detail of the user holds.
    #[get(type(copy))]
    pub new_password: Signal<String>,
    /// The action the visitor has to confirm.
    #[get(type(copy))]
    pub confirm: Signal<Option<PendingAction>>,
}
