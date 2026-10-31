use super::*;

/// The props of the QR code page.
#[derive(Clone, Copy, Debug, Default)]
pub struct QrcodePageProps {
    /// The state the page renders from.
    pub state: QrcodeState,
}

/// The props of one entry of the list of the codes.
#[derive(Clone, Debug, Default)]
pub struct QrcodeEntryProps {
    /// The state the page renders from.
    pub state: QrcodeState,
    /// The index of the entry in the list.
    pub index: usize,
    /// The text the entry encodes.
    pub text: String,
}
