use super::*;

/// The reactive state the QR code page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct QrcodeState {
    /// The entries of the list of the QR codes.
    #[get(type(copy))]
    pub entries: Signal<Vec<QrEntry>>,
    /// The error correction level of the codes.
    #[get(type(copy))]
    pub level: Signal<QrErrorLevel>,
    /// Whether the menu of the error correction level is open.
    #[get(type(copy))]
    pub level_open: Signal<bool>,
    /// The message of the last call.
    #[get(type(copy))]
    pub message: Signal<String>,
    /// Whether the message of the last call reports a failure.
    #[get(type(copy))]
    pub failed: Signal<bool>,
}
