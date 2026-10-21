use super::*;

/// The tabs of the order page.
pub(crate) const TAB_ITEMS: &[EuvTabItem] = &[
    EuvTabItem {
        key: TAB_DASHBOARD,
        label: "Dashboard",
    },
    EuvTabItem {
        key: TAB_RECORDS,
        label: "Records",
    },
    EuvTabItem {
        key: TAB_PROFILE,
        label: "Profile",
    },
];

/// The tabs of the order page of an administrator.
pub(crate) const TAB_ITEMS_ADMIN: &[EuvTabItem] = &[
    EuvTabItem {
        key: TAB_DASHBOARD,
        label: "Dashboard",
    },
    EuvTabItem {
        key: TAB_RECORDS,
        label: "Records",
    },
    EuvTabItem {
        key: TAB_USERS,
        label: "Users",
    },
    EuvTabItem {
        key: TAB_PROFILE,
        label: "Profile",
    },
];

/// The glyph of the page header.
pub(crate) const ICON_ORDER: &str = "🧾";

/// The glyph of the number of the transactions of the day.
pub(crate) const ICON_TRANSACTIONS: &str = "🔁";

/// The glyph of the income of the day.
pub(crate) const ICON_INCOME: &str = "📈";

/// The glyph of the expense of the day.
pub(crate) const ICON_EXPENSE: &str = "📉";

/// The glyph of the number of the users that joined the day.
pub(crate) const ICON_USERS: &str = "👥";

/// The glyph of the average of the transactions.
pub(crate) const ICON_AVERAGE: &str = "🧮";

/// The glyph of the largest transaction.
pub(crate) const ICON_MAXIMUM: &str = "🏆";

/// The glyph of the balance of the ledger.
pub(crate) const ICON_BALANCE: &str = "💰";

/// The glyph of the number of the records.
pub(crate) const ICON_RECORDS: &str = "📄";

/// The title of the page header.
pub(crate) const TITLE_HEADER: &str = "Order Center";

/// The subtitle of the page header.
pub(crate) const TEXT_HEADER: &str = "Records, statistics and the profile of the visitor";

/// The title of the panel of the statistics.
pub(crate) const TITLE_DASHBOARD: &str = "Overview";

/// The title of the panel of the records.
pub(crate) const TITLE_RECORDS: &str = "Records";

/// The title of the panel of the users.
pub(crate) const TITLE_USERS: &str = "Users";

/// The title of the panel of the profile.
pub(crate) const TITLE_PROFILE: &str = "Profile";

/// The title of the form that writes a record.
pub(crate) const TITLE_RECORD: &str = "New Record";

/// The label of the number of the transactions of the day.
pub(crate) const LABEL_TRANSACTIONS: &str = "Today's Transactions";

/// The label of the income of the day.
pub(crate) const LABEL_INCOME: &str = "Today's Income";

/// The label of the expense of the day.
pub(crate) const LABEL_EXPENSE: &str = "Today's Expense";

/// The label of the number of the users that joined the day.
pub(crate) const LABEL_NEW_USERS: &str = "Today's New Users";

/// The label of the average income of one transaction.
pub(crate) const LABEL_AVG_INCOME: &str = "Avg Income";

/// The label of the average expense of one transaction.
pub(crate) const LABEL_AVG_EXPENSE: &str = "Avg Expense";

/// The label of the average amount of every transaction.
pub(crate) const LABEL_OVERALL_AVG: &str = "Overall Avg";

/// The label of the largest income of one transaction.
pub(crate) const LABEL_MAX_INCOME: &str = "Max Income";

/// The label of the largest expense of one transaction.
pub(crate) const LABEL_MAX_EXPENSE: &str = "Max Expense";

/// The label of the income of the records.
pub(crate) const LABEL_TOTAL_INCOME: &str = "Total Income";

/// The label of the expense of the records.
pub(crate) const LABEL_TOTAL_EXPENSE: &str = "Total Expense";

/// The label of the balance of the records.
pub(crate) const LABEL_BALANCE: &str = "Balance";

/// The label of the number of the records.
pub(crate) const LABEL_RECORD_COUNT: &str = "Records";

/// The label of the first day of the range.
pub(crate) const LABEL_START_DATE: &str = "Start Date";

/// The label of the last day of the range.
pub(crate) const LABEL_END_DATE: &str = "End Date";

/// The label of the filter of the kind of a record.
pub(crate) const LABEL_FILTER_TYPE: &str = "Type";

/// The label of the filter of the category of a record.
pub(crate) const LABEL_FILTER_CATEGORY: &str = "Category";

/// The label of the filter that holds every kind.
pub(crate) const LABEL_ALL_TYPES: &str = "All";

/// The label of the filter of the income records.
pub(crate) const LABEL_INCOME_FILTER: &str = "Income";

/// The label of the filter of the expense records.
pub(crate) const LABEL_EXPENSE_FILTER: &str = "Expense";

/// The label of the filter that holds every category.
pub(crate) const LABEL_ALL_CATEGORIES: &str = "All";

/// The label of the category of the food.
pub(crate) const LABEL_FOOD: &str = "Food";

/// The label of the category of the ride.
pub(crate) const LABEL_TRANSPORT: &str = "Transport";

/// The label of the category of the shopping.
pub(crate) const LABEL_SHOPPING: &str = "Shopping";

/// The label of the category of the fun.
pub(crate) const LABEL_ENTERTAINMENT: &str = "Entertainment";

/// The label of the category of the salary.
pub(crate) const LABEL_SALARY: &str = "Salary";

/// The label of the category of the investment.
pub(crate) const LABEL_INVESTMENT: &str = "Investment";

/// The label of the category of everything else.
pub(crate) const LABEL_OTHER: &str = "Other";

/// The label of the amount field.
pub(crate) const LABEL_AMOUNT: &str = "Amount";

/// The label of the description field.
pub(crate) const LABEL_DESCRIPTION: &str = "Description";

/// The label of the day of the bill.
pub(crate) const LABEL_BILL_DATE: &str = "Bill Date";

/// The label of the user of the record.
pub(crate) const LABEL_TARGET_USER: &str = "Target User";

/// The label of the address field of the profile.
pub(crate) const LABEL_EMAIL: &str = "Email";

/// The label of the phone field of the profile.
pub(crate) const LABEL_PHONE: &str = "Phone";

/// The label of the current password field.
pub(crate) const LABEL_OLD_PASSWORD: &str = "Old Password";

/// The label of the new password field.
pub(crate) const LABEL_NEW_PASSWORD: &str = "New Password";

/// The label of the search field of the users.
pub(crate) const LABEL_SEARCH: &str = "Search";

/// The placeholder of the search field of the users.
pub(crate) const PLACEHOLDER_SEARCH: &str = "Search users";

/// The identifier of the search field of the users.
pub(crate) const ID_SEARCH: &str = "order-user-search";

/// The placeholder of the amount field.
pub(crate) const PLACEHOLDER_AMOUNT: &str = "0.00";

/// The placeholder of the description field.
pub(crate) const PLACEHOLDER_DESCRIPTION: &str = "Enter description...";

/// The placeholder of the address field.
pub(crate) const PLACEHOLDER_EMAIL: &str = "Enter email";

/// The placeholder of the phone field.
pub(crate) const PLACEHOLDER_PHONE: &str = "Enter phone";

/// The placeholder of the current password field.
pub(crate) const PLACEHOLDER_OLD_PASSWORD: &str = "Enter old password";

/// The placeholder of the new password field.
pub(crate) const PLACEHOLDER_NEW_PASSWORD: &str = "Enter new password";

/// The identifier of the amount field.
pub(crate) const ID_AMOUNT: &str = "order-amount";

/// The identifier of the description field.
pub(crate) const ID_DESCRIPTION: &str = "order-description";

/// The identifier of the day of the bill.
pub(crate) const ID_BILL_DATE: &str = "order-bill-date";

/// The identifier of the user of the record.
pub(crate) const ID_TARGET_USER: &str = "order-target-user";

/// The identifier of the address field of the profile.
pub(crate) const ID_EMAIL: &str = "order-email";

/// The identifier of the phone field of the profile.
pub(crate) const ID_PHONE: &str = "order-phone";

/// The identifier of the current password field.
pub(crate) const ID_OLD_PASSWORD: &str = "order-old-password";

/// The identifier of the new password field.
pub(crate) const ID_NEW_PASSWORD: &str = "order-new-password";

/// The type of a date field.
pub(crate) const INPUT_DATE: &str = "date";

/// The type of a number field.
pub(crate) const INPUT_NUMBER: &str = "number";

/// The type of a password field.
pub(crate) const INPUT_PASSWORD: &str = "password";

/// The type of an address field.
pub(crate) const INPUT_EMAIL: &str = "email";

/// The type of a phone field.
pub(crate) const INPUT_PHONE: &str = "tel";

/// The label of the button that writes a record.
pub(crate) const LABEL_NEW_RECORD: &str = "New Record";

/// The label of the button that saves the profile.
pub(crate) const LABEL_SAVE_PROFILE: &str = "Save";

/// The label of the button that saves the password.
pub(crate) const LABEL_SAVE_PASSWORD: &str = "Change Password";

/// The label of the button that closes the form of a record.
pub(crate) const LABEL_CANCEL: &str = "Cancel";

/// The label of the button that creates the record.
pub(crate) const LABEL_CREATE: &str = "Create";

/// The label of the button that sends the search.
pub(crate) const LABEL_SEARCH_BUTTON: &str = "Search";

/// The label of the button that shows the previous page.
pub(crate) const LABEL_PREVIOUS: &str = "Previous";

/// The label of the button that shows the next page.
pub(crate) const LABEL_NEXT: &str = "Next";

/// The text shown when the list of records holds no record.
pub(crate) const TEXT_EMPTY_RECORDS: &str = "No record matches this view yet.";

/// The text shown when the list of users holds no user.
pub(crate) const TEXT_EMPTY_USERS: &str = "No user matches this search.";

/// The text shown when the statistics are not available.
pub(crate) const TEXT_EMPTY_STATISTICS: &str = "The statistics are not available.";

/// The title of the empty state.
pub(crate) const TITLE_EMPTY: &str = "Nothing here yet";

/// The text shown when the visitor is not an administrator.
pub(crate) const TEXT_NO_ADMIN: &str = "Only an administrator can see the users of the server.";
