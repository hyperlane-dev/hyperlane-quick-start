/// The kind of a value that holds other values.
pub(crate) const KIND_OBJECT: &str = "object";

/// The kind of a value that holds a list.
pub(crate) const KIND_ARRAY: &str = "array";

/// The kind of a text.
pub(crate) const KIND_STRING: &str = "string";

/// The kind of a number.
pub(crate) const KIND_NUMBER: &str = "number";

/// The kind of a flag.
pub(crate) const KIND_BOOLEAN: &str = "boolean";

/// The kind of an empty value.
pub(crate) const KIND_NULL: &str = "null";

/// The kind of a value that holds a document.
pub(crate) const KIND_JSON: &str = "JSON";

/// The empty value of a field.
pub(crate) const EMPTY: &str = "";

/// The number of characters a preview holds.
pub(crate) const PREVIEW_LENGTH: usize = 60;

/// The suffix of a preview that was cut.
pub(crate) const PREVIEW_SUFFIX: &str = "...";

/// The number of rows of the field that holds the document.
pub(crate) const ROWS_INPUT: i32 = 10;

/// The number of rows of the field that holds the value of a node.
pub(crate) const ROWS_VALUE: i32 = 6;

/// The key of the value of a text that holds a number.
pub(crate) const KEY_VALUE: &str = "_value";

/// The failure reported when the document holds no text.
pub(crate) const ERROR_EMPTY_DOCUMENT: &str = "JSON content cannot be empty";

/// The failure reported when the document cannot be parsed.
pub(crate) const ERROR_PARSE: &str = "The document is not valid JSON";

/// The failure reported when the value of a node cannot be parsed.
pub(crate) const ERROR_VALUE: &str = "The value is not valid";

/// The failure reported when the document cannot be copied.
pub(crate) const ERROR_COPY: &str = "Copy failed";

/// The notice reported after the document has been copied.
pub(crate) const NOTICE_COPIED: &str = "Copied";
