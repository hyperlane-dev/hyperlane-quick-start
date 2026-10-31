use super::*;

/// The error correction level of a QR code.
///
/// The level decides how many codewords of the code can be restored when
/// the code is damaged, and how much text one code can carry.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum QrErrorLevel {
    /// About seven percent of the codewords can be restored.
    Low,
    /// About fifteen percent of the codewords can be restored.
    #[default]
    Medium,
    /// About twenty five percent of the codewords can be restored.
    Quartile,
    /// About thirty percent of the codewords can be restored.
    High,
}
