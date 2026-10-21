use super::*;

/// The variant of the primary buttons of the page.
pub(crate) const VARIANT_PRIMARY: EuvButtonVariant = EuvButtonVariant::Primary;

/// The variant of the secondary buttons of the page.
pub(crate) const VARIANT_OUTLINE: EuvButtonVariant = EuvButtonVariant::Outline;

/// The variant of the failure alerts of the page.
pub(crate) const VARIANT_ERROR: AlertVariant = AlertVariant::Error;

/// The icon of the page header.
pub(crate) const ICON_PAGE: &str = "🔗";

/// The title of the page header.
pub(crate) const TITLE_PAGE: &str = "Shortlink Generator";

/// The subtitle of the page header.
pub(crate) const SUBTITLE_PAGE: &str = "Transform your long URLs into short, shareable links";

/// The title of the form card.
pub(crate) const TITLE_FORM: &str = "Create a short link";

/// The label of the URL field.
pub(crate) const LABEL_URL: &str = "Enter your URL";

/// The placeholder of the URL field.
pub(crate) const PLACEHOLDER_URL: &str = "https://example.com/very/long/url/path";

/// The identifier of the URL field.
pub(crate) const ID_URL: &str = "shortlink-url";

/// The name of the URL field.
pub(crate) const NAME_URL: &str = "url";

/// The type of the URL field.
pub(crate) const TYPE_URL: &str = "text";

/// The hint below the URL field.
pub(crate) const HINT_URL: &str = "Make sure your URL starts with http:// or https://";

/// The label of the generate button.
pub(crate) const LABEL_GENERATE: &str = "Generate Shortlink";

/// The title of the result card.
pub(crate) const TITLE_RESULT: &str = "Your short link is ready";

/// The subtitle of the result card.
pub(crate) const SUBTITLE_RESULT: &str = "Click the link below to test it";

/// The label above the short link.
pub(crate) const LABEL_SHORTLINK: &str = "Shortlink";

/// The label of the copy button.
pub(crate) const LABEL_COPY: &str = "Copy";

/// The label of the copy button after the link was copied.
pub(crate) const LABEL_COPIED: &str = "Copied";

/// The label of the open button.
pub(crate) const LABEL_OPEN: &str = "Open link";

/// The label of the reset button.
pub(crate) const LABEL_RESET: &str = "Create another";

/// The title of the loading state.
pub(crate) const TITLE_LOADING: &str = "Generating your shortlink";

/// The title of the info card.
pub(crate) const TITLE_INFO: &str = "How it works";

/// The first line of the info card.
pub(crate) const TEXT_INFO_ONE: &str = "Enter your long URL in the field above";

/// The second line of the info card.
pub(crate) const TEXT_INFO_TWO: &str = "Generate the short link and copy it";

/// The third line of the info card.
pub(crate) const TEXT_INFO_THREE: &str = "Share the short link anywhere you want";
