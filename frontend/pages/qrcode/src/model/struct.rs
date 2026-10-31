use super::*;

/// One entry of the list of the QR codes.
///
/// The entry carries the text the code encodes; the field of the failure is
/// never written into a file.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct QrEntry {
    /// The text the QR code encodes.
    #[serde(rename = "url")]
    pub text: String,
    /// The message of the last failure of the entry.
    #[serde(skip)]
    pub error: String,
}

/// The list of the QR codes as it is written into a file.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct QrExport {
    /// The entries of the list.
    pub url_list: Vec<QrEntry>,
}
