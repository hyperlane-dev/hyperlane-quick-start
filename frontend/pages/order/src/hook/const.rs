/// The endpoint of the records of the ledger.
pub(crate) const PATH_RECORD_LIST: &str = "/api/order/record/list";

/// The endpoint that creates a record.
pub(crate) const PATH_RECORD_CREATE: &str = "/api/order/record/create";

/// The endpoint of the statistics of the ledger.
pub(crate) const PATH_OVERVIEW: &str = "/api/order/overview/statistics";

/// The endpoint of the users of the server.
pub(crate) const PATH_USER_LIST: &str = "/api/auth/user/list";

/// The endpoint of the user that is signed in.
pub(crate) const PATH_USER_INFO: &str = "/api/auth/user/info";

/// The endpoint that changes the contact fields of a user.
pub(crate) const PATH_USER_UPDATE: &str = "/api/auth/user/update";

/// The endpoint that changes the password of a user.
pub(crate) const PATH_USER_CHANGE_PASSWORD: &str = "/api/auth/user/change_password";

/// The endpoint of the public key the credentials are encrypted with.
pub(crate) const PATH_PUBLIC_KEY: &str = "/api/auth/rsa/public-key";

/// The tab that shows the statistics of the ledger.
pub(crate) const TAB_DASHBOARD: &str = "dashboard";

/// The tab that shows the records of the ledger.
pub(crate) const TAB_RECORDS: &str = "records";

/// The tab that shows the users of the server.
pub(crate) const TAB_USERS: &str = "users";

/// The tab that shows the profile of the visitor.
pub(crate) const TAB_PROFILE: &str = "profile";

/// The empty value of a field.
pub(crate) const EMPTY: &str = "";

/// The number of records one page holds.
pub(crate) const PAGE_SIZE: u64 = 20;

/// The number of users one page holds.
pub(crate) const USER_PAGE_SIZE: u64 = 20;

/// The first page of a list.
pub(crate) const PAGE_FIRST: i32 = 1;

/// The number of milliseconds the search waits before it is sent.
pub(crate) const SEARCH_DELAY_MS: i32 = 400;

/// The kind of a record that adds to the balance.
pub(crate) const TYPE_INCOME: &str = "income";

/// The kind of a record that takes from the balance.
pub(crate) const TYPE_EXPENSE: &str = "expense";

/// The category of a record that holds the food of a day.
pub(crate) const CATEGORY_FOOD: &str = "food";

/// The category of a record that holds the ride of a day.
pub(crate) const CATEGORY_TRANSPORT: &str = "transport";

/// The category of a record that holds the shopping of a day.
pub(crate) const CATEGORY_SHOPPING: &str = "shopping";

/// The category of a record that holds the fun of a day.
pub(crate) const CATEGORY_ENTERTAINMENT: &str = "entertainment";

/// The category of a record that holds the salary of a day.
pub(crate) const CATEGORY_SALARY: &str = "salary";

/// The category of a record that holds the investment of a day.
pub(crate) const CATEGORY_INVESTMENT: &str = "investment";

/// The category of a record that holds everything else.
pub(crate) const CATEGORY_OTHER: &str = "other";

/// The role of an administrator.
pub(crate) const ROLE_ADMIN: &str = "admin";

/// The kind of the filter that holds the kind of the records.
pub(crate) const FILTER_KIND_TYPE: &str = "type";

/// The kind of the filter that holds the category of the records.
pub(crate) const FILTER_KIND_CATEGORY: &str = "category";

/// The kind of the filter that holds the kind of the record that is written.
pub(crate) const FILTER_KIND_RECORD_TYPE: &str = "record-type";

/// The kind of the filter that holds the category of the record that is written.
pub(crate) const FILTER_KIND_RECORD_CATEGORY: &str = "record-category";

/// The failure reported when the amount of a record is empty.
pub(crate) const ERROR_EMPTY_AMOUNT: &str = "The amount of a record is required";

/// The failure reported when the amount of a record is not a number.
pub(crate) const ERROR_INVALID_AMOUNT: &str = "The amount of a record is not a number";

/// The failure reported when a password field is empty.
pub(crate) const ERROR_EMPTY_PASSWORD: &str = "Please fill in both passwords";

/// The failure reported when the public key cannot be fetched.
pub(crate) const ERROR_KEY_FAILED: &str = "Encryption failed, please refresh and try again";
