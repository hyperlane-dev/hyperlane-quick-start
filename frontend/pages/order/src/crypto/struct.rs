use super::*;

/// A non negative integer stored as little endian 32 bit limbs.
///
/// The integer carries the modular exponentiation of the RSA encryption the
/// server expects: the browser has no big integer type of its own, so the
/// limbs hold the arithmetic of the encryption.
#[derive(Clone, CustomDebug, Data, Default, PartialEq, Eq)]
pub struct BigUint {
    /// The limbs of the number, the least significant one first.
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(crate))]
    pub(crate) limbs: Vec<u32>,
}

/// The public key the credentials are encrypted with.
///
/// The server hands the modulus and the exponent over as base64 texts, the
/// two numbers the modular exponentiation of the encryption works with.
#[derive(Clone, CustomDebug, Data, Default, PartialEq, Eq)]
pub struct PublicKey {
    /// The modulus of the key.
    #[get(pub(super))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) modulus: BigUint,
    /// The public exponent of the key.
    #[get(pub(super))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) exponent: BigUint,
    /// The number of bytes one encrypted block holds.
    #[get(pub(super), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) size: usize,
}
