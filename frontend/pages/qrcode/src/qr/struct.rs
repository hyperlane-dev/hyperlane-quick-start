use super::*;

/// The matrix of the modules of one QR code.
///
/// The matrix holds one boolean per module; a dark module is `true`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QrCode {
    /// The number of modules on one side of the code.
    pub size: usize,
    /// The version of the code.
    pub version: usize,
    /// The modules of the code, row by row.
    pub modules: Vec<bool>,
}

/// The builder of the matrix of one QR code.
#[derive(Clone, CustomDebug, Data, Default)]
pub(crate) struct QrBuilder {
    /// The number of modules on one side of the code.
    #[get(pub(crate), type(copy))]
    pub(crate) size: usize,
    /// The version of the code.
    #[get(pub(crate), type(copy))]
    pub(crate) version: usize,
    /// The error correction level of the code.
    #[get(pub(crate), type(copy))]
    pub(crate) level: QrErrorLevel,
    /// The modules of the code; a dark module is `true`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) modules: Vec<bool>,
    /// The modules that carry a function pattern and hold no data.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) function: Vec<bool>,
}
