use super::*;

/// The tabs of the blog page.
pub(crate) const TAB_ITEMS: &[EuvTabItem] = &[
    EuvTabItem {
        key: TAB_EXPLORE,
        label: "Explore",
    },
    EuvTabItem {
        key: TAB_MY_POSTS,
        label: "My Posts",
    },
    EuvTabItem {
        key: TAB_FAVORITES,
        label: "Favorites",
    },
    EuvTabItem {
        key: TAB_WRITE,
        label: "Write",
    },
];

/// The icon of the post count.
pub(crate) const ICON_POSTS: &str = "📝";

/// The icon of the like count.
pub(crate) const ICON_LIKES: &str = "👍";

/// The icon of the comment count.
pub(crate) const ICON_COMMENTS: &str = "💬";

/// The label of the post count.
pub(crate) const LABEL_POSTS: &str = "Posts";

/// The label of the like count.
pub(crate) const LABEL_LIKES: &str = "Likes";

/// The label of the comment count.
pub(crate) const LABEL_COMMENTS: &str = "Comments";

/// The title of the post list.
pub(crate) const TITLE_POSTS: &str = "Posts";

/// The title of the editor.
pub(crate) const TITLE_WRITE: &str = "Write Post";

/// The title of the post detail.
pub(crate) const TITLE_DETAIL: &str = "Post";

/// The label of the search field.
pub(crate) const LABEL_SEARCH: &str = "Search";

/// The placeholder of the search field.
pub(crate) const PLACEHOLDER_SEARCH: &str = "Search posts";

/// The identifier of the search field.
pub(crate) const ID_SEARCH: &str = "blog-search";

/// The label of the title field.
pub(crate) const LABEL_TITLE: &str = "Title";

/// The placeholder of the title field.
pub(crate) const PLACEHOLDER_TITLE: &str = "The title of the post";

/// The identifier of the title field.
pub(crate) const ID_TITLE: &str = "blog-title";

/// The label of the summary field.
pub(crate) const LABEL_SUMMARY: &str = "Summary";

/// The placeholder of the summary field.
pub(crate) const PLACEHOLDER_SUMMARY: &str = "A short summary of the post";

/// The identifier of the summary field.
pub(crate) const ID_SUMMARY: &str = "blog-summary";

/// The label of the content field.
pub(crate) const LABEL_CONTENT: &str = "Content";

/// The placeholder of the content field.
pub(crate) const PLACEHOLDER_CONTENT: &str = "The body of the post, written in Markdown";

/// The number of rows of the content field.
pub(crate) const ROWS_CONTENT: i32 = 12;

/// The label of the publish switch.
pub(crate) const LABEL_PUBLISHED: &str = "Publish";

/// The identifier of the publish switch.
pub(crate) const ID_PUBLISHED: &str = "blog-published";

/// The label of the publish button.
pub(crate) const LABEL_PUBLISH: &str = "Publish";

/// The label of the comment field.
pub(crate) const LABEL_COMMENT: &str = "Comment";

/// The placeholder of the comment field.
pub(crate) const PLACEHOLDER_COMMENT: &str = "Write a comment";

/// The identifier of the comment field.
pub(crate) const ID_COMMENT: &str = "blog-comment";

/// The label of the send comment button.
pub(crate) const LABEL_SEND: &str = "Send";

/// The label of the previous page button.
pub(crate) const LABEL_PREVIOUS: &str = "Previous";

/// The label of the next page button.
pub(crate) const LABEL_NEXT: &str = "Next";

/// The label of the like button.
pub(crate) const LABEL_LIKE: &str = "Like";

/// The label of the favorite button.
pub(crate) const LABEL_FAVORITE: &str = "Favorite";

/// The text shown when the list holds no post.
pub(crate) const TEXT_EMPTY: &str = "No post matches this view yet.";

/// The title of the empty state.
pub(crate) const TITLE_EMPTY: &str = "Nothing here yet";

/// The label of the open button.
pub(crate) const LABEL_OPEN: &str = "Open";

/// The number of milliseconds in a minute.
pub(crate) const MILLIS_MINUTE: i64 = 60_000;

/// The number of minutes in an hour.
pub(crate) const MINUTES_HOUR: i64 = 60;

/// The number of hours in a day.
pub(crate) const HOURS_DAY: i64 = 24;

/// The text shown for a moment of the current minute.
pub(crate) const TEXT_JUST_NOW: &str = "just now";

/// The suffix of a relative time in minutes.
pub(crate) const TEXT_MINUTE_AGO: &str = " min ago";

/// The suffix of a relative time in hours.
pub(crate) const TEXT_HOUR_AGO: &str = " h ago";

/// The suffix of a relative time in days.
pub(crate) const TEXT_DAY_AGO: &str = " d ago";
