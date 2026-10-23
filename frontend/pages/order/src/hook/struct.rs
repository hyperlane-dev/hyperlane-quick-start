use super::*;

/// The reactive state the order page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct OrderState {
    /// The tab being displayed.
    #[get(type(copy))]
    pub tab: Signal<String>,
    /// The records of the list being displayed.
    #[get(type(copy))]
    pub records: Signal<Vec<RecordItem>>,
    /// The income the records of the filter hold.
    #[get(type(copy))]
    pub total_income: Signal<String>,
    /// The expense the records of the filter hold.
    #[get(type(copy))]
    pub total_expense: Signal<String>,
    /// The balance the records of the filter hold.
    #[get(type(copy))]
    pub balance: Signal<String>,
    /// The number of records the filter holds.
    #[get(type(copy))]
    pub total_count: Signal<i64>,
    /// The page of the records being displayed.
    #[get(type(copy))]
    pub page: Signal<i32>,
    /// The first day of the range the records are filtered by.
    #[get(type(copy))]
    pub start_date: Signal<String>,
    /// The last day of the range the records are filtered by.
    #[get(type(copy))]
    pub end_date: Signal<String>,
    /// The category the records are filtered by.
    #[get(type(copy))]
    pub filter_category: Signal<String>,
    /// The kind the records are filtered by.
    #[get(type(copy))]
    pub filter_type: Signal<String>,
    /// The statistics of the ledger.
    #[get(type(copy))]
    pub overview: Signal<Option<OverviewStatistics>>,
    /// The user that is signed in.
    #[get(type(copy))]
    pub current_user: Signal<Option<UserProfile>>,
    /// The users of the list being displayed.
    #[get(type(copy))]
    pub users: Signal<Vec<UserProfile>>,
    /// The number of users the server holds.
    #[get(type(copy))]
    pub users_total: Signal<i64>,
    /// Whether the server holds more users than the list shows.
    #[get(type(copy))]
    pub users_has_more: Signal<bool>,
    /// The identifiers the pages of the users start after.
    #[get(type(copy))]
    pub users_cursors: Signal<Vec<String>>,
    /// The page of the users being displayed.
    #[get(type(copy))]
    pub users_page: Signal<i32>,
    /// The keyword the users are filtered by.
    #[get(type(copy))]
    pub users_keyword: Signal<String>,
    /// Whether the keyword waits for the search to be sent.
    #[get(type(copy))]
    pub users_pending: Signal<bool>,
    /// The address of the profile.
    #[get(type(copy))]
    pub profile_email: Signal<String>,
    /// The phone number of the profile.
    #[get(type(copy))]
    pub profile_phone: Signal<String>,
    /// The current password the profile holds.
    #[get(type(copy))]
    pub old_password: Signal<String>,
    /// The new password the profile holds.
    #[get(type(copy))]
    pub new_password: Signal<String>,
    /// Whether the form that writes a record is open.
    #[get(type(copy))]
    pub create_open: Signal<bool>,
    /// The amount of the record that is written.
    #[get(type(copy))]
    pub record_amount: Signal<String>,
    /// The category of the record that is written.
    #[get(type(copy))]
    pub record_category: Signal<String>,
    /// The kind of the record that is written.
    #[get(type(copy))]
    pub record_type: Signal<String>,
    /// The description of the record that is written.
    #[get(type(copy))]
    pub record_description: Signal<String>,
    /// The day the bill of the record that is written was written on.
    #[get(type(copy))]
    pub record_bill_date: Signal<String>,
    /// The user the record that is written belongs to.
    #[get(type(copy))]
    pub record_target_user: Signal<String>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
}
